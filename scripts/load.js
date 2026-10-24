/*
podman run --rm -i \
  -v "$(pwd)/scripts":/scripts:ro,z \
  -e TARGET_URL=http://192.168.0.10:9090 \
  -e TEST_TYPE=break \
  grafana/k6 run /scripts/load.js

# TEST_TYPE options: baseline | load | stress | break
# `break` runs a staircase (hold each VU level) and reports the maximum number
# of concurrent VUs the API sustained before breaching the SLOs.
# Optional: -e STEP_VUS=10,50,100,200  -e STEP_HOLD=20 -e STEP_RAMP=10
# The full report is printed to the terminal only (no files are written).
*/
import http from 'k6/http';
import exec from 'k6/execution';
import { check, sleep } from 'k6';
import { Rate } from 'k6/metrics';

const itemCreationFailureRate = new Rate('item_creation_failures');

const TEST_TYPE = __ENV.TEST_TYPE || 'load';
const BASE_URL = __ENV.TARGET_URL || 'http://192.168.0.210:9000';

// SLOs used both for thresholds and for judging each staircase step
const SLO = { p95: 250, p99: 500, errorRate: 0.01 };

// Staircase config for the break/capacity test
const STEP_VUS = (__ENV.STEP_VUS || '10,25,50,100,150,200,300,400,500,750,1000,1500,2000')
  .split(',')
  .map((n) => parseInt(n, 10));
const STEP_RAMP = parseInt(__ENV.STEP_RAMP || '10', 10); // seconds
const STEP_HOLD = parseInt(__ENV.STEP_HOLD || '20', 10); // seconds
const STEP_LEN_MS = (STEP_RAMP + STEP_HOLD) * 1000;

const ENDPOINTS = ['POST /items/', 'GET /items/', 'GET /items/{id}'];

const staircase = [];
for (const target of STEP_VUS) {
  staircase.push({ duration: `${STEP_RAMP}s`, target });
  staircase.push({ duration: `${STEP_HOLD}s`, target });
}
staircase.push({ duration: '10s', target: 0 });

const profiles = {
  baseline: {
    executor: 'ramping-vus',
    startVUs: 0,
    stages: [
      { duration: '15s', target: 5 },
      { duration: '30s', target: 5 },
      { duration: '15s', target: 0 },
    ],
  },
  load: {
    executor: 'ramping-vus',
    startVUs: 0,
    stages: [
      { duration: '15s', target: 20 },
      { duration: '45s', target: 20 },
      { duration: '15s', target: 0 },
    ],
  },
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
  // Staircase: hold each VU level so latency/errors can be attributed to it
  break: {
    executor: 'ramping-vus',
    startVUs: 0,
    gracefulRampDown: '5s',
    gracefulStop: '5s',
    stages: staircase,
  },
};

const IS_BREAK = TEST_TYPE === 'break';

function buildThresholds() {
  const t = {};
  if (IS_BREAK) {
    // Per-step sub-metrics: these drive the capacity verdict (no abort so we see all steps)
    for (const v of STEP_VUS) {
      t[`http_req_duration{step:${v}}`] = [`p(95)<${SLO.p95}`, `p(99)<${SLO.p99}`];
      t[`http_req_failed{step:${v}}`] = [`rate<${SLO.errorRate}`];
      t[`http_reqs{step:${v}}`] = ['count>=0'];
    }
    // Safety valve: stop once the API is clearly dead (>50% errors) so the run ends
    t['http_req_failed'] = [{ threshold: 'rate<0.5', abortOnFail: true, delayAbortEval: '20s' }];
  } else {
    t['http_req_duration'] = [`p(95)<${SLO.p95}`, `p(99)<${SLO.p99}`];
    t['http_req_failed'] = [`rate<${SLO.errorRate}`];
    t['http_req_connecting'] = ['p(95)<50'];
    t['item_creation_failures'] = ['rate<0.05'];
  }
  for (const e of ENDPOINTS) {
    t[`http_req_duration{endpoint:${e}}`] = ['max>=0'];
    t[`http_req_failed{endpoint:${e}}`] = ['rate>=0'];
  }
  return t;
}

export const options = {
  scenarios: { api_load_test: profiles[TEST_TYPE] || profiles['load'] },
  thresholds: buildThresholds(),
  summaryTrendStats: ['avg', 'min', 'med', 'p(90)', 'p(95)', 'p(99)', 'max'],
};

function currentStep() {
  if (!IS_BREAK) return 'n/a';
  const idx = Math.min(
    Math.floor(exec.instance.currentTestRunDuration / STEP_LEN_MS),
    STEP_VUS.length - 1,
  );
  return String(STEP_VUS[idx]);
}

