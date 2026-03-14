# Tests Player::increment_goals

- Ce fichier n'est pas modifiable par l'IA
- le test sera écrit en Rust
- le flux testé est le F_015, utilise ce code dans le nom des tests
- Les tests fabriqués peuvent être en échecs : signaler les bugs.

## cas 1

- partir d'un joueur qui a 0 but et 0 récompense
- ajouter 1 but et vérifier qu'il a bien 0 récompenses.
- ajouter 1 but et vérifier qu'il a bien 0 récompenses.
- ajouter 1 but et vérifier qu'il a bien 1 récompenses.

## cas 21

- partir d'un joueur qui a 0 but et 0 récompense
- ajouter 10 fois un but et vérifier qu'il a bien 3 récompenses.

## cas 3

- partir d'un joueur qui a 24 buts et 0 récompense
- ajouter 1 but et vérifier qu'il a bien 8 récompenses.
- ajouter 2 fois 1 but et vérifier qu'il a bien 9 récompenses.

## cas 4

- ajouter 1 but à un joueur qui n'existe pas
- vérifier que le message d'erreur en retour

## cas 5

- ajouter 1 but à un joueur ID null
- vérifier que le message d'erreur en retour