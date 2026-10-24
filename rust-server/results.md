## v0.1.0
### Baseline tests

```sh
 █ THRESHOLDS 

    http_req_connecting
    ✓ 'p(95)<50' p(95)=5.18ms

    http_req_duration
    ✓ 'p(95)<250' p(95)=6.25ms
    ✓ 'p(99)<500' p(99)=9.27ms

    http_req_failed
    ✓ 'rate<0.01' rate=0.00%

    item_creation_failures
    ✓ 'rate<0.05' rate=0.00%


  █ TOTAL RESULTS 

    checks_total.......: 345     5.606104/s
    checks_succeeded...: 100.00% 345 out of 345
    checks_failed......: 0.00%   0 out of 345

    ✓ POST /items/ status is 201
    ✓ GET /items/ status is 200
    ✓ GET /items/{id} status is 200

    CUSTOM
    item_creation_failures.........: 0.00%  0 out of 115

    HTTP
    http_req_duration..............: avg=3.33ms min=1.16ms med=2.96ms max=10.02ms p(90)=5.19ms p(95)=6.25ms
      { expected_response:true }...: avg=3.33ms min=1.16ms med=2.96ms max=10.02ms p(90)=5.19ms p(95)=6.25ms
    http_req_failed................: 0.00%  0 out of 345
    http_reqs......................: 345    5.606104/s

    EXECUTION
    iteration_duration.............: avg=2.02s  min=2.01s  med=2.01s  max=2.03s   p(90)=2.02s  p(95)=2.02s 
    iterations.....................: 115    1.868701/s
    vus............................: 1      min=0        max=5
    vus_max........................: 5      min=5        max=5

    NETWORK
    data_received..................: 554 kB 9.0 kB/s
    data_sent......................: 41 kB  669 B/s




running (1m01.5s), 0/5 VUs, 115 complete and 0 interrupted iterations
api_load_test ✓ [ 100% ] 0/5 VUs  1m0s
```

---

### Stress Test

```sh
█ THRESHOLDS 

    http_req_connecting
    ✗ 'p(95)<50' p(95)=57.21ms

    http_req_duration
    ✗ 'p(95)<250' p(95)=10.27s
    ✗ 'p(99)<500' p(99)=12.27s

    http_req_failed
    ✓ 'rate<0.01' rate=0.00%

    item_creation_failures
    ✓ 'rate<0.05' rate=0.00%


  █ TOTAL RESULTS 

    checks_total.......: 12306   94.168259/s
    checks_succeeded...: 100.00% 12306 out of 12306
    checks_failed......: 0.00%   0 out of 12306

    ✓ POST /items/ status is 201
    ✓ GET /items/ status is 200
    ✓ GET /items/{id} status is 200

    CUSTOM
    item_creation_failures.........: 0.00%  0 out of 4102

    HTTP
    http_req_duration..............: avg=2.75s  min=1.03ms med=1.11s max=12.94s p(90)=8.07s  p(95)=10.27s
      { expected_response:true }...: avg=2.75s  min=1.03ms med=1.11s max=12.94s p(90)=8.07s  p(95)=10.27s
    http_req_failed................: 0.00%  0 out of 12306
    http_reqs......................: 12306  94.168259/s

    EXECUTION
    iteration_duration.............: avg=10.34s min=2.01s  med=5.33s max=33.88s p(90)=27.87s p(95)=28.96s
    iterations.....................: 4102   31.38942/s
    vus............................: 120    min=1          max=750
    vus_max........................: 750    min=750        max=750

    NETWORK
    data_received..................: 713 MB 5.5 MB/s
    data_sent......................: 1.5 MB 11 kB/s




running (2m10.7s), 000/750 VUs, 4102 complete and 0 interrupted iterations
api_load_test ✓ [ 100% ] 000/750 VUs  1m55s
time="2026-10-24T17:03:56Z" level=error msg="thresholds on metrics 'http_req_connecting, http_req_duration' have been crossed"
```

---
---

## v0.2.0
### Baseline Test

```sh

 LOAD TEST SUMMARY  [baseline]  target=http://192.168.0.10:9090

Duration            : 60.3s
Peak VUs            : 5
Total requests      : 300  (5.0 req/s)
Iterations          : 150
Failed requests     : 0.00%
Data sent/received  : 0.04 MB / 0.05 MB

Latency (http_req_duration)
  avg     1.8ms  min     1.0ms  med     1.5ms
  p90     2.7ms  p95     3.0ms  p99     4.0ms  max    13.3ms

  http_req_connecting    avg     2.0ms  p95     5.2ms  max     6.9ms
  http_req_waiting       avg     1.7ms  p95     2.9ms  max    13.1ms
  http_req_blocked       avg     2.0ms  p95     5.3ms  max     7.2ms

Per-endpoint breakdown
  endpoint          avg       p95       p99       max       err
  POST /items/          1.9ms     3.1ms     8.5ms    13.3ms   0.00%
  GET /items/           0.0ms     0.0ms     0.0ms     0.0ms   0.00%
  GET /items/{id}       1.7ms     2.8ms     4.0ms     4.0ms   0.00%

Checks
  OK   POST /items/ status is 201           150/150 passed
  OK   GET /items/{id} status is 200        150/150 passed

Thresholds
  PASS  http_req_failed: rate<0.01
  PASS  http_req_duration: p(95)<250
  PASS  http_req_duration: p(99)<500
  PASS  http_req_connecting: p(95)<50
  PASS  item_creation_failures: rate<0.05

running (1m00.3s), 0/5 VUs, 150 complete and 0 interrupted iterations
api_load_test ✓ [ 100% ] 0/5 VUs  1m0s
```

---

### Stress Test

