import http from 'k6/http';
import { randomIntBetween } from 'https://jslib.k6.io/k6-utils/1.6.0/index.js';

export default function () {
  let params = {
    // Reasonable SLA for heavy processing - anything longer than this is failed request.
    timeout: "10000ms",
  };

  let titles = [];

  for (let i = 0; i < 10000; i++) {
    // Max here should ideally match the max item count in the code. Otherwise, we are looking
    // up keys that do not exist, which might make the job too easy for the service.
    titles.push(randomIntBetween(0, 20 * 1000 * 1000).toString());
  }

  let payload = titles.join(',');

  http.post('http://10.0.0.7:1234/check', payload, params);
}
