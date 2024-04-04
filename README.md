# MiniPNG
Une implémentation en Rust de MiniPNG !

# Informations générales:
* Toutes les questions ont été traitées. Le programme `q6` étant une amélioration de la question 1 et 2, il répond aux trois.
* L'implémentation de la question 3 se situe principalement dans src/implem/mini_png.rs, ligne 84.
* La question 4 correspond au programme `q4`.
* Le fichier correspondant à la question 5 se trouve dans le dossier `G` (fichier `G.mp`).
* Les questions 7 et 9 sont traitées par le programme `q9`.
* La question 8 est traitée par le programme `q8`.
* Tous les programmes se comportent comme prévus avec les exemples de fichiers MiniPNG distribués sur Moodle.

# Instructions d'installation et de lancement
* Installer la dernière version stable du compilateur Rust (recommandé via `rustup`: voir `https://www.rust-lang.org/tools/install` si rust(up) n'est pas installé sur votre machine. Si `rustup` est déjà installé mais que votre toolchain n'est pas à jour, effectuez la commande `rustup update`).
* Le visualiseur d'images nécessite l'installation des bibliothèques de développement de la SDL2. Voir https://github.com/Rust-SDL2/rust-sdl2#sdl20-development-libraries pour les instructions (Mac, Windows et Linux)
* Ouvrir un terminal à la racine du projet. Effectuer la commande `cargo build` pour compiler le projet en entier, puis `cargo run --bin <question> </chemin/du/fichier/mp>` pour lancer le programme `question`. Par exemple, pour `q9` avec le fichier `G.mp` fourni: `cargo run --bin q9 ./G/G.mp`.
* La liste des programmes disponibles est `q4`, `q6`, `q8`, `q9`.