```sh
==============================================================================
 LOAD TEST SUMMARY  [stress]  target=http://192.168.0.10:9090
==============================================================================
Duration            : 116.3s
Peak VUs            : 750
Total requests      : 42236  (363.2 req/s)
Iterations          : 21118
Failed requests     : 0.00%
Data sent/received  : 5.94 MB / 7.14 MB

Latency (http_req_duration)
  avg     2.0ms  min     0.8ms  med     1.5ms
  p90     2.7ms  p95     3.5ms  p99     8.2ms  max   553.5ms

  http_req_connecting    avg     2.2ms  p95     4.5ms  max    47.9ms
  http_req_waiting       avg     1.9ms  p95     3.4ms  max   553.5ms
  http_req_blocked       avg     2.3ms  p95     4.5ms  max    47.9ms

Per-endpoint breakdown
  endpoint          avg       p95       p99       max       err
  POST /items/          2.2ms     3.7ms    10.3ms   553.5ms   0.00%
  GET /items/           0.0ms     0.0ms     0.0ms     0.0ms   0.00%
  GET /items/{id}       1.8ms     3.2ms     6.6ms    46.7ms   0.00%

Checks
  OK   POST /items/ status is 201           21118/21118 passed
  OK   GET /items/{id} status is 200        21118/21118 passed

Thresholds
  PASS  http_req_duration: p(95)<250
  PASS  http_req_duration: p(99)<500
  PASS  item_creation_failures: rate<0.05
  PASS  http_req_failed: rate<0.01
  PASS  http_req_connecting: p(95)<50

running (1m56.3s), 000/750 VUs, 21118 complete and 0 interrupted iterations
api_load_test ✓ [ 100% ] 000/750 VUs  1m55s
```

---

### Break Test

 LOAD TEST SUMMARY  [break]  target=http://192.168.0.10:9090
==============================================================================
Duration            : 297.8s
Peak VUs            : 10000
Total requests      : 88170  (296.1 req/s)
Iterations          : 44983
Failed requests     : 4.74%
Data sent/received  : 11.92 MB / 14.22 MB

Latency (http_req_duration)
  avg     3.3ms  min     0.0ms  med     1.8ms
  p90     4.3ms  p95     6.5ms  p99    34.3ms  max   583.1ms

  http_req_connecting    avg     4.2ms  p95     7.1ms  max  1116.2ms
  http_req_waiting       avg     3.2ms  p95     6.5ms  max   582.9ms
  http_req_blocked       avg     4.2ms  p95     7.1ms  max  1116.2ms

Per-endpoint breakdown
  endpoint          avg       p95       p99       max       err
  POST /items/          3.4ms     7.2ms    37.1ms   583.1ms   6.47%
  GET /items/           0.0ms     0.0ms     0.0ms     0.0ms   0.00%
  GET /items/{id}       3.1ms     6.0ms    29.7ms   191.4ms   2.89%

Checks
  FAIL POST /items/ status is 201           42763/45719 passed
  FAIL GET /items/{id} status is 200        41225/42451 passed

==============================================================================
 CAPACITY / BREAKING POINT ANALYSIS
==============================================================================
SLO per step: p95<250ms, p99<500ms, errors<1%  (ramp 10s + hold 20s per step)

VUs    req/s    avg       p95       p99       max       err     verdict
------------------------------------------------------------------------------
10     11.1         4.7ms    18.4ms    81.3ms    94.1ms   0.00% PASS
25     29.9         2.0ms     2.9ms    15.0ms    71.1ms   0.00% PASS
50     60.5         3.9ms     4.7ms    89.0ms   116.1ms   0.00% PASS
100    123.1        3.3ms     3.6ms   108.8ms   133.0ms   0.00% PASS
150    188.3        2.0ms     3.6ms    11.3ms   101.6ms   0.00% PASS
200    255.1        2.8ms     3.9ms    58.6ms   167.9ms   0.00% PASS
300    380.1        2.1ms     4.4ms     8.3ms    64.6ms   0.00% PASS
400    508.5        2.9ms     5.6ms    30.1ms    83.0ms   0.00% PASS
500    638.7        3.0ms     5.9ms    18.1ms   583.1ms   0.00% PASS
750    743.5        4.7ms    13.1ms    89.0ms   245.2ms  18.75% FAIL
1000   (not reached - test aborted earlier)
1500   (not reached - test aborted earlier)
2000   (not reached - test aborted earlier)
3000   (not reached - test aborted earlier)
4000   (not reached - test aborted earlier)
5000   (not reached - test aborted earlier)
6000   (not reached - test aborted earlier)
7000   (not reached - test aborted earlier)
8000   (not reached - test aborted earlier)
9000   (not reached - test aborted earlier)
10000  (not reached - test aborted earlier)

------------------------------------------------------------------------------
 MAX CONSECUTIVE VUs WITHIN SLO : 500
 FIRST BREAKING STEP            : 750 VUs
 BREAK REASON                   : errors 18.7% >= 1%
 (supported range is therefore 500 < capacity < 750 VUs)
 PEAK THROUGHPUT WITHIN SLO     : 638.7 req/s
 PEAK THROUGHPUT OBSERVED       : 743.5 req/s
------------------------------------------------------------------------------
 Note: the load generator can itself be the bottleneck at high VU counts; watch
 k6 host CPU/network and the server host (CPU, fds, DB locks) when interpreting.

running (04m57.8s), 00000/10000 VUs, 44983 complete and 750 interrupted iterations
api_load_test ✗ [  47% ] 00489/10000 VUs  04m57.7s/10m40.0s
time="2026-10-24T18:34:53Z" level=error msg="test run was aborted because k6 received a 'interrupt' signal"