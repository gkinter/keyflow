## Keyflow Infrastructure Hardening Checklist

### Network & TLS
- Terminate HTTPS with a managed load balancer (e.g., Cloudflare, AWS ALB) in front of the Rust API. Enforce TLS 1.2+ and redirect plaintext traffic to HTTPS.
- Restrict inbound traffic: expose the API and SvelteKit frontend via a public edge network, keep Postgres on a private subnet with security group rules that only allow the API service IPs.
- Enable Web Application Firewall (WAF) rules for common OWASP top-10 protections (rate limiting, IP reputation, bot mitigation).
- Preserve client IPs: ensure the edge forwards `X-Forwarded-For` or Proxy Protocol headers and configure the API service to trust only your ingress. Per-IP rate limiting depends on accurate source addresses.

### Secrets & Configuration
- Store secrets (database URL, GitHub OAuth credentials, session signing keys) in a dedicated secret manager (AWS Secrets Manager, 1Password SCIM Bridge, Doppler, etc.). Grant read permissions only to the CI/CD role and production runtime principal.
- Rotate secrets at least quarterly and after any suspected compromise. Keep multiple entries in `SESSION_SIGNING_KEYS` so you can roll keys without invalidating sessions; remove the oldest key only after all cookies signed with it have expired.

### Runtime Security
- Container images run as non-root, with `no-new-privileges`, read-only root filesystem, and a dedicated tmpfs mount for ephemeral writes (see `docker-compose.yml`).
- Pin image digests in production deployment manifests and scan images for vulnerabilities (e.g., using Trivy or AWS ECR scanning) as part of CI.
- Enable process-level monitoring (Falco/Aqua) to detect privilege escalation or unexpected syscalls.
- Serve the API over HTTPS so cookies flagged `SameSite=None; Secure` remain valid. Double-check that `APP_BASE_URL` matches the externally reachable scheme/host to avoid session rejection.

### Observability & Logging
- Ship structured logs to a centralized sink (Datadog, Loki, CloudWatch) and tag auth events (`user authenticated`, `session validated`, `user logged out`) for alerting on suspicious patterns.
- Enable request metrics (latency, error rate) via an ingress layer or service mesh and alert on unusual spikes in 4xx/5xx codes.
- Configure database audit logging for role changes and data export operations.

### Rate Limiting & Abuse Protection
- Apply per-IP and per-user rate limits at the edge (e.g., Cloudflare Rules or Envoy) for authentication endpoints (`/auth/github/*`) and task mutation routes. Start with low limits (e.g., 60 requests/minute) and adjust as needed.
- Implement exponential backoff for repeated session validation failures to slow brute-force attempts.
- Keep the runtime limiter (`AUTH_RATE_LIMIT_PER_MINUTE`, `AUTH_RATE_LIMIT_BURST`) aligned with your edge settings so responses stay consistent and audit logs remain actionable.

### Backups & Disaster Recovery
- Automate Postgres backups (daily full, 15-minute WAL point-in-time recovery) and test restores quarterly.
- Version migrations in `server/migrations/` and consider `sqlx migrate info` checks in CI to prevent drift.
- Store build artifacts in an immutable registry; enable image retention policies plus signed releases.

### CI/CD Recommendations
- Require signed commits and branch protection for `main`.
- Run `npm audit`, `cargo audit`, linting, and tests on every pull request. Fail the pipeline on critical/high vulnerabilities.
- Deploy via Infrastructure as Code (Terraform/Pulumi) with peer review. Use separate staging and production environments with isolated credentials.

