## Supply-Chain Guardrails

- **Automated Scanning**
  - GitHub Actions workflow `security.yml` runs `npm audit --omit=dev --audit-level=high`, `cargo audit`, `dependency-review-action`, and a smoke build (`cargo check`, `npm run check`) on every pull request and weekly on `main`.
  - Failing audit jobs block merges; treat high/critical advisories as release-blocking incidents.

- **Dependency Updates**
  - Dependabot (`.github/dependabot.yml`) opens weekly refresh PRs for npm and Cargo ecosystems with labels for easy triage.
  - Prefer merging updates promptly; if a bump cannot ship immediately, capture the risk in an issue with owner & due date.

- **Lockfile Hygiene**
  - Regenerate `client/package-lock.json` with `npm ci` and `server/Cargo.lock` with `cargo update` after dependency changes.
  - Review diff for transitive additions; reject packages lacking clear licenses or active maintenance.

- **Third-Party Vetting**
  - Record new dependencies (purpose, maintainer reputation, license) in PR descriptions.
  - For crates/npm packages that ship native code or post-install scripts, perform manual source review before adoption.

- **CI Enhancements (Optional)**
  - Add Trivy image scans for `client` and `server` Docker images, failing on `CRITICAL` CVEs.
  - Integrate OpenSSF Scorecard or SLSA checks for repository health metrics.

