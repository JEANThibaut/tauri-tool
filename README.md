# 🛠️ Tauri Multi-Tool Suite

Suite d'applications de bureau développées avec Tauri, organisées en **workspace Cargo** pour partager du code commun tout en compilant chaque outil séparément.

## 📁 Structure du Projet

```
tauri-tool/
├── Cargo.toml              # Configuration du workspace Cargo
├── common/                 # Code Rust partagé entre tous les outils
│   ├── src/
│   │   └── lib.rs         # Fonctions utilitaires communes
│   └── Cargo.toml
├── apps/
│   ├── tool-1/            # Premier outil
│   │   ├── src/           # Frontend (HTML/CSS/JS)
│   │   ├── src-tauri/     # Backend Rust
│   │   └── package.json
│   └── tool-2/            # Deuxième outil (avec calculatrice)
│       ├── src/
│       ├── src-tauri/
│       └── package.json
└── README.md
```

## 🎯 Architecture

### Workspace Cargo
Le fichier `Cargo.toml` à la racine définit un **workspace** qui regroupe :
- `apps/tool-1/src-tauri` : Backend de Tool 1
- `apps/tool-2/src-tauri` : Backend de Tool 2  
- `common` : Bibliothèque Rust partagée

**Avantages :**
- ✅ Compilation séparée de chaque outil
- ✅ Code partagé dans `common/`
- ✅ Dépendances centralisées
- ✅ Optimisations globales en mode release

### Communication Frontend ↔️ Backend

```javascript
// Frontend (JavaScript)
const { invoke } = window.__TAURI__.core;
const result = await invoke("greet", { name: "John" });
```

```rust
// Backend (Rust)
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
```

## 🚀 Développement

### Prérequis
- [Node.js](https://nodejs.org/)
- [Rust](https://www.rust-lang.org/)
- [Tauri Prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites/)

### Lancer Tool 1

```powershell
cd apps\tool-1
npm install
npm run dev
```

### Lancer Tool 2

```powershell
cd apps\tool-2
npm install
npm run dev
```

## 📦 Compilation

### Compiler Tool 1 uniquement

```powershell
cd apps\tool-1
npm run build
```

L'exécutable sera dans `apps\tool-1\src-tauri\target\release\`

### Compiler Tool 2 uniquement

```powershell
cd apps\tool-2
npm run build
```

### Compiler tous les outils

```powershell
# À la racine
cargo build --release --workspace
```

## 🔧 Ajouter un Nouvel Outil

1. **Copier une app existante**
   ```powershell
   Copy-Item -Path "apps\tool-1" -Destination "apps\tool-3" -Recurse
   ```

2. **Modifier `Cargo.toml` à la racine**
   ```toml
   [workspace]
   members = [
       "apps/tool-1/src-tauri",
       "apps/tool-2/src-tauri",
       "apps/tool-3/src-tauri",  # Ajouter ici
       "common",
   ]
   ```

3. **Mettre à jour les fichiers de tool-3**
   - `apps/tool-3/src-tauri/Cargo.toml` : Changer `name = "tool-3"`
   - `apps/tool-3/package.json` : Changer `"name": "tool-3"`
   - `apps/tool-3/src-tauri/tauri.conf.json` : Mettre à jour `productName` et `identifier`
   - `apps/tool-3/src-tauri/src/lib.rs` : Implémenter votre logique

4. **Utiliser le code commun**
   ```rust
   use common::{AppConfig, format_message};
   
   #[tauri::command]
   fn ma_commande() -> String {
       let config = AppConfig::new("Tool 3", "0.1.0");
       format_message(&config.app_name, "Mon message")
   }
   ```

## 📚 Code Commun (`common/`)

La crate `common` contient du code Rust partagé :

```rust
// Disponible dans tous les outils
use common::{AppConfig, format_message};

let config = AppConfig::new("Mon Tool", "1.0.0");
let msg = format_message("INFO", "Application démarrée");
```

Pour ajouter du code commun, éditez `common/src/lib.rs`.

## 🎨 Différences entre Tool 1 et Tool 2

### Tool 1
- Commande `greet` : Salutation simple
- Commande `get_app_info` : Informations de l'app

### Tool 2  
- Commande `greet` : Salutation en français
- Commande `get_app_info` : Informations de l'app
- Commande `calculate` : Addition de deux nombres

## 🔍 Commandes Utiles

```powershell
# Lister tous les membres du workspace
cargo metadata --no-deps | Select-String "tool-"

# Compiler en debug
cd apps\tool-1
cargo build --manifest-path src-tauri\Cargo.toml

# Tests
cargo test --workspace

# Nettoyer les artefacts
cargo clean
```

## 📖 Ressources

- [Documentation Tauri](https://tauri.app/)
- [Cargo Workspaces](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html)
- [Tauri Commands](https://tauri.app/v1/guides/features/command/)

## 🐛 Dépannage

**Problème** : L'app ne se lance pas
- Vérifiez que Rust est installé : `rustc --version`
- Vérifiez que Node.js est installé : `node --version`
- Nettoyez et recompilez : `cargo clean && cargo build`

**Problème** : Erreur de dépendance
- Mettez à jour Cargo.lock : `cargo update`
- Supprimez `target/` et recompilez
