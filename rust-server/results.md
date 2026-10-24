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
