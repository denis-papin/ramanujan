---
id: FLOW-PLAYER-DECREASE-GOALS
title: Player::decrease_goals
type: flow
status: draft
language: plsql
related_entities:
  - ENTITY-PLAYER
related_tests:
  - IT-F016
---

# But
Décrémenter de `1` le nombre de buts d'un `Player` et restaurer la cohérence des `Award`.

# Entrées
- `p_player_id`: `player.id`

# Sorties
- aucune

# Préconditions
- `p_player_id` ne doit pas être null
- le `Player` doit exister
- le `Player` doit avoir au moins `1` but

# Postconditions
- `player.goals` est décrémenté de `1`
- `award_count(player) = floor(player.goals / 3)`

# Règles métier
- Un `Award` est requis pour chaque palier de `3` buts.
- Si la décrémentation fait franchir un palier vers le bas, les `Award` en trop doivent être supprimés.
- L'émetteur d'un `Award` doit être `FIFA` ou `FRANCE_FOOTBALL`.
- L'année d'un `Award` doit être comprise entre `1980` et `2030`.

# Erreurs
- `ERR-PLAYER-ID-NULL`
  - code: `ORA-20001`
  - message: `p_player_id must not be null`
- `ERR-PLAYER-NOT-FOUND`
  - code: `ORA-20003`
  - message: `Player not found for id {player_id}`
- `ERR-PLAYER-GOALS-UNDERFLOW`
  - code: `ORA-20004`
  - message: `Player goals cannot be negative for id {player_id}`

# Exemples
- `player(1 goal, 0 award)` -> decrease -> `player(0 goal, 0 award)`
- `player(3 goals, 1 award)` -> decrease -> `player(2 goals, 0 award)`
- `player(25 goals, 8 awards)` -> decrease -> `player(24 goals, 8 awards)`
- `player(24 goals, 8 awards)` -> decrease -> `player(23 goals, 7 awards)`
