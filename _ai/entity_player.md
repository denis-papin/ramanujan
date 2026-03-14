# Player entity definition

- Ce fichier n'est pas modifiable par l'IA

## Tables
player (anciennement : person)
award

## Entity

- entity_name: Player
- main_table: player
- components:
  - award:
    - main_table: award


The player is an entity which has a name and a number of goals.
It owns a list of awards (components) which has year of delivery and an issuer (FIFA, FRANCE_FOOTBALL).

## Rules

- Every time a play increase it's number of goals, it might receive another award depending on the total of goals.
- An award must have been allocated for every pool of 3 goals.

