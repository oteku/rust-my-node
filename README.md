# Rust My Node

Ce projet est un exemple d'utilisation de [Neon Bindings](https://neon-bindings.com/), servant de support à la présentation `Rust My Node` présentée lors de Lille FP 14

- [Slides](https://fr.slideshare.net/ThomasHaessle/rust-my-node)
- Suivre les prochains [LilleFP](https://www.meetup.com/fr-FR/Lille-FP/)

## Run exemple

Prérequis :

- Installer [node](https://nodejs.org/en/)
- Installer [Rust](https://www.rust-lang.org/tools/install)
- Installer [Neon](https://neon-bindings.com/docs/getting-started)

- Cloner le repository
- Installer les dépendances node `npm i`
- Compiler l'addon natif `neon build`

Exécuter les exemples :

- `node hello.js`
- `node fibo-sync.js --value 1`
- `node fibo-sync.js --value 45`
- `node fibo-sync.js --value 50`
- `node fibo-async.js --value 45`
- `node foldersize.js`
