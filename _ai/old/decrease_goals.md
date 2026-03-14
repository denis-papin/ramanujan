# System Player::decrease_points

- Ce fichier n'est pas modifiable par l'IA

It takes the id of a Player as an input and decrement the number of points of one unit.
Every 3 points, the total number of awards must stay aligned with the number of points.

- language : PL/SQL
- entity: relire le fichier entity_player.md
- read and run the unit tests : read the file tests_decrease_points.md
- ne modifie pas les TUs lors de l'implémentation du code

## Rules

- Issuer must be GOLD or SILVER
- Year must be between 1980 and 2030
- Player points cannot become negative
- Award count must always be equal to floor(points / 3)

## Context
- You can directly check the schema for the PL/SQL code
- You can change the code in the database using the DENIS mcp connection
- Best separation of actions by using private procedure
- Use packages whenever it makes sense
- Simplify the code if possible
- Mettre de la doc en début de chacune de routine PL/SQL
- Use uppercase for reserved PL/SQL words
- Use constants for meaningful numeric or text business values
