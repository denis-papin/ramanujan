---
id: IT-F015
title: Tests d'integration Player::increment_goals
type: integration-test
status: active
target_flow: FLOW-PLAYER-INCREMENT-GOALS
naming_prefix: f_015
language: rust
---

# Objectif de couverture
Valider le comportement métier et les messages d'erreur métier de `Player::increment_goals`.

# Cas de test

## TC-F015-001
Étant donné :
- player.goals = 0
- player.awards = 0

Quand :
- increment 1 time
- increment 1 time
- increment 1 time

Alors :
- après l'appel 1 : goals = 1, awards = 0
- après l'appel 2 : goals = 2, awards = 0
- après l'appel 3 : goals = 3, awards = 1

## TC-F015-002
Étant donné :
- player.goals = 0
- player.awards = 0

Quand :
- increment 10 times

Alors :
- goals finaux = 10
- awards finaux = 3

## TC-F015-003
Étant donné :
- player.goals = 24
- player.awards = 0

Quand :
- increment 1 time
- increment 2 more times

Alors :
- après l'appel 1 : goals = 25, awards = 8
- après l'appel 3 : goals = 27, awards = 9

## TC-F015-004
Étant donné :
- player id = -1

Quand :
- increment 1 time

Alors :
- error code contains `ORA-20003`
- error message contains `Player not found for id -1`

## TC-F015-005
Étant donné :
- player id = null

Quand :
- increment 1 time

Alors :
- error code contains `ORA-20001`
- error message contains `p_player_id must not be null`
