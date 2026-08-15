# ROADMAP.md

**Versión:** 0.1.1  
**Fecha:** 2026-08-15  
**Canónico:** este archivo. [`docs/tasks.md`](docs/tasks.md) redirige aquí.

---

## Reglas

- Cada hito referencia requisitos de [SPECS.md](./SPECS.md).
- Desviaciones relevantes → [DECISIONS.md](./DECISIONS.md).
- Trabajo de features en PRs pequeños; CI verde antes de pedir merge.
- Estimación relativa: S / M / L.

## Estado actual

| Dimensión | Estado |
|-----------|--------|
| Versión publicada | 0.1.0 en crates.io; **0.1.1** en vuelo (metadata + panic-safe) |
| Tests | 129+ (main); cobertura vía `cargo llvm-cov` → Coveralls |
| CI | Self-hosted `[self-hosted, ts]`; Renovate (no Dependabot) |
| Branch protection | Activa en `main` |
| Backlog trazable | Issues GitHub (abrir si faltan) |

## Hitos

### v0.1.1 — Metadata y robustez (en curso) — S

**Criterio de salida:**
- [x] `repository`/`homepage` y badges → org `Iniciativas-Alexendros`
- [x] Sin `unwrap` panic-prone en `domain/` (C-01, C-02)
- [ ] Tag `v0.1.1` + release.yml + crates.io actualizado

### v0.2.0 — Calidad y robustez — L

**Criterio de salida:**
- [ ] Reglas C1, C5, C7, E4, E6 con tests
- [ ] Campo ADR + taxonomía N3 / tipos T4 (rescate phase4)
- [ ] Pipeline + config TOML enriquecida (rescate phase5)
- [ ] `domain::verification` aplica I1–I7 en pipeline
- [ ] Documentación canónica raíz alineada (SPECS/ROADMAP/DECISIONS/AGENTS)
- [ ] Issues cerrados o movidos con criterio explícito

**Fuera de alcance v0.2.0:** CardDAV, watch mode, GUI.

### v0.3.0 — Integraciones — L

- CardDAV sync, watch mode, filtros por categoría
- Requiere ADR de proveedor/red antes de implementar

### v1.0.0 — Producción — L

- API estable (semver estricto)
- Benchmarks (criterion)
- Cross-compile macOS/Windows — **bloqueado por decisión de release** (ADR pendiente: matrix GitHub-hosted vs `cargo-dist`)
- Corpus de regresión >10k contactos

## Ramas de trabajo rescatables

| Rama | Contenido | Acción |
|------|-----------|--------|
| `feat/phase4-n3-t4-adr-clean` | N3, T4, ADR, `verification.rs` | Rebase + PR draft |
| `feat/phase5-pipeline-toml` | phase4 + pipeline/TOML | Rebase tras phase4 |
| `feat/phase4-n3-t4-adr` | Duplicado ruidoso | Cerrar tras adoptar `-clean` |

## Dependencias congeladas (ver DECISIONS)

- `nom` < 8, `toml` < 1, `chardetng` < 1 (Renovate `allowedVersions`)
