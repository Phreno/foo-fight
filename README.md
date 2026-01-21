# Foo Fight

Foo Fight est une application CLI (Command Line Interface) avec interface TUI (Text User Interface) pour l'entraînement au "speed typing" de commandes. Inspiré de ShortcutFoo, cet outil vous aide à mémoriser et à taper rapidement les commandes de différents outils (Git, Jujutsu, PowerShell, etc.).

## Fonctionnalités

- **Interface TUI interactive** avec ratatui pour une expérience utilisateur agréable
- **Système de dictionnaires** modulaires et extensibles (format TOML)
- **Validation des réponses** avec support des alias de commandes
- **Statistiques en temps réel** : progression, taux de réussite, série (streak)
- **Feedback immédiat** avec possibilité de réessayer ou passer
- **Compatible Windows, Linux et macOS**

## Installation et lancement

### Prérequis
- Rust stable (version 2021 edition ou ultérieure)

### Compiler et exécuter

```bash
# Compiler le projet
cargo build --release

# Exécuter directement
cargo run

# Ou exécuter le binaire compilé
./target/release/foo-fight
```

### Exécuter les tests

```bash
cargo test
```

## Utilisation

### Écran de sélection
Au lancement, vous verrez la liste des dictionnaires disponibles :
- **↑/↓** : Naviguer dans la liste
- **Enter** : Sélectionner un dictionnaire
- **Esc/q** : Quitter l'application

### Écran d'entraînement
Une fois un dictionnaire sélectionné :
- **Tapez** votre réponse dans le champ de saisie
- **Enter** : Valider votre réponse
- **Backspace** : Effacer un caractère
- **Ctrl+C/Esc** : Retour à la sélection
- **[R]** : Réessayer (après une réponse incorrecte)
- **[S]** : Passer à la question suivante (après une réponse incorrecte)

L'application affiche en temps réel :
- Votre progression (question actuelle / total)
- Nombre de réponses correctes
- Votre série de bonnes réponses
- Votre taux de réussite

## Ajouter un dictionnaire

Les dictionnaires sont stockés dans le dossier `dictionaries/` au format TOML.

### Format TOML

Créez un fichier `.toml` dans le dossier `dictionaries/` :

```toml
name = "Nom du dictionnaire"
version = 1
language = "fr"

[[items]]
id = "identifiant_unique"
prompt = "Question ou description de la commande"
answer = "commande attendue"
aliases = ["alias1", "alias2"]  # Optionnel : réponses alternatives acceptées
tags = ["tag1", "tag2"]         # Optionnel : pour filtrage futur
difficulty = 1                  # Optionnel : niveau de difficulté (1-3)
```

### Exemple

```toml
name = "Git - Commandes de base"
version = 1
language = "fr"

[[items]]
id = "git_status"
prompt = "Afficher l'état du dépôt"
answer = "git status"
aliases = ["git st"]
tags = ["git", "basics"]
difficulty = 1

[[items]]
id = "git_add"
prompt = "Ajouter tous les fichiers à l'index"
answer = "git add ."
aliases = ["git add -A"]
tags = ["git", "staging"]
difficulty = 1
```

## Dictionnaires inclus

Trois dictionnaires sont fournis en exemple :
- **Git - Bases** : 12 commandes Git essentielles
- **Jujutsu** : 12 commandes pour le système de contrôle de version Jujutsu
- **PowerShell & Pester** : 12 commandes PowerShell et framework de test Pester

## Architecture du code

```
src/
├── main.rs         # Point d'entrée, boucle événementielle
├── app.rs          # State machine de l'application
├── dict.rs         # Parsing et validation des dictionnaires
├── engine.rs       # Logique d'entraînement et statistiques
└── ui.rs           # Rendu de l'interface TUI avec ratatui

dictionaries/       # Dictionnaires au format TOML
├── git.toml
├── jujutsu.toml
└── powershell_pester.toml
```

## Dépendances principales

- **ratatui** : Framework TUI pour l'interface
- **crossterm** : Backend terminal cross-platform
- **serde** + **toml** : Parsing des fichiers de configuration
- **anyhow** : Gestion des erreurs
- **rand** : Mélange aléatoire des questions

## Develop in Codespaces

Ce projet est configuré pour une utilisation immédiate dans GitHub Codespaces. L'environnement de développement est automatiquement configuré avec :

- Rust stable avec `rustfmt` et `clippy`
- Extensions VS Code : rust-analyzer, Even Better TOML, crates
- Configuration automatique : formatage à la sauvegarde, inlay hints activés
- Dépendances pré-téléchargées au démarrage

Pour commencer :
1. Cliquez sur "Code" → "Codespaces" → "Create codespace on master"
2. Attendez que l'environnement se charge (quelques secondes)
3. Exécutez `cargo build`, `cargo test`, ou `cargo run` directement

## Release Process

Les releases sont automatiquement créées lors d'un push sur la branche `master`. Le processus :

1. **CI automatique** : À chaque push, les workflows vérifient le formatage (`cargo fmt`), exécutent `clippy` et les tests
2. **Build multi-plateformes** : Génération des binaires pour Linux x86_64, Windows x86_64, macOS x86_64 et aarch64
3. **Publication** : Création automatique d'une release GitHub avec les artefacts nommés `foo-fight-<version>-<target>.(tar.gz|zip)`

**Pour publier une nouvelle version :**
- Mettez à jour le champ `version` dans `Cargo.toml`
- Committez et pushez sur `master`
- La release sera créée automatiquement avec les notes de changelog

Les releases sont accessibles sur la page [Releases](https://github.com/Phreno/foo-fight/releases) du projet.

## Licence

Ce projet est fourni tel quel pour un usage éducatif et personnel.
 
