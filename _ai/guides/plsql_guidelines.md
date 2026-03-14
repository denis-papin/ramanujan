# Guide PL/SQL

## Objectif

Centraliser les conventions générales PL/SQL afin de ne pas surcharger les fiches métier.

## Conventions générales

- Utiliser des packages lorsque cela a du sens.
- Utiliser des procédures privées pour isoler les sous-actions métier.
- Utiliser des mots réservés PL/SQL en majuscules.
- Utiliser des constantes pour les valeurs métier significatives.
- Mettre un commentaire court en tête de chaque routine.
- Préférer des erreurs métier explicites aux erreurs techniques implicites.

## Gestion des erreurs

- Lever des erreurs métier avec `raise_application_error` quand le domaine le demande.
- Vérifier explicitement les cas `ID NULL`, `ligne absente`, ou invariant non respecté.
- Ne pas dépendre d'un comportement implicite si un contrôle métier clair est possible.

## Lisibilité

- Une routine principale courte.
- Des noms de procédures et paramètres alignés avec le vocabulaire métier.
- Un nombre limité de variables temporaires.

## Lien avec les fiches métier

Les fiches de flux ne doivent pas recopier toutes ces règles.

Elles doivent seulement mentionner, si nécessaire :

- la technologie cible
- les contraintes réellement spécifiques au flux
