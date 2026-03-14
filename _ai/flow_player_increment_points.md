---
id: FLOW-PLAYER-INCREMENT-POINTS
title: Player::increment_points
type: flow
status: active
language: plsql
related_entities:
  - ENTITY-PLAYER
related_tests:
  - IT-F015
  - IT-F016
---

# But
Incrémenter de `1` le nombre de points d'un `Player` et restaurer la cohérence des `Award`.

# Entrées
- `p_player_id`: `player.id`

# Sorties
- aucune

# Préconditions
- `p_player_id` ne doit pas être null
- le `Player` doit exister

# Postconditions
- `player.points` est incrémenté de `1`
- `award_count(player) = floor(player.points / 3)`

# Règles métier
- Un `Award` est requis pour chaque palier de `3` points.
- L'émetteur d'un `Award` doit être `GOLD` ou `SILVER`.
- L'année d'un `Award` doit être comprise entre `1980` et `2030`.

# Erreurs
- `ERR-PLAYER-ID-NULL`
  - code: `ORA-20001`
  - message: `p_player_id must not be null`
- `ERR-PLAYER-NOT-FOUND`
  - code: `ORA-20003`
  - message: `Player not found for id {player_id}`

# Exemples
- `player(0 points, 0 awards)` -> increment -> `player(1 point, 0 award)`
- `player(2 points, 0 awards)` -> increment -> `player(3 points, 1 award)`
- `player(24 points, 0 awards)` -> increment -> `player(25 points, 8 awards)`
