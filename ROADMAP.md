# ROADMAP.md

**Versión:** 0.2.0  
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
| Versión publicada | 0.1.0 en crates.io (`vcf-cribador`); **0.2.0** debut **Zedazo** (`zedazo`) en vuelo |
| Tests | 129+ (main); cobertura vía `cargo llvm-cov` → Coveralls |
| CI | `ubuntu-latest` (#30; self-hosted cuando vuelva); Renovate |
| Branch protection | Activa en `main` |
| Backlog trazable | Issues GitHub (#32 rename) |

## Hitos

### v0.1.1 — Metadata y robustez — S

**Criterio de salida:**
- [x] `repository`/`homepage` y badges → org `Iniciativas-Alexendros`
- [x] Sin `unwrap` panic-prone en `domain/` (C-01, C-02)
- [ ] Tag `v0.1.1` + release.yml + crates.io (`vcf-cribador` aviso → `zedazo`)

### v0.2.0 — Rename a Zedazo (migración + base phase4/5 ya en main) — M

**Criterio de salida:**
- [ ] Crate/binario `zedazo`; repo `Iniciativas-Alexendros/zedazo`
- [ ] Props `X-ZEDAZO-*`, TOML `[zedazo]` (+ alias `[cribado]`), JSON `zedazo_result`
- [ ] ADR-0014; CHANGELOG con tabla de migración
- [ ] Subcomando `cribar` sin cambio (verbo de dominio)
- [ ] `make ci` verde; docs canónicos alineados
- [x] Campo ADR + taxonomía N3 / tipos T4 (#28)
- [x] Pipeline + config TOML enriquecida (#29)

**Fuera de alcance v0.2.0:** CardDAV, marca registrada, dominio de pago.

### v0.3.0 — Calidad y robustez — L

**Criterio de salida:**
- [ ] Reglas C1, C5, C7, E4, E6 con tests
- [ ] `domain::verification` aplica I1–I7 en pipeline
- [ ] Documentación canónica raíz alineada (SPECS/ROADMAP/DECISIONS/AGENTS)
- [ ] Issues cerrados o movidos con criterio explícito

**Fuera de alcance v0.3.0:** CardDAV, watch mode, GUI.

### v0.4.0 — Integraciones — L

*(antes numerado v0.3.0)*

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
| `feat/phase4-n3-t4-adr-clean` | N3, T4, ADR, `verification.rs` | Rebase + PR draft → v0.3.0 |
| `feat/phase5-pipeline-toml` | phase4 + pipeline/TOML | Rebase tras phase4 → v0.3.0 |
| `feat/phase4-n3-t4-adr` | Duplicado ruidoso | Cerrar tras adoptar `-clean` |

## Dependencias congeladas (ver DECISIONS)

- `nom` < 8, `toml` < 1, `chardetng` < 1 (Renovate `allowedVersions`)
