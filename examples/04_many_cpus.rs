use axum::{Router, http::StatusCode, response::IntoResponse, routing::post};
use forbidden_text_check::is_forbidden_text_static;
use hyper_util::rt::TokioIo;
use hyper_util::service::TowerToHyperService;
use many_cpus::ProcessorSet;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc::{Receiver, Sender, channel};
use tower::Service;

// We start main() on a single-threaded Tokio runtime because all main() does
// is listen for connections and pass them on to the real worker pool.
#[tokio::main(flavor = "current_thread")]
async fn main() {
    increase_ulimit();

    let addr = SocketAddr::from(([0, 0, 0, 0], 1234));
    println!("Server starting on http://{}", addr);

    let processors = ProcessorSet::default();
    let num_workers = processors.len();
    println!("Starting {} worker threads", num_workers);

    let mut work_txs = Vec::with_capacity(num_workers);

    for _ in 0..num_workers {
        const WORKER_QUEUE_SIZE: usize = 4;

        let (tx, rx) = channel(WORKER_QUEUE_SIZE);
        work_txs.push(tx);

        // In each loop iteration, we spawn a new worker thread that the OS is allowed to assign
        // to any of the processors in the set to balance load among them. This is almost entirely
        // equivalent to `thread::spawn()`, except by using `ProcessorSet::default()` we ensure that
        // all processors are made available for these threads on Windows, even on many-processor
        // systems with multiple processor groups where threads can otherwise be limited to only
        // one processor group.
        processors.spawn_thread(move |_| worker_entrypoint(rx));
    }

    listener_entrypoint(addr, work_txs).await;
}

async fn listener_entrypoint(addr: SocketAddr, work_txs: Vec<Sender<TcpStream>>) {
    let listener = TcpListener::bind(addr).await.unwrap();

    let mut next_worker = 0;

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        work_txs[next_worker].send(stream).await.unwrap();
        next_worker = (next_worker + 1) % work_txs.len();
    }
}

fn worker_entrypoint(mut rx: Receiver<TcpStream>) {
    // Every worker thread gets its own Tokio runtime, which means all the tasks of this
    // thread remain in this thread - there is no implicit travel between threads and
    // multithreaded activity only occurs when explicitly intended by the service author.
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(async move {
        // We build a new Axum app on every worker, ensuring that workers are independent.
        let app = Router::new().route("/check", post(check));
        let service_factory = app.into_make_service_with_connect_info::<SocketAddr>();

        while let Some(stream) = rx.recv().await {
            let peer_addr = stream.peer_addr().unwrap();

            // For each connection, we spawn a new task to handle it.
            tokio::spawn({
                let mut service_factory = service_factory.clone();

                async move {
                    let service = service_factory.call(peer_addr).await.unwrap();
                    let hyper_service = TowerToHyperService::new(service);

                    let http = hyper::server::conn::http1::Builder::new();

                    // We do not care if the request handling succeeds or fails, so ignore result.
                    _ = http
                        .serve_connection(TokioIo::new(stream), hyper_service)
                        .await;
                }
            });
        }
    });
}

async fn check(body: String) -> impl IntoResponse {
    if is_forbidden_text_static(&body) {
        (StatusCode::OK, "true")
    } else {
        (StatusCode::OK, "false")
    }
}

/// On Linux, we can easily run out of file descriptors that we need for our sockets, so
/// we need to increase the ulimit. This is a no-op on Windows as Windows has no such limitations.
fn increase_ulimit() {
    #[cfg(unix)]
    nix::sys::resource::setrlimit(nix::sys::resource::Resource::RLIMIT_NOFILE, 8192, 8192).unwrap();
}
