# Explorer++ 🧭⚡

The stylish, fast file explorer powered by SvelteKit + Tauri. A desktop app that feels like the web — with Rust muscle under the hood.

![Svelte](https://img.shields.io/badge/Svelte-5-orange) ![Tauri](https://img.shields.io/badge/Tauri-2-blue) ![Rust](https://img.shields.io/badge/Rust-stable-brown) ![pnpm](https://img.shields.io/badge/pnpm-10-4a8) ![TailwindCSS](https://img.shields.io/badge/Tailwind-4-38bdf8)

In short: modern UI, native performance. Double‑click and go. 💫

---

## Quick Start

1) Install prerequisites
- Node.js ≥ 18 and pnpm ≥ 8 (recommended): https://pnpm.io/installation
- Rust (stable) + Cargo: https://rustup.rs
- Tauri system requirements:
  - Windows: Visual Studio Build Tools (C++), WebView2 Runtime
  - macOS: Xcode Command Line Tools
  - Linux: See Tauri docs for packages (webkit2gtk, libayatana‑appindicator, etc.)

2) Install dependencies
```
pnpm install
```

3) Configure start folder (optional but recommended)
- Create `.env.local` in the project root and set `PUBLIC_START_PATH`:
  - Windows example: `PUBLIC_START_PATH="C:\\Users\\YourName"`
  - macOS/Linux example: `PUBLIC_START_PATH="/Users/yourname"`

4) Run the desktop app in dev mode (Tauri)
```
pnpm tauri dev
```
– launches the Rust backend and Svelte dev server at `http://localhost:1420` inside a native window.

5) Browser‑only development (optional)
```
pnpm dev
```
– serves the app as a web app in your browser.

6) Build for production
- Desktop build (installer/bundle):
```
pnpm tauri build
```
– artifacts are under `src-tauri/target/release/bundle/*`.

- Static web build (SPA):
```
pnpm build && pnpm preview
```

---

## Features (current)
- Fast directory listing (Rust command `list_files`).
- List and grid views with sorting.
- Search field, breadcrumb navigation, refresh button.
- Modern, accessible UI with Tailwind 4 and Bits‑UI.

Planned/ideas: open on double‑click, context menus, multi‑select, tabs, favorites, and more.

---

## Useful Scripts
- `pnpm dev`: Svelte dev server (browser).
- `pnpm build`: Production web build to `build`.
- `pnpm preview`: Local preview of the web build.
- `pnpm check`: Type and Svelte checks.
- `pnpm tauri dev`: Desktop app in dev mode.
- `pnpm tauri build`: Build native bundles/installers.

---

## Configuration
- `PUBLIC_START_PATH`: Initial directory when the app opens.
  - Windows: Double‑escape backslashes in `.env.local` (`C:\\path\\to\\folder`).
  - `.env.local` is git‑ignored and overrides local settings.
- Port/dev URL: `vite.config.js` sets `1420` (Tauri expects a fixed port; `strictPort: true`).

---

## Project Structure (short)
- `src/`: Frontend (SvelteKit, UI components).
- `src/routes/+page.svelte`: Main view with directory list.
- `src/components/Toolbar.svelte`: Toolbar with search, sorting, view switch.
- `src-tauri/src/lib.rs`: Rust backend, incl. the `list_files` command.
- `src-tauri/tauri.conf.json`: Tauri app configuration.

---

## Troubleshooting
- Port 1420 in use: close the other process or free the port (Tauri dev expects 1420).
- Windows build issues: ensure “Desktop development with C++” (MSVC) and the WebView2 runtime are installed.
- Linux dependencies: install the packages listed in the Tauri docs (webkit2gtk, etc.).
- Paths in `.env.local`: on Windows always double‑escape backslashes.
- Blank screen in dev: reinstall deps and clear cache (`rm -rf .svelte-kit`), then `pnpm install` + `pnpm tauri dev`.

---

## Recommended IDE Extensions
- VS Code: Svelte, Tauri, rust‑analyzer.

---

## License
MIT

---

## Contributing
PRs & issues welcome! Share ideas for features or design.
