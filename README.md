# ramanujan_oracle_tu

Service Rust minimal avec `axum` et un test Oracle lisant sa configuration depuis `env.toml`.

## Prérequis

- Rust récent
- Client Oracle disponible sur la machine cible (`libclntsh.so`)

## Lancer le serveur

1. Renseigner `env.toml`
2. Exécuter `cargo run`
3. Appeler `GET /health/oracle`

## Lancer le test

```bash
cargo test select_one_from_dual_returns_one -- --nocapture
```

Le test exécute uniquement :

```sql
select 1 from dual
```

Serveur MCP 

```bash
cd /home/denis/SQLcl
./sqlcl/bin/sql -mcp
```
