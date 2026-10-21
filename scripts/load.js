/* podman run --rm -i \
  -v "$(pwd)/scripts":/scripts:z \
  -e TARGET_URL=http://192.168.0.210:9000 \
  grafana/k6 run /scripts/load.js
*/

import http from 'k6/http';
import { check, sleep } from 'k6';

export const options = {
  stages: [
    { duration: '15s', target: 20 },  // Ramp up to 20 virtual users
    { duration: '30s', target: 20 },  // Hold steady at 20 users
    { duration: '15s', target: 50 },  // Ramp up to 50 users
    { duration: '30s', target: 50 },  // Hold steady at 50 users
    { duration: '15s', target: 0 },   // Scale down to 0
  ],
  thresholds: {
    http_req_duration: ['p(95)<250', 'p(99)<500'], // Fail if p95 > 250ms
    http_req_failed: ['rate<0.01'],              // Error rate must be under 1%
  },
};

const BASE_URL = __ENV.TARGET_URL || 'http://192.168.0.210:9000';

export default function () {
  const headers = { 'Content-Type': 'application/json' };

  // Use native JS to generate a random string instead of importing k6/javascript
  const randomSuffix = Math.random().toString(36).substring(2, 7);

  // 1. WRITE: Create a new item (POST /items/)
  const payload = JSON.stringify({
    title: `Item_${randomSuffix}`,
    description: `Load test description ${randomSuffix}`,
  });

  let res = http.post(`${BASE_URL}/items/`, payload, { headers });
  check(res, {
    'POST /items/ status is 201': (r) => r.status === 201,
  });

  let createdId = null;
  if (res.status === 201) {
    try {
      const body = JSON.parse(res.body);
      createdId = body.id;
    } catch (e) {}
  }

  sleep(0.5);

  // 2. READ: Get all items (GET /items/)
  res = http.get(`${BASE_URL}/items/`);
  check(res, {
    'GET /items/ status is 200': (r) => r.status === 200,
  });

  sleep(0.5);

  // 3. READ: Get single item by ID (GET /items/{id})
  if (createdId) {
    res = http.get(`${BASE_URL}/items/${createdId}`);
    check(res, {
      'GET /items/{id} status is 200': (r) => r.status === 200,
    });
  }

  sleep(1);
}