export default function () {
  const headers = { 'Content-Type': 'application/json' };
  const step = currentStep();
  const tags = (endpoint) => ({ step, endpoint });
  const suffix = Math.random().toString(36).substring(2, 7);

  // 1. WRITE
  let res = http.post(
    `${BASE_URL}/items/`,
    JSON.stringify({ title: `Item_${suffix}`, description: `Load test description ${suffix}` }),
    { headers, tags: { ...tags('POST /items/'), name: 'POST /items/' } },
  );
  const postOk = check(res, { 'POST /items/ status is 201': (r) => r.status === 201 });
  itemCreationFailureRate.add(!postOk);

  let createdId = null;
  if (res.status === 201) {
    try {
      createdId = JSON.parse(res.body).id;
    } catch (e) {
      // ignore malformed body
    }
  }
  sleep(0.5);

  // // 2. READ list
  // res = http.get(`${BASE_URL}/items/`, { tags: { ...tags('GET /items/'), name: 'GET /items/' } });
  // check(res, { 'GET /items/ status is 200': (r) => r.status === 200 });
  // sleep(0.5);

  // 3. READ single
  if (createdId) {
    res = http.get(`${BASE_URL}/items/${createdId}`, {
      tags: { ...tags('GET /items/{id}'), name: 'GET /items/{id}' },
    });
    check(res, { 'GET /items/{id} status is 200': (r) => r.status === 200 });
  }
  sleep(1);
}

// ---------- Reporting ----------

const ms = (v) => (v === undefined || v === null ? '   n/a' : `${v.toFixed(1)}ms`.padStart(9));
const pct = (v) => (v === undefined || v === null ? '  n/a' : `${(v * 100).toFixed(2)}%`.padStart(7));
const pad = (s, n) => String(s).padEnd(n);
const line = (c = '=') => c.repeat(78);

function vals(data, name) {
  const m = data.metrics[name];
  return m ? m.values : undefined;
}

function stepResults(data) {
  return STEP_VUS.map((vus) => {
    const d = vals(data, `http_req_duration{step:${vus}}`);
    const f = vals(data, `http_req_failed{step:${vus}}`);
    const n = vals(data, `http_reqs{step:${vus}}`);
    const reached = !!(d && n && n.count > 0);
    const p95 = d && d['p(95)'];
    const p99 = d && d['p(99)'];
    const err = f ? f.rate : undefined;
    const reasons = [];
    if (reached) {
      if (p95 >= SLO.p95) reasons.push(`p95 ${p95.toFixed(0)}ms >= ${SLO.p95}ms`);
      if (p99 >= SLO.p99) reasons.push(`p99 ${p99.toFixed(0)}ms >= ${SLO.p99}ms`);
      if (err >= SLO.errorRate) reasons.push(`errors ${(err * 100).toFixed(1)}% >= ${SLO.errorRate * 100}%`);
    }
    return {
      vus, reached, p95, p99, avg: d && d.avg, max: d && d.max, err,
      rps: reached ? n.count / (STEP_RAMP + STEP_HOLD) : undefined,
      pass: reached && reasons.length === 0,
      reasons,
    };
  });
}

function capacityReport(steps) {
  const out = [];
  out.push(line(), ' CAPACITY / BREAKING POINT ANALYSIS', line());
  out.push(
    `SLO per step: p95<${SLO.p95}ms, p99<${SLO.p99}ms, errors<${SLO.errorRate * 100}%` +
      `  (ramp ${STEP_RAMP}s + hold ${STEP_HOLD}s per step)`,
  );
  out.push('');
  out.push(
    pad('VUs', 7) + pad('req/s', 9) + pad('avg', 10) + pad('p95', 10) + pad('p99', 10) +
      pad('max', 10) + pad('err', 8) + 'verdict',
  );
  out.push('-'.repeat(78));
  for (const s of steps) {
    if (!s.reached) {
      out.push(pad(s.vus, 7) + '(not reached - test aborted earlier)');
      continue;
    }
    out.push(
      pad(s.vus, 7) + pad(s.rps.toFixed(1), 9) + pad(ms(s.avg), 10) + pad(ms(s.p95), 10) +
        pad(ms(s.p99), 10) + pad(ms(s.max), 10) + pad(pct(s.err), 8) +
        (s.pass ? 'PASS' : 'FAIL'),
    );
  }
  out.push('');

  let maxConsecutive = 0;
  for (const s of steps) {
    if (s.pass) maxConsecutive = s.vus;
    else break;
  }
  const firstFail = steps.find((s) => s.reached && !s.pass);
  const anyPassAfterFail = firstFail && steps.some((s) => s.pass && s.vus > firstFail.vus);
  const peakRps = steps.filter((s) => s.pass).reduce((m, s) => Math.max(m, s.rps), 0);
  const peakRpsAll = steps.filter((s) => s.reached).reduce((m, s) => Math.max(m, s.rps), 0);

  out.push(line('-'));
  out.push(` MAX CONSECUTIVE VUs WITHIN SLO : ${maxConsecutive || 'none (failed at lowest step)'}`);
  if (firstFail) {
    out.push(` FIRST BREAKING STEP            : ${firstFail.vus} VUs`);
    out.push(` BREAK REASON                   : ${firstFail.reasons.join('; ')}`);
    out.push(` (supported range is therefore ${maxConsecutive} < capacity < ${firstFail.vus} VUs)`);
    if (anyPassAfterFail) out.push(' NOTE: a higher step passed after a failure - results are noisy; re-run to confirm.');
  } else {
    out.push(` No step breached the SLO up to ${STEP_VUS[STEP_VUS.length - 1]} VUs - raise STEP_VUS to find the limit.`);
  }
  out.push(` PEAK THROUGHPUT WITHIN SLO     : ${peakRps.toFixed(1)} req/s`);
  out.push(` PEAK THROUGHPUT OBSERVED       : ${peakRpsAll.toFixed(1)} req/s`);
  out.push(line('-'));
  out.push(' Note: the load generator can itself be the bottleneck at high VU counts; watch');
  out.push(' k6 host CPU/network and the server host (CPU, fds, DB locks) when interpreting.');
  return { text: out.join('\n'), maxConsecutive, firstFail };
}

