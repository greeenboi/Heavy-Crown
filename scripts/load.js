/* 
podman run --rm -i \
  -v "$(pwd)/scripts":/scripts:z \
  -e TARGET_URL=http://192.168.0.210:9000 \
  -e TEST_TYPE=baseline \
  grafana/k6 run /scripts/load.js

podman run --rm -i \
  -v "$(pwd)/scripts":/scripts:z \
  -e TARGET_URL=http://192.168.0.210:9000 \
  -e TEST_TYPE=load \
  grafana/k6 run /scripts/load.js

podman run --rm -i \
  -v "$(pwd)/scripts":/scripts:z \
  -e TARGET_URL=http://192.168.0.210:9000 \
  -e TEST_TYPE=stress \
  grafana/k6 run /scripts/load.js

podman run --rm -i \
  -v "$(pwd)/scripts":/scripts:z \
  -e TARGET_URL=http://192.168.0.210:9000 \
  -e TEST_TYPE=soak \
  grafana/k6 run /scripts/load.js
*/
import http from 'k6/http';
import { check, sleep } from 'k6';
import { Rate } from 'k6/metrics';

// Custom metrics to track specific business logic errors
const itemCreationFailureRate = new Rate('item_creation_failures');

// Select test type via environment variable (default to 'load')
// Options: 'baseline', 'load', 'stress', 'soak', 'break'
const TEST_TYPE = __ENV.TEST_TYPE || 'load';

const profiles = {
  // 1. Baseline Test: Light load to establish speed baseline
  baseline: {
    executor: 'ramping-vus',
    startVUs: 0,
    stages: [
      { duration: '15s', target: 5 },
      { duration: '30s', target: 5 },
      { duration: '15s', target: 0 },
    ],
  },
  // 2. Load Test: Expected peak traffic simulation
  load: {
    executor: 'ramping-vus',
    startVUs: 0,
    stages: [
      { duration: '15s', target: 20 },
      { duration: '45s', target: 20 },
      { duration: '15s', target: 0 },
    ],
  },
  // 3. Stress Test: Ramping up traffic until the API breaks down
  stress: {
    executor: 'ramping-vus',
    startVUs: 0,
    stages: [
      { duration: '15s', target: 20 },
      { duration: '20s', target: 100 },
      { duration: '25s', target: 250 },
      { duration: '20s', target: 500 },
      { duration: '20s', target: 750 },
      { duration: '15s', target: 0 },
    ],
  },
  // 4. Soak (Endurance) Test: Sustained average load to catch memory/storage leaks
  soak: {
    executor: 'ramping-vus',
    startVUs: 0,
    stages: [
      { duration: '30s', target: 20 },
      { duration: '5m', target: 20 }, // Extend this to hours for a real soak test
      { duration: '30s', target: 0 },
    ],
  },
  // 5. Break Test: Keep increasing VUs until the system must be stopped
  break: {
    executor: 'ramping-vus',
    startVUs: 0,
    stages: [
      { duration: '30s', target: 1000 },
      { duration: '45s', target: 10000 },
      { duration: '45s', target: 50000 },
    ],
  },
};

export const options = {
  scenarios: {
    api_load_test: profiles[TEST_TYPE] || profiles['load'],
  },
  thresholds: {
    // Client-side experience metric thresholds
    http_req_duration: ['p(95)<250', 'p(99)<500'], // p95 & p99 tail-latencies
    http_req_failed: ['rate<0.01'],              // Error rate under 1%
    http_req_connecting: ['p(95)<50'],           // Track TCP handshake bottlenecks
    item_creation_failures: ['rate<0.05'],       // Custom threshold for POST failures
  },
};

const BASE_URL = __ENV.TARGET_URL || 'http://192.168.0.210:9000';

export default function () {
  const headers = { 'Content-Type': 'application/json' };
  const randomSuffix = Math.random().toString(36).substring(2, 7);

  // 1. WRITE: Create a new item (POST /items/)
  const payload = JSON.stringify({
    title: `Item_${randomSuffix}`,
    description: `Load test description ${randomSuffix}`,
  });

  let res = http.post(`${BASE_URL}/items/`, payload, { headers });
  
  const postSuccess = check(res, {
    'POST /items/ status is 201': (r) => r.status === 201,
  });
  itemCreationFailureRate.add(!postSuccess);

  let createdId = null;
  if (res.status === 201) {
    try {
      const body = JSON.parse(res.body);
      createdId = body.id;
    } catch (e) {
      // Handle JSON parse errors gracefully
    }
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