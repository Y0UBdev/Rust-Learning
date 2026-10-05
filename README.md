# Roadmap de projets Rust — énoncés détaillés

## Niveau 1

### Projet 1 : Utilitaire de comptage de fichier
**Objectif** : reproduire le comportement d'un outil comme `wc`.
**Cahier des charges** :
- Le programme prend en argument le chemin d'un fichier texte.
- Il affiche le nombre de lignes, de mots et de caractères du fichier.
- Si le fichier n'existe pas ou n'est pas lisible, un message d'erreur clair doit s'afficher, sans faire planter le programme brutalement.
- Bonus : accepter plusieurs fichiers en argument et afficher un total.
- Bonus : ajouter des options du type `-l` (lignes seulement), `-w` (mots seulement).

### Projet 2 : Gestionnaire de tâches en ligne de commande
**Objectif** : une todo-list persistante entre les exécutions.
**Cahier des charges** :
- Commandes : ajouter une tâche, lister les tâches, marquer une tâche comme terminée, supprimer une tâche.
- Les tâches doivent être sauvegardées sur disque et rechargées à chaque lancement du programme.
- Chaque tâche a au minimum : un identifiant, un texte, un statut (fait/pas fait).
- Le programme doit gérer proprement les cas d'erreur (fichier de sauvegarde corrompu, identifiant inexistant, etc.) sans planter.
- Bonus : dates d'échéance, priorités, catégories/tags, tri de l'affichage.

## Niveau 2

### Projet 3 : Évaluateur d'expressions arithmétiques
**Objectif** : un programme qui prend une expression mathématique sous forme de texte (ex. `3 + 4 * (2 - 1)`) et renvoie le résultat.
**Cahier des charges** :
- Doit gérer les opérateurs `+`, `-`, `*`, `/` avec les priorités mathématiques standards.
- Doit gérer les parenthèses.
- Doit détecter et signaler les erreurs de syntaxe (expression malformée, parenthèse non fermée) sans planter.
- Doit détecter la division par zéro et la signaler proprement.
- Bonus : nombres décimaux, opérateur puissance, fonctions comme `sqrt` ou `abs`, variables (`x = 3; x + 2`).

### Projet 4 : Liste chaînée puis arbre binaire de recherche
**Objectif** : implémenter ces structures de données classiques soi-même, sans utiliser les équivalents de la bibliothèque standard.

**Partie A — liste chaînée simple** :
- Insertion en tête, insertion en fin, suppression d'un élément par valeur, recherche d'un élément.
- Affichage de tous les éléments dans l'ordre.
- Bonus : liste doublement chaînée, avec parcours dans les deux sens.

**Partie B — arbre binaire de recherche** :
- Insertion, recherche, suppression d'un nœud.
- Parcours en ordre (infixe), qui doit renvoyer les éléments triés.
- Calcul de la hauteur de l'arbre.
- Bonus : équilibrage automatique (AVL ou rouge-noir), suppression d'un nœud ayant deux enfants (cas le plus délicat).

### Projet 5 : Cache LRU (Least Recently Used)
**Objectif** : une structure de cache à capacité fixe qui évince l'élément le moins récemment utilisé quand elle est pleine.
**Cahier des charges** :
- Le cache a une capacité maximale définie à la création.
- Opération `get(clé)` : renvoie la valeur si elle existe, et la marque comme "récemment utilisée".
- Opération `put(clé, valeur)` : ajoute ou met à jour une entrée ; si le cache est plein, l'élément le moins récemment utilisé doit être supprimé avant l'ajout.
- Les deux opérations doivent être efficaces (pas de parcours linéaire de tout le cache à chaque appel, si possible).
- Bonus : expiration des entrées après un certain temps, statistiques (taux de succès/échec du cache).

## Niveau 3

### Projet 6 : Outil de recherche dans des fichiers (type grep)
**Objectif** : rechercher une chaîne de caractères dans un ou plusieurs fichiers.
**Cahier des charges** :
- Le programme prend un motif de recherche et un ou plusieurs fichiers/dossiers en argument.
- Il affiche les lignes contenant le motif, avec le numéro de ligne et le nom du fichier.
- Option pour une recherche insensible à la casse.
- Option pour une recherche récursive dans un dossier.
- Bonus : support des expressions régulières, mise en couleur du texte trouvé, option pour afficher le contexte (lignes avant/après).

