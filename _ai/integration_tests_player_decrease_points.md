---
id: IT-F016
title: Tests d'integration Player::decrease_points
type: integration-test
status: draft
target_flow: FLOW-PLAYER-DECREASE-POINTS
naming_prefix: IT-F016
language: rust
---

# Objectif de couverture
Valider le comportement métier et les messages d'erreur métier de `Player::decrease_points`.

# Cas de test

## TC-F016-001
Étant donné :
- player.points = 1
- player.awards = 0

Quand :
- decrease 1 time

Alors :
- points finaux = 0
- awards finaux = 0

## TC-F016-002
Étant donné :
- player.points = 3
- player.awards = 1

Quand :
- decrease 1 time

Alors :
- points finaux = 2
- awards finaux = 0

## TC-F016-003
Étant donné :
- player.points = 24
- player.awards = 8

Quand :
- decrease 1 time

Alors :
- points finaux = 23
- awards finaux = 7

## TC-F016-004
Étant donné :
- player id = -1

Quand :
- decrease 1 time

Alors :
- error code contains `ORA-20003`
- error message contains `Player not found for id -1`

## TC-F016-005
Étant donné :
- player id = null

Quand :
- decrease 1 time

Alors :
- error code contains `ORA-20001`
- error message contains `p_player_id must not be null`

## TC-F016-006
Étant donné :
- player.points = 0
- player.awards = 0

Quand :
- decrease 1 time

Alors :
- error code contains `ORA-20004`
- error message contains `Player points cannot be negative`
