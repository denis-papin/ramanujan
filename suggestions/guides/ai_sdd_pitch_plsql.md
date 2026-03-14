# Vendre l'approche AI SDD à une équipe PL/SQL

## Idée

L'AI SDD, pour *AI Spec-Driven Development*, consiste à faire des fiches courtes, structurées et traçables pour décrire :

- les entités métier
- les flux métier
- les tests d'intégration
- les bugs observés

L'objectif n'est pas de produire plus de documentation. L'objectif est de produire une documentation assez claire pour être relue par un humain, exploitée par une IA, et transformée rapidement en code, tests et correctifs fiables.

## Pourquoi c'est utile en PL/SQL

Dans un projet PL/SQL, les bugs viennent souvent de zones grises :

- règles métier implicites dans un package body
- effets de bord sur plusieurs tables
- messages d'erreur non homogènes
- cas limites mal couverts
- écart entre le script SQL source et ce qui est réellement compilé en base

L'AI SDD réduit ces zones grises en posant avant le code :

- ce que le flux doit faire
- ce qu'il ne doit jamais faire
- quelles erreurs métier il doit lever
- quels tests doivent prouver le comportement

## Bénéfices concrets pour corriger des bugs

### 1. On corrige un bug métier, pas juste une ligne de code

Avec une fiche de bug reliée à un flux et à des tests, on évite les correctifs locaux du type "ça marche pour mon cas".  
Le bug est exprimé avec :

- le symptôme
- le comportement attendu
- la règle métier violée
- les cas de tests qui doivent passer après correction

En PL/SQL, c'est particulièrement utile quand un package modifie plusieurs objets en base.

### 2. Les erreurs Oracle deviennent des erreurs métier explicites

Sans spec, on finit vite avec des erreurs techniques comme :

- `ORA-06502`
- `NO_DATA_FOUND`
- comportements différents selon le chemin d'exécution

Avec AI SDD, la fiche du flux impose les erreurs attendues :

- `ORA-20001` si l'identifiant est `NULL`
- `ORA-20003` si le `Player` n'existe pas
- `ORA-20004` si on tente de décrémenter sous zéro

Le gain est immédiat :

- comportement plus lisible
- tests plus stables
- debugging plus rapide

### 3. Les TUs et tests d'intégration ne sont plus inventés au fil de l'eau

Les tests viennent des fiches, pas de l'inspiration du moment.  
On couvre plus facilement :

- le nominal
- les seuils métier
- les incohérences de données
- les cas d'erreur

Sur un package PL/SQL, cela évite les trous classiques : "le flux marche à +1 mais pas à +10", "le message d'erreur n'est pas le bon", "les données dérivées ne sont pas réalignées".

### 4. L'IA aide mieux quand la spec est stable

Une IA produit de meilleurs correctifs si elle dispose de :

- l'entité concernée
- le flux concerné
- les invariants métier
- les tests attendus

Sans cela, elle improvise.  
Avec cela, elle peut :

- proposer un correctif cohérent
- ajouter les tests manquants
- signaler les incohérences de spec
- limiter les régressions

### 5. On garde une trace claire des décisions

Quand un bug est corrigé, on sait :

- quelle règle métier a été clarifiée
- quel flux a été modifié
- quels tests protègent désormais le comportement

Cela aide beaucoup dans un contexte PL/SQL où une partie importante de la logique vit en base et se relit moins facilement qu'un service applicatif classique.

## Exemple simple

Sans AI SDD :

- un bug est signalé sur `Player::decrease_goals`
- un développeur modifie la procédure
- un autre découvre ensuite qu'il fallait aussi supprimer des `Award`
- un test technique passe, mais la règle métier reste incomplète

Avec AI SDD :

1. la fiche `entity_player.md` rappelle l'invariant :  
   `award_count(player) = floor(goals / 3)`
2. la fiche `flow_player_decrease_goals.md` précise le comportement attendu
3. la fiche `integration_tests_player_decrease_goals.md` liste les cas à couvrir
4. le correctif PL/SQL est guidé par ces éléments
5. les tests protègent le comportement après livraison

## Ce que l'équipe y gagne

- moins d'ambiguïtés métier
- moins de corrections partielles
- des packages PL/SQL plus prédictibles
- des messages d'erreur métier homogènes
- une meilleure collaboration entre humains et IA
- une base documentaire utile, pas décorative

## Ce que cela ne veut pas dire

AI SDD ne remplace pas :

- la revue de code
- la compréhension métier
- les tests exécutés sur une vraie base Oracle

En revanche, AI SDD donne un cadre qui rend ces activités plus rapides et plus fiables.

## Proposition d'adoption simple

Commencer petit, sur un flux à la fois :

1. une fiche entité
2. une fiche flux
3. une fiche de tests d'intégration
4. une fiche bug si un défaut existe déjà

Sur un projet PL/SQL, ce niveau de discipline suffit déjà à améliorer fortement la qualité des correctifs.