### Projet 7 : Analyseur de fichier de données (logs ou CSV)
**Objectif** : lire un fichier structuré et en extraire des statistiques.
**Cahier des charges** :
- Lecture d'un fichier CSV ou d'un fichier de logs avec un format défini (ex. `[date] [niveau] message`).
- Extraction de statistiques : par exemple, nombre d'occurrences par catégorie, valeur moyenne d'une colonne numérique, entrées filtrées selon un critère.
- Le programme doit rester utilisable si certaines lignes sont malformées (les ignorer et le signaler, sans arrêter tout le traitement).
- Bonus : export du résultat dans un nouveau fichier, agrégation par intervalle de temps (nombre d'erreurs par heure, par exemple).

## Niveau 4

### Projet 8 : Hasher de fichiers en parallèle
**Objectif** : calculer l'empreinte (hash) de tous les fichiers d'un dossier, en exploitant plusieurs cœurs du processeur.
**Cahier des charges** :
- Parcours récursif d'un dossier donné en argument.
- Calcul d'un hash pour chaque fichier trouvé.
- Le calcul doit être réparti sur plusieurs threads pour accélérer le traitement sur de gros volumes.
- Affichage final : chemin du fichier + son hash, éventuellement triés.
- Bonus : détection de fichiers dupliqués (même hash), barre de progression, limitation du nombre de threads utilisés.

### Projet 9 : Pool de workers (thread pool)
**Objectif** : une structure qui exécute des tâches en parallèle en réutilisant un nombre fixe de threads, plutôt que d'en créer un par tâche.
**Cahier des charges** :
- Le pool est créé avec un nombre fixe de workers.
- On peut lui soumettre des tâches (des unités de travail quelconques) qui seront exécutées dès qu'un worker est disponible.
- Les tâches ne doivent jamais être perdues, même en cas de forte charge.
- Le pool doit pouvoir s'arrêter proprement (attendre que toutes les tâches en cours se terminent avant de fermer).
- Bonus : récupération du résultat de chaque tâche, gestion des tâches qui échouent (panique dans une tâche ne doit pas faire planter tout le pool).

### Projet 10 : Serveur de chat en réseau
**Objectif** : un serveur auquel plusieurs clients peuvent se connecter simultanément pour s'envoyer des messages.
**Cahier des charges** :
- Le serveur accepte plusieurs connexions simultanées.
- Un message envoyé par un client est diffusé à tous les autres clients connectés.
- Gestion propre de la déconnexion d'un client (le serveur ne doit pas planter).
- Un client minimal (en ligne de commande) pour se connecter et envoyer/recevoir des messages.
- Bonus : pseudonymes, salons de discussion séparés, historique des messages, messages privés.

## Niveau 5 — projets de plus grande envergure

### Projet 11 : API web
**Objectif** : une API REST pour gérer une ressource simple (ex. carnet d'adresses, liste de livres).
**Cahier des charges** :
- Routes pour créer, lire, mettre à jour et supprimer des entrées (CRUD complet).
- Les données doivent être persistées dans une base de données, pas seulement en mémoire.
- Validation des données envoyées par le client (types, champs obligatoires).
- Codes de statut HTTP appropriés selon le résultat de chaque opération.
- Bonus : authentification, pagination des résultats, filtrage/recherche via les paramètres de requête.

### Projet 12 : Base de données clé-valeur en réseau
**Objectif** : un serveur similaire à Redis dans son principe, accessible via le réseau.
**Cahier des charges** :
- Le serveur écoute sur un port et accepte des commandes simples : stocker une valeur sous une clé, récupérer une valeur, supprimer une clé.
- Les données doivent survivre à un redémarrage du serveur (persistance sur disque).
- Un client en ligne de commande pour envoyer des commandes au serveur.
- Bonus : expiration automatique des clés (durée de vie), types de données plus riches (listes, compteurs), réplication basique.

### Projet 13 : Interpréteur de langage de programmation
**Objectif** : un interpréteur pour un petit langage (variables, conditions, boucles, fonctions).
**Cahier des charges** :
- Le langage doit supporter au minimum : variables, opérations arithmétiques, conditions (`if`/`else`), boucles, déclaration et appel de fonctions.
- Le programme doit détecter les erreurs de syntaxe et les erreurs d'exécution (ex. variable non déclarée) avec des messages compréhensibles.
- Le programme doit pouvoir exécuter un fichier contenant du code dans ce langage.
- Bonus : un mode interactif (REPL), gestion des portées de variables imbriquées, structures de données (listes, tableaux associatifs).

### Projet 14 : Interface en mode texte (TUI)
**Objectif** : une application interactive dans le terminal, avec une interface graphique en mode texte.
**Cahier des charges** :
- Choisir un cas d'usage concret : tableau de bord système (CPU, mémoire), client pour l'un des projets précédents (chat, todo), lecteur de fichiers, etc.
- Navigation au clavier entre les différents éléments de l'interface.
- Mise à jour de l'affichage en temps réel si l'usage le justifie (ex. données système).
- L'interface doit rester utilisable et lisible même si la taille du terminal change.

### Projet 15 : Traceur de rayons (ray tracer)
**Objectif** : générer une image en simulant la trajectoire de rayons lumineux dans une scène 3D simple.
**Cahier des charges** :
- Modéliser une scène avec des objets simples (sphères au minimum), une caméra et une ou plusieurs sources de lumière.
- Calculer, pour chaque pixel de l'image, la couleur résultant de l'intersection des rayons avec les objets de la scène.
- Gérer au minimum les ombres et les reflets.
- Exporter le résultat sous forme d'image.
- Bonus : matériaux réfractifs (verre), anti-aliasing, scènes plus complexes avec des objets variés.

### Projet 16 : Petit jeu
**Objectif** : un jeu simple mais complet, avec une boucle de jeu, des entités et une interaction utilisateur.
**Cahier des charges** :
- Définir des règles de jeu claires et un objectif (victoire/défaite).
- Boucle de jeu qui met à jour l'état et rafraîchit l'affichage à intervalle régulier.
- Gestion des entrées utilisateur (clavier ou souris) pendant que le jeu tourne.
- Détection des collisions ou des interactions entre entités, si le jeu s'y prête.
- Bonus : plusieurs niveaux, système de score, sauvegarde de progression.

Si tu veux, dis-moi lequel tu comptes attaquer en premier et je peux le découper en étapes/jalons pour que tu saches par où commencer sans que je te donne de code.