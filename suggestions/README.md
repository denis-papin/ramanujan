# suggestions

Ce dossier contient :

- des templates AI SDD réutilisables
- des fiches transformées à partir des documents actuels du dossier `_ai/`
- des guides globaux pour séparer les règles métier des conventions transverses

Les originaux ne sont pas modifiés.

## Structure

- `guides/`
  - `ai_sdd_guide.md`
  - `plsql_guidelines.md`
- `templates/`
  - `entity.template.md`
  - `flow.template.md`
  - `integration-test.template.md`
  - `bug.template.md`
- `examples/`
  - `entity_player.md`
  - `flow_player_increment_goals.md`
  - `flow_player_decrease_goals.md`
  - `integration_tests_player_increment_goals.md`
  - `integration_tests_player_decrease_goals.md`
  - `bug_0001.md`

## Principes appliqués

- front matter stable
- identifiants explicites
- séparation métier / technique
- langue majoritairement française
- termes métier stables conservés en anglais, par exemple `Player`, `Award`
- règles métier formulées de façon testable
- traçabilité entre entités, flux, tests et bugs