function overallReport(data) {
  const out = [];
  const dur = vals(data, 'http_req_duration');
  const failed = vals(data, 'http_req_failed');
  const reqs = vals(data, 'http_reqs');
  const iters = vals(data, 'iterations');
  const vusMax = vals(data, 'vus_max');
  const recv = vals(data, 'data_received');
  const sent = vals(data, 'data_sent');
  const testSecs = data.state.testRunDurationMs / 1000;

  out.push(line(), ` LOAD TEST SUMMARY  [${TEST_TYPE}]  target=${BASE_URL}`, line());
  out.push(`Duration            : ${testSecs.toFixed(1)}s`);
  out.push(`Peak VUs            : ${vusMax ? vusMax.max : 'n/a'}`);
  out.push(`Total requests      : ${reqs ? reqs.count : 0}  (${reqs ? reqs.rate.toFixed(1) : 0} req/s)`);
  out.push(`Iterations          : ${iters ? iters.count : 0}`);
  out.push(`Failed requests     : ${failed ? (failed.rate * 100).toFixed(2) : 0}%`);
  if (sent && recv) {
    out.push(`Data sent/received  : ${(sent.count / 1e6).toFixed(2)} MB / ${(recv.count / 1e6).toFixed(2)} MB`);
  }
  out.push('');
  if (dur) {
    out.push('Latency (http_req_duration)');
    out.push(`  avg ${ms(dur.avg)}  min ${ms(dur.min)}  med ${ms(dur.med)}`);
    out.push(`  p90 ${ms(dur['p(90)'])}  p95 ${ms(dur['p(95)'])}  p99 ${ms(dur['p(99)'])}  max ${ms(dur.max)}`);
    out.push('');
  }
  for (const phase of ['http_req_connecting', 'http_req_waiting', 'http_req_blocked']) {
    const v = vals(data, phase);
    if (v) out.push(`  ${pad(phase, 22)} avg ${ms(v.avg)}  p95 ${ms(v['p(95)'])}  max ${ms(v.max)}`);
  }
  out.push('');
  out.push('Per-endpoint breakdown');
  out.push(pad('  endpoint', 20) + pad('avg', 10) + pad('p95', 10) + pad('p99', 10) + pad('max', 10) + 'err');
  for (const e of ENDPOINTS) {
    const d = vals(data, `http_req_duration{endpoint:${e}}`);
    const f = vals(data, `http_req_failed{endpoint:${e}}`);
    if (!d) continue;
    out.push(
      pad(`  ${e}`, 20) + pad(ms(d.avg), 10) + pad(ms(d['p(95)']), 10) +
        pad(ms(d['p(99)']), 10) + pad(ms(d.max), 10) + pct(f && f.rate),
    );
  }
  out.push('');
  out.push('Checks');
  const checks = data.root_group && data.root_group.checks;
  if (checks) {
    const list = Array.isArray(checks) ? checks : Object.values(checks);
    for (const c of list) {
      const total = c.passes + c.fails;
      out.push(`  ${c.fails === 0 ? 'OK  ' : 'FAIL'} ${pad(c.name, 36)} ${c.passes}/${total} passed`);
    }
  }
  if (!IS_BREAK) {
    out.push('');
    out.push('Thresholds');
    for (const [name, m] of Object.entries(data.metrics)) {
      if (!m.thresholds || name.includes('{')) continue;
      for (const [expr, res] of Object.entries(m.thresholds)) {
        out.push(`  ${res.ok ? 'PASS' : 'FAIL'}  ${name}: ${expr}`);
      }
    }
  }
  return out.join('\n');
}

export function handleSummary(data) {
  let text = overallReport(data);
  let capacity = null;
  if (IS_BREAK) {
    const steps = stepResults(data);
    const rep = capacityReport(steps);
    text += '\n\n' + rep.text;
    capacity = {
      maxConsecutiveVUsWithinSLO: rep.maxConsecutive,
      firstBreakingStep: rep.firstFail ? rep.firstFail.vus : null,
      steps,
    };
  }
  text += '\n';
  return { stdout: text };
}
