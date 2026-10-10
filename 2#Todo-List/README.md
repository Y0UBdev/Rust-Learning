# Projet 2 : Gestionnaire de tâches en ligne de commande
**Objectif** : une todo-list persistante entre les exécutions.

**Cahier des charges** :
- Commandes : ajouter une tâche, lister les tâches, marquer une tâche comme terminée, supprimer une tâche.
- Les tâches doivent être sauvegardées sur disque et rechargées à chaque lancement du programme.
- Chaque tâche a au minimum : un identifiant, un texte, un statut (fait/pas fait).
- Le programme doit gérer proprement les cas d'erreur (fichier de sauvegarde corrompu, identifiant inexistant, etc.) sans planter.

## Fonctionnalités attendues

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
cargo run 
```

## Notes

Bonus : dates d'échéance, priorités, catégories/tags, tri de l'affichage.
