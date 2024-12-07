## Tp Flux stochastiques – modèles du hasard - Nombres quasi-aléatoires - Parallélisme

### Comment faire tourner ce code

- Avoir Rust d'installer sur son ordi : https://www.rust-lang.org/tools/install
- Cloner le repo git, et faire la commande suivante dans un terminal :
`cargo run --release`
(la commande doit être faite dans le dossier clonné du git/le dossier parent à `src`)

C'est tout :)

Pour faire tourner le code en mode `debug`, il faut enlever le flag `--release`


Les dépendeances pour les librairies externes seront téléchargées par `cargo` lors du premier lancement du programme.

Tout ce qui peut être regénéré/recompilé est sauvegardé dans le dossier `target`. Ce dossier peut donc être supprimé (car souvent gros) et être regénérer plus tard si besoin. Les éxécutables, compilations incrémentales, rapport de [Criteron](https://github.com/bheisler/criterion.rs) sont dedans.

### Bibliothèque équivalente en Rust.

Voici un comparatif des librairies utilisées pour trouver une implémentation satisfaisante de Mersenne Twister:

| lien crate io | initaliser le générateur depuis entier | initaliser le générateur depuis tableau d'entier | sérialiser/désérialiser |
|---|---|---|---|
| [rand_mt](https://crates.io/crates/rand_mt)  | ✔️ | ✔️ | ❌ |
| [doryen-extra](https://github.com/ilyvion/doryen-extra)  | ✔️ | ❌ | ❌ |
| [vrd](https://crates.io/crates/vrd)  | ✔️ | ✔️ | ✔️ |


Le choix final a été de retenir la crate `vrd` afin de pouvoir sauvegarder l'état du générateur pour faire du parallélisme par la suite.

### Dépendances du projet


```
# voir les dépendances du projet
cargo tree

tp4 v0.1.0 (E:\Me\Rust\ZZ3\simu\tp4)
├── criterion v0.5.1
│   ├── anes v0.1.6
│   ├── cast v0.3.0
│   ├── ciborium v0.2.2
│   │   ├── ciborium-io v0.2.2
│   │   ├── ciborium-ll v0.2.2
│   │   │   ├── ciborium-io v0.2.2
│   │   │   └── half v2.4.1
│   │   │       └── cfg-if v1.0.0
│   │   └── serde v1.0.215
│   │       └── serde_derive v1.0.215 (proc-macro)
│   │           ├── proc-macro2 v1.0.92
│   │           │   └── unicode-ident v1.0.14     
│   │           ├── quote v1.0.37
│   │           │   └── proc-macro2 v1.0.92 (*)
│   │           └── syn v2.0.89
│   │               ├── proc-macro2 v1.0.92 (*)
│   │               ├── quote v1.0.37 (*)
│   │               └── unicode-ident v1.0.14
│   ├── clap v4.5.21
│   │   └── clap_builder v4.5.21
│   │       ├── anstyle v1.0.10
│   │       └── clap_lex v0.7.3
│   ├── criterion-plot v0.5.0
│   │   ├── cast v0.3.0
│   │   └── itertools v0.10.5
│   │       └── either v1.13.0
│   ├── is-terminal v0.4.13
│   │   └── windows-sys v0.52.0
│   │       └── windows-targets v0.52.6
│   │           └── windows_x86_64_msvc v0.52.6
│   ├── itertools v0.10.5 (*)
│   ├── num-traits v0.2.19
│   │   [build-dependencies]
│   │   └── autocfg v1.4.0
│   ├── once_cell v1.20.2
│   ├── oorandom v11.1.4
│   ├── plotters v0.3.7
│   │   ├── num-traits v0.2.19 (*)
│   │   ├── plotters-backend v0.3.7
│   │   └── plotters-svg v0.3.7
│   │       └── plotters-backend v0.3.7
│   ├── rayon v1.10.0
│   │   ├── either v1.13.0
│   │   └── rayon-core v1.12.1
│   │       ├── crossbeam-deque v0.8.5
│   │       │   ├── crossbeam-epoch v0.9.18
│   │       │   │   └── crossbeam-utils v0.8.20
│   │       │   └── crossbeam-utils v0.8.20
│   │       └── crossbeam-utils v0.8.20
│   ├── regex v1.11.1
│   │   ├── aho-corasick v1.1.3
│   │   │   └── memchr v2.7.4
│   │   ├── memchr v2.7.4
│   │   ├── regex-automata v0.4.9
│   │   │   ├── aho-corasick v1.1.3 (*)
│   │   │   ├── memchr v2.7.4
│   │   │   └── regex-syntax v0.8.5
│   │   └── regex-syntax v0.8.5
│   ├── serde v1.0.215 (*)
│   ├── serde_derive v1.0.215 (proc-macro) (*)
│   ├── serde_json v1.0.133
│   │   ├── itoa v1.0.13
│   │   ├── memchr v2.7.4
│   │   ├── ryu v1.0.18
│   │   └── serde v1.0.215 (*)
│   ├── tinytemplate v1.2.1
│   │   ├── serde v1.0.215 (*)
│   │   └── serde_json v1.0.133 (*)
│   └── walkdir v2.5.0
│       ├── same-file v1.0.6
│       │   └── winapi-util v0.1.9
│       │       └── windows-sys v0.52.0 (*)
│       └── winapi-util v0.1.9 (*)
├── rand v0.8.5
│   ├── rand_chacha v0.3.1
│   │   ├── ppv-lite86 v0.2.20
│   │   │   └── zerocopy v0.7.35
│   │   │       ├── byteorder v1.5.0
│   │   │       └── zerocopy-derive v0.7.35 (proc-macro)
│   │   │           ├── proc-macro2 v1.0.92 (*)
│   │   │           ├── quote v1.0.37 (*)
│   │   │           └── syn v2.0.89 (*)
│   │   └── rand_core v0.6.4
│   │       └── getrandom v0.2.15
│   │           └── cfg-if v1.0.0
│   └── rand_core v0.6.4 (*)
├── rayon v1.10.0 (*)
├── ron v0.8.1
│   ├── base64 v0.21.7
│   ├── bitflags v2.6.0
│   │   └── serde v1.0.215 (*)
│   ├── serde v1.0.215 (*)
│   └── serde_derive v1.0.215 (proc-macro) (*)
├── serde v1.0.215 (*)
├── serde-binary v0.5.0
│   ├── binary-stream v3.4.0
│   │   └── thiserror v1.0.69
│   │       └── thiserror-impl v1.0.69 (proc-macro)
│   │           ├── proc-macro2 v1.0.92 (*)
│   │           ├── quote v1.0.37 (*)
│   │           └── syn v2.0.89 (*)
│   ├── serde v1.0.215 (*)
│   └── thiserror v1.0.69 (*)
└── vrd v0.0.8
    ├── bitflags v2.6.0 (*)
    ├── dtt v0.0.6
    │   ├── regex v1.11.1 (*)
    │   ├── serde v1.0.215 (*)
    │   └── time v0.3.36
    │       ├── deranged v0.3.11
    │       │   └── powerfmt v0.2.0
    │       ├── num-conv v0.1.0
    │       ├── powerfmt v0.2.0
    │       └── time-core v0.1.2
    ├── rand v0.8.5 (*)
    ├── rlg v0.0.4
    │   ├── dtt v0.0.6 (*)
    │   ├── hostname v0.4.0
    │   │   ├── cfg-if v1.0.0
    │   │   └── windows v0.52.0
    │   │       ├── windows-core v0.52.0
    │   │       │   └── windows-targets v0.52.6 (*)
    │   │       └── windows-targets v0.52.6 (*)
    │   ├── serde v1.0.215 (*)
    │   ├── serde_json v1.0.133 (*)
    │   ├── tokio v1.41.1
    │   │   ├── bytes v1.8.0
    │   │   ├── mio v1.0.2
    │   │   │   └── windows-sys v0.52.0 (*)
    │   │   ├── parking_lot v0.12.3
    │   │   │   ├── lock_api v0.4.12
    │   │   │   │   └── scopeguard v1.2.0
    │   │   │   │   [build-dependencies]
    │   │   │   │   └── autocfg v1.4.0
    │   │   │   └── parking_lot_core v0.9.10
    │   │   │       ├── cfg-if v1.0.0
    │   │   │       ├── smallvec v1.13.2
    │   │   │       └── windows-targets v0.52.6 (*)
    │   │   ├── pin-project-lite v0.2.15
    │   │   ├── socket2 v0.5.7
    │   │   │   └── windows-sys v0.52.0 (*)
    │   │   ├── tokio-macros v2.4.0 (proc-macro)
    │   │   │   ├── proc-macro2 v1.0.92 (*)
    │   │   │   ├── quote v1.0.37 (*)
    │   │   │   └── syn v2.0.89 (*)
    │   │   └── windows-sys v0.52.0 (*)
    │   └── vrd v0.0.7
    │       ├── bitflags v2.6.0 (*)
    │       ├── dtt v0.0.5
    │       │   ├── regex v1.11.1 (*)
    │       │   ├── serde v1.0.215 (*)
    │       │   └── time v0.3.36 (*)
    │       ├── rand v0.8.5 (*)
    │       ├── rlg v0.0.3
    │       │   ├── dtt v0.0.5 (*)
    │       │   ├── hostname v0.3.1
    │       │   │   ├── match_cfg v0.1.0
    │       │   │   └── winapi v0.3.9
    │       │   ├── serde_json v1.0.133 (*)
    │       │   ├── tokio v1.41.1 (*)
    │       │   └── vrd v0.0.5
    │       │       ├── rand v0.8.5 (*)
    │       │       └── serde v1.0.215 (*)
    │       ├── serde v1.0.215 (*)
    │       ├── serde-big-array v0.5.1
    │       │   └── serde v1.0.215 (*)
    │       ├── serde_json v1.0.133 (*)
    │       ├── tokio v1.41.1 (*)
    │       └── uuid v1.11.0
    │           └── getrandom v0.2.15 (*)
    │   [build-dependencies]
    │   └── version_check v0.9.5
    ├── serde v1.0.215 (*)
    ├── serde-big-array v0.5.1 (*)
    ├── serde_json v1.0.133 (*)
    ├── tokio v1.41.1 (*)
    └── uuid v1.11.0 (*)
    [build-dependencies]
    └── version_check v0.9.5
```