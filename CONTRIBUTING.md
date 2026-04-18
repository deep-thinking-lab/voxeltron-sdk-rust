# Contributing to Voxeltron

## Contribution Policy

Deep Thinking LLC does not currently accept unsolicited external contributions to Voxeltron. This is a pragmatic decision based on review bandwidth, not a permanent position.

If you have a patch you believe is critical, open an issue first so we can confirm fit and review path before you spend time on a pull request.

If a maintainer explicitly invites a pull request, the Contributor License Agreement must be signed before the PR can be merged. CLA Assistant or an equivalent check will prompt automatically when that flow is enabled in the org.

Repository-local draft CLA materials live in [docs/legal/README.md](docs/legal/README.md). Those drafts are for review and legal iteration only; they are not the operative signed agreements.

## Getting Started

```bash
git clone https://github.com/deep-thinking-llc/voxeltron.git
cd voxeltron
```

Install [mise](https://mise.jdx.dev/) for toolchain management, then:

```bash
mise install
```

This pins exact versions of Rust, Go, protoc, Bun, and Node.js via `.mise.toml`.

## Development Setup

### Rust (daemon)

```bash
cargo build --workspace
```

### Go (TUI)

```bash
cd tui
go build ./cmd/voxeltron
```

### Protobuf

Regenerate stubs after changing `.proto` files:

```bash
make proto
```

Do not commit generated protobuf files — CI runs `make proto`.

## Running Tests

### Rust

```bash
cargo test --workspace                  # All unit tests
cargo test -p voxeltrond <test_name>    # Single test (substring match)
```

### Go

```bash
cd tui
go test ./...
go test ./internal/ui/... -run TestSomething -v   # Single test
```

### Integration tests (require Docker)

```bash
cargo test --test '*' --workspace
```

## Code Style

### Rust

- **Format:** `cargo fmt` (rustfmt defaults). CI runs `cargo fmt --check`.
- **Lint:** `cargo clippy --workspace -- -D warnings`. Warnings are errors.
- **Audit:** `cargo audit` for CVE checks.

Key rules:
- No `.unwrap()` or `.expect()` in production code.
- Use `thiserror` for error enums, `anyhow` only at binary entry points.
- Group imports: std, external crates, internal crates (blank line between groups).
- No `unsafe` without a `// SAFETY:` comment.

### Go

- **Format:** `gofmt -w .`
- **Vet:** `go vet ./...`
- **Static analysis:** `staticcheck ./...`
- **Vuln scan:** `govulncheck ./...`

Key rules:
- Always return and wrap errors: `fmt.Errorf("context: %w", err)`.
- No `panic` in library code.
- Group imports: stdlib, external, internal (blank line separated).

## Commit Guidelines

Use [Conventional Commits](https://www.conventionalcommits.org/):

```
feat(proxy): add hot-swap backend routing
fix: correct TLS cert renewal race condition
refactor: extract build pipeline into separate module
test: add integration tests for database provisioning
docs: update deployment guide
chore: bump tokio to 1.38
```

Scope is optional. The body explains **why**, not what — the diff shows what.

Every commit must leave `cargo test --workspace` and `go test ./...` passing.

## Pull Request Process

1. Only open a PR after a maintainer has invited one.
2. Branch from `main`.
3. Keep commits focused — one logical change per PR.
4. Ensure all CI gates pass before requesting review:

| Check | Command |
|---|---|
| Rust format | `cargo fmt --check` |
| Rust lint | `cargo clippy --workspace -- -D warnings` |
| Rust tests | `cargo test --workspace` |
| Rust CVE scan | `cargo audit` |
| Go format | `gofmt -l .` |
| Go vet | `go vet ./...` |
| Go static analysis | `staticcheck ./...` |
| Go tests | `go test ./...` |
| Go vuln scan | `govulncheck ./...` |

5. Reference the relevant issue/discussion in the PR description.
6. Add a release-note entry with a **Feedback Linkage** section mapping source feedback -> implementation change.
7. Respond to review feedback promptly.

Use the pull request template in [.github/PULL_REQUEST_TEMPLATE.md](.github/PULL_REQUEST_TEMPLATE.md) for invited PRs.


### Feedback Traceability

> Required for user-facing changes.

When submitting user-facing work, include all of the following:

1. Source feedback link (Issue/Discussion/Discord thread)
2. Decision summary (accepted/rejected + rationale)
3. Commit or PR implementing the change
4. Release notes entry carrying the same linkage

> Update running evidence log: `docs/evidence/community/2026-02-26-feedback.md`

## Architecture Overview

```
crates/voxeltrond/       # Rust daemon binary (Tokio async runtime)
crates/voxeltron-proto/  # Protobuf definitions + generated Rust/Go stubs
crates/voxeltron-common/ # Shared types, errors, utilities
tui/                     # Go TUI binary (Bubbletea framework)
plugins/sdk/             # Plugin SDKs (Rust, JS, Go)
tests/integration/       # Integration tests (require Docker)
tests/e2e/               # End-to-end tests
tests/fixtures/          # Sample apps for build pipeline tests
site/                    # Astro marketing + docs site
```

The daemon (`voxeltrond`) and TUI (`voxeltron`) communicate over gRPC. Proto files live in `crates/voxeltron-proto/proto/` using proto3 syntax. Breaking changes to existing proto fields are forbidden — add new fields and deprecate old ones.

## License

Voxeltron is dual-licensed. See [LICENSE](LICENSE), [LICENSE-AGPL](LICENSE-AGPL), and [LICENSE-COMMERCIAL.md](LICENSE-COMMERCIAL.md). If a maintainer invites your contribution, you must also sign the CLA before the change can be merged.
