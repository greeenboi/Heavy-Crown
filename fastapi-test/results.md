### Baseline test

```sh
TOTAL RESULTS 

    checks_total.......: 330     5.326768/s
    checks_succeeded...: 100.00% 330 out of 330
    checks_failed......: 0.00%   0 out of 330

    ✓ POST /items/ status is 201
    ✓ GET /items/ status is 200
    ✓ GET /items/{id} status is 200

    CUSTOM
    item_creation_failures.........: 0.00% 0 out of 110

    HTTP
    http_req_duration..............: avg=32.39ms min=3.76ms med=12.48ms max=137.28ms p(90)=74.08ms p(95)=93.83ms
      { expected_response:true }...: avg=32.39ms min=3.76ms med=12.48ms max=137.28ms p(90)=74.08ms p(95)=93.83ms
    http_req_failed................: 0.00% 0 out of 330
    http_reqs......................: 330   5.326768/s

    EXECUTION
    iteration_duration.............: avg=2.1s    min=2.06s  med=2.09s   max=2.16s    p(90)=2.13s   p(95)=2.14s
    iterations.....................: 110   1.775589/s
    vus............................: 1     min=0        max=5
    vus_max........................: 5     min=5        max=5

    NETWORK
    data_received..................: 17 MB 274 kB/s
    data_sent......................: 40 kB 645 B/s




running (1m02.0s), 0/5 VUs, 110 complete and 0 interrupted iterations
api_load_test ✓ [ 100% ] 0/5 VUs  1m0s
```

---

### Stress test

```sh
THRESHOLDS 

    http_req_connecting
    ✓ 'p(95)<50' p(95)=6.12ms

    http_req_duration
    ✗ 'p(95)<250' p(95)=59.99s
    ✗ 'p(99)<500' p(99)=1m0s

    http_req_failed
    ✗ 'rate<0.01' rate=36.03%

    item_creation_failures
    ✗ 'rate<0.05' rate=56.86%


  █ TOTAL RESULTS 

    checks_total.......: 1321   9.110202/s
    checks_succeeded...: 63.96% 845 out of 1321
    checks_failed......: 36.03% 476 out of 1321

    ✗ POST /items/ status is 201
      ↳  43% — ✓ 308 / ✗ 406
    ✗ GET /items/ status is 200
      ↳  88% — ✓ 280 / ✗ 35
    ✗ GET /items/{id} status is 200
      ↳  88% — ✓ 257 / ✗ 35

    CUSTOM
    item_creation_failures.........: 56.86% 406 out of 714

    HTTP
    http_req_duration..............: avg=21.57s   min=3.73ms med=541.5ms  max=1m0s   p(90)=59.99s p(95)=59.99s
      { expected_response:true }...: avg=925.51ms min=3.73ms med=134.39ms max=31.17s p(90)=1.08s  p(95)=1.9s  
    http_req_failed................: 36.03% 476 out of 1321
    http_reqs......................: 1321   9.110202/s

    EXECUTION
    iteration_duration.............: avg=13.79s   min=2.07s  med=2.73s    max=1m37s  p(90)=1m3s   p(95)=1m33s 
    iterations.....................: 299    2.062037/s
    vus............................: 3      min=1           max=750
    vus_max........................: 750    min=750         max=750

    NETWORK
    data_received..................: 48 MB  329 kB/s
    data_sent......................: 286 kB 2.0 kB/s




running (2m25.0s), 000/750 VUs, 299 complete and 725 interrupted iterations
api_load_test ✓ [ 100% ] 001/750 VUs  1m55s
time="2026-10-23T17:43:29Z" level=error msg="thresholds on metrics 'http_req_duration, http_req_failed, item_creation_failures' have been crossed"
```