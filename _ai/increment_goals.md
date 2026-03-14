# System Player::increment_goals

- Ce fichier n'est pas modifiable par l'IA

It takes the id of a Player as an input and increment the number of goals of one unit.
Every 3 goals, generate a new award by a random issuer and a random year.

- language : PL/SQL
- entity: relire le fichier entity_player.md
- read and run the unit tests : read the file tests_increment_goals.md
- ne modifie pas les TUs lors de l'implémentation du code

## Rules 

- Issuer must be FIFA or FRANCE_FOOTBALL
- Year must be between 1980 and 2030

## Context
- You can directly check the schema for the PL/SQL code
- You can change the code in the database using the DENIS mcp connection
- Best separation of actions by using private procedure
- Use packages whenever it makes sense
- Simplify the code if possible
- Mettre de la doc en début de chacune de routine PL/SQL
- Use uppercase for reserved PL/SQL words
- Use constants for meaningful numeric or text business values