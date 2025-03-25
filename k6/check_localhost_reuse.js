import http from 'k6/http';

export const options = {
  noConnectionReuse: false,
};

export default function () {
  let params = {
    // Reasonable SLA for heavy processing - anything longer than this is failed request.
    timeout: "5000ms",
  };

  let res = http.post('http://127.0.0.1:1234/check', 'this is not a number', params);

  if (res.body !== 'false') {
    throw new Error(`Unexpected response body: ${res.body}`);
  }
}
