# Escenarios — Protección de main

## Happy paths

| ID | Escenario | Resultado esperado |
|----|-----------|-------------------|
| H1 | `make ci` en local pasa | Exit 0 |
| H2 | PR a `main` con CI verde | Checks ✅ |
| H3 | Branch protection aplicada | Push directo denegado |
| H4 | Renovate no propone `nom>=8` / `toml>=1` / `chardetng>=1` | Filtrado por `allowedVersions` |
| H5 | Coverage sube LCOV a Coveralls | Job Coverage ✅ |

## Edge cases

| ID | Escenario | Resultado esperado |
|----|-----------|-------------------|
| E1 | PR con conflictos tras rebase | Resolver manualmente |
| E2 | Renovate major de nom | PR sin automerge; requiere ADR + migración |
| E3 | Merge sin CI verde | Bloqueado por branch protection |

## Errores esperados

| ID | Error | Manejo |
|----|-------|--------|
| R1 | Token sin permisos de protection | Pedir `repo` admin |
| R2 | Runner sin llvm-tools / cargo-llvm-cov | Instalar en runner self-hosted |
| R3 | Coveralls badge org antigua | Actualizar README + proyecto Coveralls |

## Histórico

Los escenarios H2/H3/H5 relativos a Dependabot PRs #8/#12 están **archivados** (migración a Renovate, ADR-0007).
