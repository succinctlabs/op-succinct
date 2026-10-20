const { lstatSync, readFileSync } = require('node:fs');
const { join } = require('node:path');

const reports = {
  '.github/workflows/cargo-tests.yml': {
    name: 'Cargo Tests', file: 'execution_report.md', kind: 'execute-batch',
  },
  '.github/workflows/cycle-count-diff.yml': {
    name: 'Cycle Count Diff', file: 'cycle_count_report.md', kind: 'cycle-count-diff',
  },
};

module.exports = async ({ github, context, core, reportDir }) => {
  const run = context.payload.workflow_run;
  const report = reports[run.path];
  const repository = `${context.repo.owner}/${context.repo.repo}`;
  if (run.event !== 'pull_request' || run.conclusion !== 'success' ||
      !report || run.name !== report.name || run.repository.full_name !== repository) {
    throw new Error('Expected a successful performance workflow pull_request run in this repository');
  }

  // Fork runs can have an empty pull_requests list. Resolve the PR using API metadata.
  const pulls = await github.paginate(github.rest.pulls.list, {
    ...context.repo, state: 'open', per_page: 100,
    head: `${run.head_repository.owner.login}:${run.head_branch}`,
  });
  const matchesRun = (pr) => pr.state === 'open' &&
    pr.base.repo.full_name === repository &&
    pr.head.repo?.id === run.head_repository.id &&
    pr.head.ref === run.head_branch && pr.head.sha === run.head_sha;
  const candidates = pulls.filter(matchesRun);
  if (candidates.length === 0) {
    core.info('No open PR has this repository, branch and head SHA; skip stale or closed results');
    return;
  }
  if (candidates.length !== 1) {
    throw new Error('More than one open PR matches this workflow run');
  }
  const pr = candidates[0];
  const path = join(reportDir, report.file);
  const stat = lstatSync(path);
  if (!stat.isFile() || stat.size > 60_000) {
    throw new Error('Expected a regular performance report of at most 60000 bytes');
  }
  // Artifact text is data. Never import it or pass it to a shell.
  const markdown = readFileSync(path, 'utf8').trim();
  if (!markdown) throw new Error('Performance report is empty');

  const marker = `<!-- op-succinct-performance:${report.kind} -->`;
  const comments = await github.paginate(github.rest.issues.listComments, {
    ...context.repo, issue_number: pr.number, per_page: 100,
  });
  const previous = comments.find((comment) =>
    comment.user?.login === 'github-actions[bot]' &&
    comment.body?.startsWith(`${marker}\n`));
  const previousRun = previous?.body.match(/<!-- run:(\d+) attempt:(\d+) -->/);
  if (previousRun && (Number(previousRun[1]) > run.id ||
      (Number(previousRun[1]) === run.id && Number(previousRun[2]) > run.run_attempt))) {
    core.info('A newer performance result is already posted');
    return;
  }

  // The PR may advance while the artifact or comments are being read.
  const { data: current } = await github.rest.pulls.get({ ...context.repo, pull_number: pr.number });
  if (!matchesRun(current)) {
    core.info('PR changed before publication; skip stale results');
    return;
  }
  const body = `${marker}\n<!-- run:${run.id} attempt:${run.run_attempt} -->\n` +
    `${markdown}\n\n[Workflow run](${run.html_url}) · Commit \`${run.head_sha}\``;
  if (previous) {
    await github.rest.issues.updateComment({ ...context.repo, comment_id: previous.id, body });
  } else {
    await github.rest.issues.createComment({ ...context.repo, issue_number: pr.number, body });
  }
};
