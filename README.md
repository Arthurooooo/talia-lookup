# Talia Lookup (Tauri)

Version Windows + Mac de l'outil de recherche croisée d'élèves Bubble. Stack : Rust (Tauri 2) + SvelteKit + TypeScript.

## Run en dev

```bash
pnpm install
pnpm tauri dev
```

## Pour publier une nouvelle version

Voir **[RELEASING.md](./RELEASING.md)** pour le détail. En résumé : bump la version, tag, push, GitHub Actions s'occupe du reste, les utilisateurs reçoivent l'update au prochain démarrage.

## Architecture

- `src-tauri/src/bubble.rs` — client HTTP Bubble (mêmes endpoints qu'avant).
- `src-tauri/src/lib.rs` — commandes Tauri, tray icon, plugins (updater, clipboard, store, keyring).
- `src/routes/+page.svelte` — UI principale (3 vues : settings / search / detail).
- `src/lib/components/*.svelte` — composants UI.
- `.github/workflows/release.yml` — build cross-platform sur push de tag `v*`.
- `.keys/talia.key` (gitignoré !) — clé privée de signature des updates.
- `.keys/talia.key.pub` — clé publique (intégrée dans `tauri.conf.json`).

## Système de mise à jour

- L'app check l'endpoint au démarrage (URL dans `tauri.conf.json` → `plugins.updater.endpoints`).
- Si le `latest.json` indique une version plus récente, télécharge le binaire signé.
- Vérifie la signature avec la clé publique embarquée.
- Installe et relance.
