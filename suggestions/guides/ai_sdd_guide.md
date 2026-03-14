# Guide AI SDD

## Objectif

Définir une manière stable d'écrire des fiches lisibles par des humains et exploitables par une IA pour :

- décrire le métier
- guider l'implémentation
- générer ou vérifier des tests

## Principes

- Une fiche = un seul rôle :
  - entité
  - flux
  - tests d'intégration
  - bug
- Le métier doit être séparé des recommandations techniques.
- Les références entre fiches doivent être explicites et stables.
- Les règles doivent être formulées de façon testable.

## Langue

- Le contenu doit être rédigé majoritairement en français.
- Les termes métier canoniques peuvent rester en anglais, par exemple :
  - `Player`
  - `Award`
  - `increment_goals`
- Les identifiants techniques et les IDs de fiches restent courts et stables :
  - `ENTITY-PLAYER`
  - `FLOW-PLAYER-INCREMENT-GOALS`
  - `IT-F015`
  - `BUG-0001`

## Structure minimale par fiche

Chaque fiche commence par un front matter avec au minimum :

- `id`
- `title`
- `type`
- `status`

Et selon le type :

- entité : `main_table`, `components`, `related_flows`
- flux : `language`, `related_entities`, `related_tests`
- tests : `target_flow`, `naming_prefix`, `language`
- bug : `related_flow`, `related_tests`

## Types de fiches

### Fiche entité

Doit contenir :

- un résumé métier
- les attributs
- les composants
- les relations
- les invariants
- les règles métier
- des exemples

### Fiche flux

Doit contenir :

- le but métier
- les entrées
- les sorties
- les préconditions
- les postconditions
- les règles métier
- les erreurs métier
- des exemples

### Fiche de tests d'intégration

Doit contenir :

- l'objectif de couverture
- des cas nommés
- un découpage `Given / When / Then`
- les résultats attendus

### Fiche bug

Doit contenir :

- le symptôme
- le comportement attendu
- le comportement observé
- la cause racine si elle est connue
- la stratégie de correction

## Ce qui ne doit pas être dans les fiches métier

- des conventions générales de style de code
- des recommandations purement technologiques globales
- des renvois flous comme `relire tel fichier`

Ces informations doivent aller dans :

- un guide global
- ou un guide technologique spécialisé
