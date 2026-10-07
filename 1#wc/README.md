# Projet #1 : Utilitaire de comptage de fichiers

## Objectif

Reproduire le comportement d'un outil de type `wc`, en Rust, pour compter les lignes, les mots et les caractères d'un ou plusieurs fichiers texte.

## Description

Ce projet consiste à développer un petit utilitaire en ligne de commande capable de lire un fichier et d'afficher ses statistiques de manière claire et robuste.

## Fonctionnalités attendues

- Prendre en argument le chemin d'un fichier texte
- Afficher le nombre de lignes, de mots et de caractères
- Gérer proprement les erreurs de lecture (fichier introuvable, inaccessible, etc.)
- Accepter plusieurs fichiers en argument, avec un total final optionnel
- Supporter des options de filtrage comme :
  - `-l` : lignes seulement
  - `-w` : mots seulement

## Prérequis

- Rust installé sur votre machine
- Un terminal capable d'exécuter les commandes `cargo`

## Lancement

### Compiler le projet

```bash
cargo build
```

### Exécuter le programme

```bash
cargo run -- fichier.txt
cargo run -- -l fichier.txt
cargo run -- -w fichier.txt
cargo run -- -l -w fichier.txt
```

## Notes

Le but n'est pas seulement d'obtenir un résultat fonctionnel, mais aussi d'écrire du code lisible, robuste et facile à maintenir, avec une gestion propre des cas limites.
