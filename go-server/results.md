## v0.1.0
### Baseline Test

```sh
==============================================================================
 LOAD TEST SUMMARY  [baseline]  target=http://192.168.0.10:9090
==============================================================================
Duration            : 61.3s
Peak VUs            : 5
Total requests      : 230  (3.7 req/s)
Iterations          : 115
Failed requests     : 0.00%
Data sent/received  : 0.03 MB / 0.54 MB

Latency (http_req_duration)
  avg     5.3ms  min     1.3ms  med     4.1ms
  p90     9.0ms  p95    11.8ms  p99    19.7ms  max    27.6ms

  http_req_connecting    avg     0.1ms  p95     0.0ms  max     7.8ms
  http_req_waiting       avg     5.0ms  p95    11.7ms  max    27.5ms
  http_req_blocked       avg     0.1ms  p95     0.0ms  max     7.9ms

Per-endpoint breakdown
  endpoint          avg       p95       p99       max       err
  POST /items/          0.0ms     0.0ms     0.0ms     0.0ms   0.00%
  GET /items/           0.0ms     0.0ms     0.0ms     0.0ms   0.00%
  GET /items/{id}       0.0ms     0.0ms     0.0ms     0.0ms   0.00%

Checks
  FAIL POST /items status is 201            0/115 passed
  OK   GET /items/ status is 200            115/115 passed

Thresholds
  PASS  http_req_duration: p(95)<250
  PASS  http_req_duration: p(99)<500
  PASS  http_req_connecting: p(95)<50
  PASS  http_req_failed: rate<0.01
  FAIL  item_creation_failures: rate<0.05

running (1m01.3s), 0/5 VUs, 115 complete and 0 interrupted iterations
api_load_test ✓ [ 100% ] 0/5 VUs  1m0s
time="2026-11-11T12:03:02Z" level=error msg="thresholds on metrics 'item_creation_failures' have been crossed"
```

---

### Load Test

```sh
==============================================================================
 LOAD TEST SUMMARY  [stress]  target=http://192.168.0.10:9090
==============================================================================
Duration            : 138.0s
Peak VUs            : 750
Total requests      : 9083  (65.8 req/s)
Iterations          : 4541
Failed requests     : 5.48%
Data sent/received  : 1.25 MB / 703.01 MB

Latency (http_req_duration)
  avg  3139.4ms  min     1.5ms  med   400.1ms
  p90  8747.0ms  p95 14705.2ms  p99 27095.6ms  max 60001.2ms

  http_req_connecting    avg    37.6ms  p95   232.9ms  max  7418.9ms
  http_req_waiting       avg   967.7ms  p95  5262.4ms  max 57042.2ms
  http_req_blocked       avg    37.6ms  p95   232.9ms  max  7418.9ms

Per-endpoint breakdown
  endpoint          avg       p95       p99       max       err
  POST /items/          0.0ms     0.0ms     0.0ms     0.0ms   0.00%
  GET /items/           0.0ms     0.0ms     0.0ms     0.0ms   0.00%
  GET /items/{id}       0.0ms     0.0ms     0.0ms     0.0ms   0.00%

Checks
  FAIL POST /items status is 201            0/4542 passed
  FAIL GET /items/ status is 200            4393/4541 passed

Thresholds
  FAIL  item_creation_failures: rate<0.05
  FAIL  http_req_connecting: p(95)<50
  FAIL  http_req_failed: rate<0.01
  FAIL  http_req_duration: p(95)<250
  FAIL  http_req_duration: p(99)<500

running (2m18.0s), 000/750 VUs, 4541 complete and 1 interrupted iterations
api_load_test ✓ [ 100% ] 000/750 VUs  1m55s
time="2026-11-11T12:06:26Z" level=error msg="thresholds on metrics 'http_req_connecting, http_req_duration, http_req_failed, item_creation_failures' have been crossed"
```

---