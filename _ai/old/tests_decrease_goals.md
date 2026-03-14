# Tests Player::decrease_points

- Ce fichier n'est pas modifiable par l'IA
- le test sera écrit en Rust
- le flux testé est le F_016, utilise ce code dans le nom des tests
- Les tests fabriqués peuvent être en échecs : signaler les bugs.

## cas 1

- partir d'un joueur qui a 1 but et 0 récompense
- enlever 1 but et vérifier qu'il a bien 0 récompense.

## cas 2

- partir d'un joueur qui a 3 buts et 1 récompense
- enlever 1 but et vérifier qu'il a bien 0 récompense.

## cas 3

- partir d'un joueur qui a 24 buts et 8 récompenses
- enlever 1 but et vérifier qu'il a bien 7 récompenses.

## cas 4

- enlever 1 but à un joueur qui n'existe pas
- vérifier que le message d'erreur en retour

## cas 5

- enlever 1 but à un joueur ID null
- vérifier que le message d'erreur en retour

## cas 6

- partir d'un joueur qui a 0 but et 0 récompense
- enlever 1 but
- vérifier que le message d'erreur en retour
