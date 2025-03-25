import http from 'k6/http';

export const options = {
  noConnectionReuse: true,
};

export default function () {
  let params = {
    // Reasonable SLA for heavy processing - anything longer than this is failed request.
    timeout: "5000ms",
  };

  http.post('http://10.0.0.8:1234/check', '7854627897289748571', params);
}
