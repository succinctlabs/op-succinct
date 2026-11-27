const assert = require('node:assert/strict');
const { mkdtempSync, writeFileSync, symlinkSync, rmSync } = require('node:fs');
const { tmpdir } = require('node:os');
const { join } = require('node:path');
const { test } = require('node:test');
const publish = require('./publish-performance-report.cjs');

function fixture(t) {
  const reportDir = mkdtempSync(join(tmpdir(), 'performance-report-'));
  t.after(() => rmSync(reportDir, { recursive: true, force: true }));
  writeFileSync(join(reportDir, 'cycle_count_report.md'), '## Performance Comparison\n| Cycles | 123 |');
  writeFileSync(join(reportDir, 'execution_report.md'), '## Execution Stats\n| Cycles | 456 |');
  const run = {
    id: 42, run_attempt: 1, event: 'pull_request', conclusion: 'success',
    path: '.github/workflows/cycle-count-diff.yml', name: 'Cycle Count Diff',
    repository: { full_name: 'succinctlabs/op-succinct' },
    head_repository: { id: 9, owner: { login: 'contributor' } },
    head_branch: 'fix/example', head_sha: 'a'.repeat(40), pull_requests: [],
    html_url: 'https://github.com/succinctlabs/op-succinct/actions/runs/42',
  };
  const pr = {
    number: 17, state: 'open', base: { repo: { full_name: 'succinctlabs/op-succinct' } },
    head: { repo: { id: 9 }, ref: run.head_branch, sha: run.head_sha },
  };
  const writes = [];
  const calls = [];
  const comments = [];
  const github = { rest: {
    pulls: { list: () => {}, get: async () => ({ data: pr }) },
    issues: {
      listComments: () => {},
      createComment: async (value) => writes.push({ type: 'create', ...value }),
      updateComment: async (value) => writes.push({ type: 'update', ...value }),
    },
  } };
  const pulls = [pr];
  github.paginate = async (method, args) => {
    calls.push(args);
    if (method === github.rest.pulls.list) return pulls;
    if (method === github.rest.issues.listComments) return comments;
    throw new Error('Unexpected API call');
  };
  const context = { repo: { owner: 'succinctlabs', repo: 'op-succinct' }, payload: { workflow_run: run } };
  const invoke = () => publish({ github, context, core: { info: () => {} }, reportDir });
  return { run, pr, pulls, comments, github, writes, calls, invoke, reportDir };
}

test('posts on the fork PR even when workflow_run.pull_requests is empty', async (t) => {
  const f = fixture(t);
  await f.invoke();
  assert.equal(f.calls[0].head, 'contributor:fix/example');
  assert.equal(f.writes.length, 1);
  assert.equal(f.writes[0].issue_number, 17);
  assert.match(f.writes[0].body, /op-succinct-performance:cycle-count-diff/);
  assert.match(f.writes[0].body, /Cycles \| 123/);
  assert.match(f.writes[0].body, /actions\/runs\/42/);
});

test('same-repository and Dependabot PRs use the same publisher', async (t) => {
  const f = fixture(t);
  f.run.head_repository.owner.login = 'succinctlabs';
  f.run.actor = { login: 'dependabot[bot]' };
  await f.invoke();
  assert.equal(f.calls[0].head, 'succinctlabs:fix/example');
  assert.equal(f.writes.length, 1);
});

for (const change of [
  (f) => { f.run.event = 'push'; },
  (f) => { f.run.conclusion = 'failure'; },
  (f) => { f.run.path = '.github/workflows/other.yml'; },
  (f) => { f.run.name = 'Other'; },
  (f) => { f.run.repository.full_name = 'contributor/other'; },
]) {
  test(`rejects an ineligible source run: ${change}`, async (t) => {
    const f = fixture(t);
    change(f);
    await assert.rejects(f.invoke, /Expected a successful performance workflow/);
    assert.equal(f.writes.length, 0);
  });
}

