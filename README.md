# RustAgent — Assistant de codage

> ⚠️ **Statut : En cours de développement (non abouti / incomplet)**

Ce projet est actuellement **en cours de développement actif et n'est pas
encore dans un état final et fonctionnel**. Le code de ce dépôt représente un
prototype précoce d'un assistant de codage de bureau. Plusieurs fonctionnalités
sont partiellement implémentées, peuvent ne pas compiler proprement et n'ont
pas été entièrement testées ou validées. Traitez tout ce qui se trouve ici comme
expérimental.

---

## Présentation

**RustAgent** est une application de bureau native qui combine un assistant de
chat IA, un éditeur de code avec coloration syntaxique et un terminal intégré
dans une seule fenêtre redimensionnable. Il est écrit en **Rust** et repose sur
le framework GUI [**Freya**](https://github.com/marc2332/freya).

L'idée est simple : vous demandez à l'assistant (en langage naturel) d'écrire
du code, il génère le code et le place directement dans l'éditeur, puis vous
pouvez l'exécuter dans le terminal intégré en un seul clic.

<img width="1292" height="701" alt="image" src="https://github.com/user-attachments/assets/8084ab30-cc48-4ba3-9f7d-904a774517b0" />

### Fonctionnalités prévues

- 💬 **Panneau de chat IA** — discuter avec un LLM adossé à l'API **Albert** du
  gouvernement français (point de terminaison compatible OpenAI).
- ✍️ **Éditeur de code** — un éditeur propulsé par tree-sitter avec coloration
  syntaxique et thème sombre.
- 🖥️ **Terminal intégré** — un terminal `bash` complet rendu dans l'application.
- 🚀 **Exécuter le code** — exécuter ce qui se trouve dans l'éditeur dans le
  terminal.
- 🌐 **Prise en charge multi-langages** — Python, Rust, JavaScript, TypeScript,
  HTML, CSS, C, C++, Java et Go.

---

## État actuel / Ce qui n'est PAS abouti

Parce qu'il s'agit d'un projet inachevé, veuillez prendre connaissance des
limitations suivantes avant d'essayer de le compiler ou de l'exécuter :

- **Compilation non garantie.** Le code repose sur un ensemble spécifique de
  dépendances d'espace de travail et de fonctionnalités Freya qui peuvent ne
  pas se résoudre dans un checkout autonome. Il n'y a ni `Cargo.lock` ni racine
  d'espace de travail validés.
- **Aucune gestion de clé API.** ~~L'application lit `ALBERT_API_KEY` depuis
  l'environnement, mais il n'y a aucune configuration, validation ou
  intégration progressive.~~ *(Résolu — voir la section Configuration : la clé
  peut être fournie via l'environnement, un fichier de configuration, ou le
  panneau Paramètres, avec validation au démarrage, gestion des erreurs,
  nouvelle tentative et délai d'attente.)*
- **Hypothèses de plateforme.** Le terminal lance `bash` et plusieurs commandes
  d'exécution utilisent des chemins Unix (`/tmp/...`, `xdg-open`, `python3`,
  `gcc`, `g++`, `javac`, `go`, `node`, `npx`). Celles-ci ne fonctionneront pas
  sur Windows sans adaptation.
- **Interface non testée.** Les panneaux de chat, d'éditeur et de terminal sont
  reliés entre eux mais n'ont pas été exercés de bout en bout.
- **Aucun test, CI ou packaging.** Il n'y a pas de suite de tests, de pipeline
  de construction ni de configuration de publication.

En bref : il s'agit d'un **prototype / preuve de concept**, pas d'un produit
prêt à être distribué.

---

## Structure du projet

```
coding-assistant/
├── Cargo.toml          # Manifeste du paquet et dépendances
├── src/
│   └── main.rs         # Application entière (prototype en un seul fichier)
└── agent_memory.json   # Mémoire de session de l'agent (ne fait pas partie de l'app)
```

Toute l'application vit actuellement dans un seul fichier `src/main.rs`.

---

## Pour commencer

### Prérequis

- Une chaîne d'outils **Rust** récente (le manifeste cible `edition = "2024"`).
- Le framework **Freya** et ses dépendances natives (voir la
  [documentation Freya](https://github.com/marc2332/freya) pour la configuration
  de la plateforme).
- Une **clé API Albert** (service IA du gouvernement français) exportée sous le
  nom `ALBERT_API_KEY`.
- Les environnements d'exécution des langages que vous souhaitez exécuter
  (Python, Node, GCC/G++, JDK, Go, Rust, etc.).

### Compilation et exécution

```bash
# Compiler le projet
cargo build

# Exécuter l'application
cargo run
```

> ⚠️ Comme indiqué ci-dessus, une compilation propre **n'est actuellement pas
> garantie** en raison de l'état inachevé du projet.

---

## Comment cela fonctionne (comportement prévu)

1. **Chat** — Saisissez un message dans la zone de saisie et appuyez sur
   **Envoyer**. Le message est envoyé à l'API Albert via `rig-core`.
2. **Détection du langage** — L'application devine le langage de programmation
   à partir de votre message (par ex. « écris une fonction Python » → Python).
3. **Génération de code** — Si votre message demande du code, la réponse de
   l'assistant est analysée pour en extraire les blocs de code, qui sont insérés
   dans l'éditeur. Le chat n'affiche qu'une confirmation, pas le code lui-même.
4. **Exécution** — Appuyez sur **Exécuter le code** pour écrire le contenu de
   l'éditeur dans un fichier temporaire et l'exécuter dans le terminal intégré
   à l'aide de la commande du langage détecté.
5. **Effacer l'éditeur** — Saisir « clear editor » (ou une formulation
   équivalente) vide l'éditeur localement sans appeler l'IA.

### Actions de la barre d'outils

| Bouton              | Action                                          |
| ------------------- | ----------------------------------------------- |
| **Paramètres**      | Ouvre le panneau de configuration de la clé API. |
| **Effacer le chat** | Vide l'historique du chat.                      |
| **Réinitialiser le terminal** | Tue et relance le shell `bash` intégré. |
| **Exécuter le code**| Exécute le contenu de l'éditeur dans le terminal. |

---

## Langages pris en charge

| Langage    | Extension | Commande d'exécution (Unix)                     |
| ---------- | --------- | ----------------------------------------------- |
| Python     | `.py`     | `python3 <fichier>`                             |
| Rust       | `.rs`     | `rustc <fichier> -o /tmp/main_rs && /tmp/main_rs` |
| JavaScript | `.js`     | `node <fichier>`                                |
| TypeScript | `.ts`     | `npx ts-node <fichier>`                         |
| HTML       | `.html`   | `xdg-open <fichier>`                            |
| CSS        | `.css`    | *(rien à exécuter)*                             |
| C          | `.c`      | `gcc <fichier> -o /tmp/main_c && /tmp/main_c`   |
| C++        | `.cpp`    | `g++ <fichier> -o /tmp/main_cpp && /tmp/main_cpp` |
| Java       | `.java`   | `javac <fichier> && java Main`                  |
| Go         | `.go`     | `go run <fichier>`                              |

---

## Configuration

Le point de terminaison IA et le modèle sont codés en dur en haut de
`src/main.rs` :

```rust
const ALBERT_ENDPOINT: &str = "https://albert.api.etalab.gouv.fr/v1";
const ALBERT_MODEL: &str = "deepseek-v4-flash";
```

### Clé API

La clé API peut être fournie par trois canaux, par ordre de priorité :

1. **Variable d'environnement** `ALBERT_API_KEY` (priorité la plus élevée).
2. **Fichier de configuration** dans le répertoire de configuration de la
   plateforme :
   - Linux : `~/.config/rustagent/config.toml`
   - macOS : `~/Library/Application Support/rustagent/config.toml`
   - Windows : `%APPDATA%\rustagent\config.toml`
3. **Panneau Paramètres** — le bouton **Paramètres** de la barre d'outils ouvre
   un panneau où vous pouvez saisir et enregistrer la clé dans le fichier de
   configuration.

Au démarrage, l'application valide la clé et affiche un avertissement clair si
elle est manquante ou invalide. Les erreurs d'API sont classées (authentification,
limite de débit, réseau, délai d'attente, modèle) avec des messages adaptés, et
les échecs transitoires sont automatiquement réessayés avec un backoff
progressif. Chaque requête dispose d'un délai d'attente de 60 secondes pour que
l'interface ne se bloque jamais indéfiniment.

---

## Dépendances

Principales crates utilisées :

- **freya** — framework GUI (avec les fonctionnalités `markdown`, `terminal`,
  `remote-asset` et `code-editor`).
- **rig-core** — bibliothèque cliente LLM (fournisseur compatible OpenAI).
- **tokio** — runtime asynchrone.
- **ropey** — structure de données rope qui sous-tend l'éditeur.
- **tree-sitter-\*** — grammaires et requêtes de coloration pour chaque langage.
- **rio-vt** — émulation de terminal virtuel.
- **serde / serde_json** — sérialisation.
- **thiserror** — types d'erreur.

---

## Feuille de route (ce qu'il reste à faire)

- [x] Faire compiler proprement le projet dans un checkout autonome.
- [x] Ajouter une racine d'espace de travail / `Cargo.lock` et épingler les
      versions des dépendances.
- [x] Prise en charge multiplateforme du terminal et des commandes d'exécution
      (Windows/macOS).
- [x] Configuration correcte de la clé API et gestion des erreurs.
- [ ] Tests et CI.
- [ ] Packaging / builds de publication.
- [ ] Validation de bout en bout du flux chat → éditeur → terminal.

---

## Licence

Ce projet est distribué sous la licence **MIT**.

Copyright (c) 2026 **Salvatore DI DiO**

Voir le fichier [LICENSE](LICENSE) pour le texte complet de la licence.

---

## Avertissement

Il s'agit d'un projet **inachevé**. Il est fourni à titre de référence et
d'expérimentation uniquement. Ne vous y fiez pas pour une utilisation en
production.

---

## Remerciements

Ce code a été développé avec l'assistance de **DeepseekFlash**, un modèle
fourni via l'**API Albert** de la [DINUM](https://www.numerique.gouv.fr/numerique-etat/dinum/) et [CsAgent](https://github.com/artydev/csagent)
