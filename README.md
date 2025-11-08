# Keyflow

Keyflow is a keyboard-first task manager with Git integration. Inspired by the simplicity of Superhuman, it is designed to help developers stay focused, organized, and in flow, all without touching the mouse. Manage your tasks, sync with Git issues, and navigate everything efficiently through the keyboard.

**TODO: Actually write the README rather than getting ChatGPT to generate a very vague one!**

## Features

- **Keyboard-First Navigation**: Keyflow is fully operable with keyboard shortcuts, making it fast and efficient.
- **Git Integration**: Seamlessly sync with your Git repositories. Convert issues into tasks, manage branches, and more.
- **Task Management**: Create, edit, and prioritize tasks with a clean, simple interface.
- **Command Palette**: Perform any action with a quick command palette, keeping you in flow.

## Tech Stack

- **Frontend**: [SvelteKit](https://kit.svelte.dev/)
- **Backend**: [Rust](https://www.rust-lang.org/) with [PostgreSQL](https://www.postgresql.org/)
- **Styling**: [Tailwind CSS](https://tailwindcss.com/)

## Getting Started

### Prerequisites

To get started with Keyflow, you'll need the following installed on your machine:

- [Node.js](https://nodejs.org/) (v16 or later)
- [PostgreSQL](https://www.postgresql.org/)
- [Git](https://git-scm.com/)
- [Rust](https://www.rust-lang.org/)

### Installation

1. **Clone the repository**:
   ```sh
   git clone https://github.com/yourusername/keyflow.git
   cd keyflow
   ```

2. **Install dependencies**:
   ```sh
   npm install
   ```

3. **Set up the database**:
    - Create a PostgreSQL database named `keyflow`.
    - Enable the `pgcrypto` extension (`CREATE EXTENSION IF NOT EXISTS "pgcrypto";`).
    - Run `sqlx migrate run` from `server/` to apply database migrations.

4. **Configure environment variables**:
    - Copy the templates and fill them with real values:
      ```sh
      cp client/env.example client/.env
      cp server/env.example server/.env
      ```
    - **Client** (`client/.env`):
      - `VITE_API_BASE_URL`: The origin of the Rust API (default `http://localhost:8080`).
    - **Server** (`server/.env`):
      - `DATABASE_URL`: Connection string for the Postgres instance.
      - `GITHUB_CLIENT_ID` / `GITHUB_CLIENT_SECRET`: Credentials for your GitHub OAuth app.
      - `SESSION_SIGNING_KEYS`: Comma-separated list of 32+ character secrets used to sign session tokens. The first key is used to issue new cookies; older keys remain valid so you can rotate without logging users out.
      - `APP_BASE_URL`: Public URL where the Rust API is reachable (used for OAuth callbacks).
      - `FRONTEND_ORIGIN`: Allowed origin for the SvelteKit frontend.
      - `PORT`: Port for the API server (default `8080`).
      - `AUTH_RATE_LIMIT_PER_MINUTE` / `AUTH_RATE_LIMIT_BURST`: Per-IP throttling for `/auth/*` routes (defaults to 60 requests/minute with a burst of 10).
      - `ALLOW_INSECURE_COOKIES`: Set to `true` only for local development; in production the API must be served via HTTPS so session cookies are accepted by modern browsers.

5. **Run the development servers**:
   - From `server/`, run `cargo run` to start the Rust API.
   - From `client/`, run `npm run dev` to start the SvelteKit app.

6. **Access the application**:
    - Open your browser and navigate to `http://localhost:5173`.

## Security

- **Architecture Overview**: The `client/` SvelteKit frontend communicates with the Rust API in `server/`. The API will be responsible for user authentication, task data access, and GitHub integration. Treat the API and database as the primary trust boundary; all incoming requests must be authenticated and authorized before accessing user data. Static assets are served from the frontend build; no secrets should be bundled in the client.
- **Secrets Management**: Never commit real secrets. Copy the environment templates (`client/env.example`, `server/env.example`) into `.env` files for local development. Rotate GitHub credentials and session keys regularly, and store production secrets in a managed secrets store (e.g., 1Password, AWS Secrets Manager, GitHub Actions Secrets). Enforce least privilege on database users and use dedicated accounts for automation. Keep at least two active `SESSION_SIGNING_KEYS` so that key rotation does not evict active sessions. Require HTTPS in every environment (set `ALLOW_INSECURE_COOKIES=false`) so browsers accept `SameSite=None; Secure` session cookies.
- **Local Onboarding**: For local setup, populate placeholders in the `.env` files with development credentials (scoped GitHub OAuth app, throwaway session keys, local Postgres URL). When onboarding new developers, share secrets through secure channels and revoke access for inactive users.
- **Operational Practices**: Plan for periodic security reviews, dependency patching, and log monitoring. Restrict deployment credentials to CI/CD roles, enforce MFA on developer accounts, configure security logging/alerting for authentication events, and ensure backups and restores are regularly tested.
- **Authentication Flow**: OAuth logins are handled via GitHub using the Rust API as the callback handler. The backend issues short-lived, signed, HTTP-only session cookies (`SameSite=None; Secure` in production) and validates them on every request. CSRF is mitigated with state & PKCE during OAuth plus a double-submit token: the server sets a readable `kf_csrf` cookie and exposes the same value from `/session`, and clients must mirror it in the `X-CSRF-Token` header for any state-changing request (e.g., logout). Per-IP rate limiting protects `/auth/*` routes and emits warnings when throttled.

## Internationalization

- Locale-aware routing (`/{locale}/...`) is enabled for the following locales: `en-US`, `en-GB`, `fr-FR`, `de-DE`, `es-ES`, `it-IT`, `ja-JP`, `zh-CN`, `zh-TW`, `ko-KR`. Additional locales can be added through `src/lib/i18n/locales.ts`.
- UI strings are sourced from i18n keys via `svelte-i18n`. Add or update translations under `src/lib/i18n/translations/`.
- Runtime locale negotiation order: URL prefix, stored preference (future work), then `Accept-Language`, defaulting to `en-US`.
- Use the locale switcher in the header to preview other locales during development; routes fall back to English copy until localized strings are provided.

## Deployment

- Run `docker-compose up --build` to bootstrap Postgres, the Rust API, and the SvelteKit frontend with hardened defaults (non-root containers, read-only rootfs, tmpfs for `/tmp`).
- Review `docs/infra/DEPLOYMENT.md` for TLS termination, secret management, logging, and rate-limiting guidance before promoting to production.

## Maintenance

- Automated checks (`.github/workflows/security.yml`) run `npm audit`, `cargo audit`, dependency reviews, and a cross-stack smoke build (`cargo check`, `npm run check`) on PRs and weekly. Treat failures as release blockers.
- Dependabot (`.github/dependabot.yml`) proposes weekly npm & Cargo updates; track outstanding updates and document risk acceptances.
- Review the supply-chain playbook in `docs/security/SUPPLY_CHAIN.md` for guidance on vetting third-party packages and keeping lockfiles healthy.

## Usage

- **Command Palette**: Press `Ctrl+K` to open the command palette.
- **Create Task**: Use the command palette or press `N` to create a new task.
- **Navigate**: Use `Tab` and `Enter` to navigate through the application seamlessly.

## Contributing

Contributions are welcome! To get started:

1. Fork the repository.
2. Create a new branch (`git checkout -b feature/your-feature`).
3. Commit your changes (`git commit -m 'Add new feature'`).
4. Push to the branch (`git push origin feature/your-feature`).
5. Open a Pull Request.

Please make sure your code is well-tested and follows the project's style guidelines.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Inspired by [Superhuman](https://superhuman.com/).
- Built with passion for developers who want to stay productive and focused.