for (const [field, change] of [
  ['head SHA', (pr) => { pr.head.sha = 'b'.repeat(40); }],
  ['head repository', (pr) => { pr.head.repo.id = 99; }],
  ['head branch', (pr) => { pr.head.ref = 'other'; }],
  ['base repository', (pr) => { pr.base.repo.full_name = 'contributor/other'; }],
  ['closed PR', (pr) => { pr.state = 'closed'; }],
]) {
  test(`skips a PR with a different ${field}`, async (t) => {
    const f = fixture(t);
    change(f.pr);
    await f.invoke();
    assert.equal(f.writes.length, 0);
  });
}

test('does not guess when multiple PRs match', async (t) => {
  const f = fixture(t);
  f.pulls.push({ ...f.pr, number: 18 });
  await assert.rejects(f.invoke, /More than one open PR/);
  assert.equal(f.writes.length, 0);
});

test('rechecks the PR head immediately before writing', async (t) => {
  const f = fixture(t);
  f.github.rest.pulls.get = async () => ({ data: {
    ...f.pr, head: { ...f.pr.head, sha: 'b'.repeat(40) },
  } });
  await f.invoke();
  assert.equal(f.writes.length, 0);
});

test('updates only the matching report comment owned by the Actions bot', async (t) => {
  const f = fixture(t);
  const marker = '<!-- op-succinct-performance:cycle-count-diff -->\n';
  f.comments.push(
    { id: 1, user: { login: 'github-actions[bot]' }, body: 'Unrelated bot report' },
    { id: 2, user: { login: 'contributor' }, body: marker },
    { id: 3, user: { login: 'github-actions[bot]' }, body: marker },
  );
  await f.invoke();
  assert.equal(f.writes[0].type, 'update');
  assert.equal(f.writes[0].comment_id, 3);
});

test('does not overwrite a newer run or rerun', async (t) => {
  for (const metadata of ['run:43 attempt:1', 'run:42 attempt:2']) {
    const f = fixture(t);
    f.comments.push({ id: 1, user: { login: 'github-actions[bot]' },
      body: `<!-- op-succinct-performance:cycle-count-diff -->\n<!-- ${metadata} -->` });
    await f.invoke();
    assert.equal(f.writes.length, 0);
  }
});

test('Cargo Tests and Cycle Count Diff keep separate comments', async (t) => {
  const f = fixture(t);
  f.run.path = '.github/workflows/cargo-tests.yml';
  f.run.name = 'Cargo Tests';
  f.comments.push({ id: 1, user: { login: 'github-actions[bot]' },
    body: '<!-- op-succinct-performance:cycle-count-diff -->\nExisting comparison' });
  await f.invoke();
  assert.equal(f.writes[0].type, 'create');
  assert.match(f.writes[0].body, /op-succinct-performance:execute-batch/);
  assert.match(f.writes[0].body, /Cycles \| 456/);
});

for (const [name, content] of [['empty', '  '], ['oversize', 'x'.repeat(60_001)]]) {
  test(`rejects an ${name} report`, async (t) => {
    const f = fixture(t);
    writeFileSync(join(f.reportDir, 'cycle_count_report.md'), content);
    await assert.rejects(f.invoke);
    assert.equal(f.writes.length, 0);
  });
}

test('rejects a symlink instead of reading a file outside the report', async (t) => {
  const f = fixture(t);
  rmSync(join(f.reportDir, 'cycle_count_report.md'));
  symlinkSync(__filename, join(f.reportDir, 'cycle_count_report.md'));
  await assert.rejects(f.invoke, /regular performance report/);
  assert.equal(f.writes.length, 0);
});

test('treats script-like artifact content as plain comment data', async (t) => {
  const f = fixture(t);
  const text = '$(touch SHOULD_NOT_EXIST)\n${{ secrets.GITHUB_TOKEN }}\nrequire("node:fs").writeFileSync("SHOULD_NOT_EXIST", "");';
  writeFileSync(join(f.reportDir, 'cycle_count_report.md'), text);
  await f.invoke();
  assert.ok(f.writes[0].body.includes(text));
});

test('reports a GitHub posting failure', async (t) => {
  const f = fixture(t);
  f.github.rest.issues.createComment = async () => { throw new Error('403'); };
  await assert.rejects(f.invoke, /403/);
});
