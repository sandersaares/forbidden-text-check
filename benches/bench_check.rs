use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use forbidden_text_check::*;
use region_cached::RegionCachedExt;

criterion_group!(benches, entrypoint);
criterion_main!(benches);

const HAYSTACK: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Vestibulum tincidunt ex facilisis, egestas arcu non, tempus nisl. Ut rutrum interdum erat eget pulvinar. Quisque dui turpis, sagittis non massa at, mattis imperdiet tellus. Nunc posuere tortor quis volutpat aliquam. Mauris tempor vel risus tincidunt lacinia. Phasellus eu facilisis sapien, eu vestibulum ipsum. Donec pellentesque elementum pharetra. Sed gravida nec nunc sed viverra. Duis tempus eros placerat, vehicula nunc eu, scelerisque massa. Praesent rhoncus, nisl vitae mollis fringilla, mi eros malesuada leo, quis dictum ex felis non tortor. Phasellus suscipit est nisl, vel mollis nunc commodo id. Interdum et malesuada fames ac ante ipsum primis in faucibus. Morbi varius, nibh vitae faucibus imperdiet, urna mauris pellentesque nibh, fermentum tincidunt turpis nibh vitae ligula. Vestibulum ut enim euismod, elementum nunc eget, cursus ante. Etiam efficitur velit egestas sem scelerisque, eget interdum nunc aliquet. Aenean ultricies, augue id blandit vestibulum, ante dui varius lorem, eu bibendum tortor orci sit amet turpis. Donec vel lobortis nisi. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec sit amet odio risus. Pellentesque id magna quis ex tincidunt tristique non id odio. Mauris ornare justo est, non convallis nisi efficitur ut. Vestibulum eu maximus quam, ullamcorper hendrerit lectus. Sed finibus dolor eu neque imperdiet cursus. In volutpat vitae magna nec consectetur. Proin dapibus bibendum odio, vel vestibulum velit vestibulum ac. Proin sapien tortor, volutpat ut malesuada at, hendrerit dapibus eros. Mauris tempus lorem rhoncus erat posuere tristique eget sed tellus. Maecenas vitae sollicitudin odio. Nunc vitae tristique erat. Etiam ligula sem, varius ac nisi vulputate, blandit accumsan mi. Fusce euismod feugiat commodo. Nulla eget scelerisque ex. Sed lobortis vehicula aliquam. Donec auctor dolor metus, non placerat nisi gravida et. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. Phasellus vitae euismod eros. Integer auctor ipsum nunc, sit amet faucibus felis pellentesque et. Donec mattis tortor vitae fermentum pulvinar. Praesent vel velit id enim sagittis laoreet. Integer egestas sit amet dolor quis lobortis. Integer tincidunt leo a magna convallis pretium. Vestibulum lacinia quam a arcu consectetur, sed imperdiet ante interdum. Praesent maximus ante ut elit euismod consequat. Phasellus mattis leo vitae ipsum laoreet venenatis. Ut eleifend non neque ut aliquam. Nulla facilisi. Mauris vel ex aliquam, posuere quam ac, cursus leo. Nulla malesuada erat libero, quis dapibus justo fringilla varius. Aenean eget mattis arcu. Maecenas aliquam ex ipsum, eu venenatis ligula tempus ut. Mauris magna tellus, euismod et elit in, fermentum sodales purus. Phasellus tempus tristique massa, vitae ultrices tortor accumsan ut. Maecenas molestie felis dui, vitae finibus urna malesuada et. Cras eget est mattis, condimentum erat sed, dignissim libero. Praesent luctus ante in semper aliquet. Maecenas gravida tellus et nibh viverra, a congue turpis viverra. Pellentesque arcu libero, maximus vitae vehicula sed, facilisis nec dolor. Etiam semper volutpat urna id faucibus. Nam tincidunt orci quis mi condimentum, vitae consectetur mauris facilisis. Fusce porta rhoncus nisl, non imperdiet nisi sollicitudin sit amet. Integer ullamcorper urna lacus. Curabitur ut ante ac dolor cursus feugiat. Ut vulputate molestie viverra. Duis aliquet euismod libero, id tempor magna efficitur a. Nunc lorem felis, mollis non est eu, venenatis elementum enim. Etiam eu orci porttitor, dignissim ex vel, rutrum ipsum. Nunc finibus sed metus et aliquam. Pellentesque in elit lacinia, lobortis nunc id, lacinia dolor";

fn entrypoint(c: &mut Criterion) {
    let mut g = c.benchmark_group("number_crunching");

    // Touch each of the data sets to ensure they are loaded into memory.
    black_box(FORBIDDEN_TEXTS.len());
    black_box(FORBIDDEN_TEXTS_REGION_CACHED.with_cached(|x| x.len()));

    // The data set is huge, so let's not be greedy.
    g.sample_size(10);

    g.bench_function("static", |b| {
        b.iter(|| is_forbidden_text_static(HAYSTACK));
    });

    g.bench_function("region_cached", |b| {
        b.iter(|| is_forbidden_text_region_cached(HAYSTACK));
    });

    g.finish();
}
