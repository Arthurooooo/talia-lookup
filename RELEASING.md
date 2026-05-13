# Publier une version (Mac + Windows)

Toute la pipeline tourne sur GitHub Actions. Tu dois faire l'**install initiale UNE seule fois** (étapes 1-3), puis chaque release c'est juste les étapes 4-5.

---

## Setup initial (une fois)

### 1. Crée le repo GitHub

```bash
cd /Users/arthur/talia-tool-tauri
git init
git add .
git commit -m "Initial commit"
# Crée le repo sur github.com (web), puis :
git remote add origin git@github.com:TON_USER/talia-lookup.git
git push -u origin main
```

### 2. Mets à jour l'URL d'updates dans `tauri.conf.json`

Remplace `REPLACE_OWNER/REPLACE_REPO` par ton vrai chemin :

```jsonc
"endpoints": [
  "https://github.com/TON_USER/talia-lookup/releases/latest/download/latest.json"
]
```

Commit cette modif.

### 3. Configure les secrets GitHub

Va dans **Settings → Secrets and variables → Actions → New repository secret** et crée :

- **TAURI_SIGNING_PRIVATE_KEY** : copie-colle TOUT le contenu de `.keys/talia.key` (le fichier en clair, base64). Sur Mac : `pbcopy < .keys/talia.key`
- **TAURI_SIGNING_PRIVATE_KEY_PASSWORD** : laisse vide (j'ai généré la clé sans password ; si tu veux ajouter un password plus tard, regénère et remplace)

⚠️ **Backup** : copie `.keys/talia.key` dans 1Password ou ailleurs. Si tu perds cette clé, tu ne pourras PLUS jamais publier d'update qui s'installe automatiquement chez tes utilisateurs (ils devraient réinstaller manuellement la nouvelle version, car la signature ne correspondra plus à la clé publique embarquée).

### 4. Premier `tauri build` local pour valider

```bash
pnpm tauri build
```

Si ça produit `src-tauri/target/release/bundle/dmg/Talia Lookup_0.1.0_aarch64.dmg` (Mac) sans erreur, tu es bon.

---

## Publier une nouvelle version

### 1. Bump la version

Dans **3 fichiers**, mets le même numéro (ex. `0.2.0`) :

- `package.json` → `"version": "0.2.0"`
- `src-tauri/Cargo.toml` → `version = "0.2.0"`
- `src-tauri/tauri.conf.json` → `"version": "0.2.0"`

Astuce : un seul script de bump :

```bash
./bump-version.sh 0.2.0   # voir le script fourni
```

### 2. Tag + push

```bash
git add -A
git commit -m "v0.2.0"
git tag v0.2.0
git push && git push --tags
```

### 3. C'est tout

GitHub Actions :
1. Détecte le tag `v*`
2. Build en parallèle sur **macos-latest arm64 + macos-latest x86_64 + windows-latest**
3. Signe chaque binaire avec ta clé privée (depuis le secret)
4. Crée une **GitHub Release** (en draft) avec :
   - `Talia.Lookup_0.2.0_aarch64.dmg` (Mac Apple Silicon)
   - `Talia.Lookup_0.2.0_x64.dmg` (Mac Intel)
   - `Talia.Lookup_0.2.0_x64-setup.exe` (Windows installer NSIS)
   - `Talia.Lookup_0.2.0_x64_en-US.msi` (Windows MSI)
   - `latest.json` (manifest de mise à jour signé)
5. Va dans l'onglet Releases, vérifie le draft, clique **Publish**.

**Côté utilisateur**, dès qu'il relance l'app :
- L'app lit `https://github.com/TON_USER/talia-lookup/releases/latest/download/latest.json`
- Voit que la version dans le manifest > version installée
- Télécharge le binaire signé, vérifie la signature, remplace l'app, relance.

Aucune action requise par l'utilisateur.

---

## Test local de l'updater (sans tag)

Pour valider sans push :

```bash
# build local avec une version basse
pnpm tauri build

# install l'app
open src-tauri/target/release/bundle/macos/Talia\ Lookup.app

# bump la version, re-build, publie un latest.json bidon sur un serveur local
# (technique avancée, demande-moi si besoin)
```

---

## Distribution initiale (avant que les updates auto soient en place)

Pour ton premier rollout — quand tes collègues n'ont encore rien d'installé :

1. Push le tag `v0.1.0`
2. Publie le draft que GitHub Actions a généré (clique sur Publish)
3. Récupère l'URL du `.exe` (Windows) et du `.dmg` (Mac), envoie-les aux collègues par Slack
4. Eux installent → l'app embarque déjà la clé publique → toutes les futures versions se mettront à jour toutes seules
