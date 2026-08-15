# Protección de main — Contrato

## Branch Protection Rules (GitHub API)

```json
{
  "required_status_checks": {
    "strict": true,
    "contexts": [
      "Check (stable)",
      "Check (MSRV 1.80)",
      "Format",
      "Clippy",
      "Test",
      "Doc",
      "Coverage"
    ]
  },
  "enforce_admins": true,
  "required_pull_request_reviews": {
    "required_approving_review_count": 1,
    "require_code_owner_reviews": true,
    "dismiss_stale_reviews": true
  },
  "restrictions": null,
  "allow_force_pushes": false,
  "allow_deletions": false,
  "required_linear_history": true
}
```

Fuente operativa: [`docs/branch-protection.json`](../../docs/branch-protection.json).

## Renovate contract (sustituye Dependabot)

Configuración: [`.github/renovate.json`](../../.github/renovate.json).

| Ecosistema | Restricción | Razón |
|------------|-------------|-------|
| cargo | `nom` < 8 | Breaking (trait `Parser`) — ADR-0009 |
| cargo | `toml` < 1 | Breaking — ADR-0009 |
| cargo | `chardetng` < 1 | Breaking — ADR-0009 |
| cargo | `tempfile` < 3.20 | Pin temporal |
| github-actions | `dtolnay/rust-toolchain` < 1.100 | Evitar deriva MSRV accidental |

## CI requirements

| Job | Toolchain | Comando |
|-----|-----------|---------|
| Check (stable) | stable | `cargo check --all-features` |
| Check (MSRV) | 1.80 | `cargo check --all-features` |
| Format | stable | `cargo fmt --all -- --check` |
| Clippy | stable | `cargo clippy --all-features -- -D warnings` |
| Test | stable | `cargo test --all-features` |
| Doc | stable | `cargo doc --no-deps --document-private-items` |
| Coverage | stable | `cargo llvm-cov --lcov --output-path coverage/lcov.info` → Coveralls |

Runners: `[self-hosted, ts]` (ADR-0008).
