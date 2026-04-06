# Contributing to OP Succinct

Thanks for your interest in contributing to OP Succinct!

## Code of Conduct

The OP Succinct project adheres to the Rust Code of Conduct. This code of conduct describes the minimum behavior expected from all contributors.

## Ways to contribute
There are fundamentally three ways an individual can contribute:

1. By opening an issue: For example, if you believe that you have uncovered a bug in OP-Succinct, creating a new issue in the issue tracker is the way to report it.
2. By adding context: Providing additional context to existing issues, such as screenshots and code snippets to help resolve issues.
3. By resolving issues: Typically this is done in the form of either demonstrating that the issue reported is not a problem after all, or more often, by opening a pull request that fixes the underlying problem, in a concrete and reviewable manner.

Anybody can participate in any stage of contribution. We urge you to participate in the discussion around bugs and participate in reviewing PRs.

## Reporting Bugs

If you're experiencing an issue, please open a [GitHub Issue](https://github.com/succinctlabs/op-succinct/issues).

## Contributions Related to Spelling and Grammar

At this time, we will not be accepting contributions that only fix spelling or grammatical errors in documentation, code or elsewhere.

## Backporting Changes

OP Succinct maintains multiple release lines (e.g., `main` for v4.x development, `release/v3.x` for v3.x maintenance). When a fix or non-breaking feature should be applied to a maintenance branch, it needs to be backported.

### How to Backport

1. Create a branch from `release/v3.x` and cherry-pick the merge commit:
   ```bash
   git fetch origin release/v3.x
   git checkout -b backport/<pr-number> origin/release/v3.x
   git cherry-pick -m 1 <merge-commit-sha>
   ```
2. Resolve any conflicts:
   - **Cargo.lock**: Don't merge manually — resolve `Cargo.toml` first, then run `cargo update`.
   - **ELF binaries**: Accept the current `release/v3.x` ELFs during cherry-pick, then rebuild from the backport branch with the correct SP1 toolchain. Never copy ELFs from `main`.
   - **Cargo.toml**: Keep the `release/v3.x` workspace version. Apply dependency changes from the source PR.
3. Run `cargo fmt --all` and `cargo clippy --all-features --all-targets -- -D warnings -A incomplete-features`.
4. Open a PR targeting `release/v3.x`.

### When to Backport

| Decision | When |
|----------|------|
| Backport | Bug fixes, non-breaking features, dependency upgrades (e.g., SP1 bumps) that should also be in v3.x |
| Skip | Breaking changes, features that depend on v4.x-only code, changes not applicable to v3.x |

*Adapted from the [Reth contributing guide](https://github.com/paradigmxyz/reth/blob/main/CONTRIBUTING.md).*  


