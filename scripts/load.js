import http from 'k6/http';
import { check, sleep } from 'k6';
import { randomString } from 'k6/javascript';

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

// Target URL defaults to localhost or can be overridden via environment variable
const BASE_URL = __ENV.TARGET_URL || 'http://192.168.1.50:8000';

export default function () {
  const headers = { 'Content-Type': 'application/json' };

  // 1. WRITE: Create a new item (POST /items/)
  const payload = JSON.stringify({
    title: `Item_${randomString(5)}`,
    description: `Load test description ${randomString(10)}`,
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