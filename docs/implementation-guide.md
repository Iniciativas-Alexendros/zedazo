# vcf-cribador — Guía de implementación

**Versión:** 0.1.1  
**Fecha:** 2026-08-15

> **Estado:** Las fases del MVP **v0.1.0** están implementadas en `main`. Las casillas siguientes documentan el historial de implementación; el trabajo pendiente vive en [ROADMAP.md](../ROADMAP.md) e issues.
>
> Documentos canónicos: [SPECS.md](../SPECS.md), [ARCHITECTURE.md](../ARCHITECTURE.md), [DECISIONS.md](../DECISIONS.md), [AGENTS.md](../AGENTS.md).

---

## Cómo leer esta guía

Para requisitos actuales usa [SPECS.md](../SPECS.md). Esta guía resume qué capas ya existen y qué queda para v0.2.0+.

**Documentos de referencia por rol:**

| Si necesitas... | Lee... |
|-----------------|--------|
| Objetivos, invariantes, criterios | [`SPECS.md`](../SPECS.md) |
| Modelo de dominio | [`docs/domain.md`](domain.md) |
| Arquitectura / CI | [`ARCHITECTURE.md`](../ARCHITECTURE.md) |
| Eventos y pipeline | [`docs/events.md`](events.md) |
| Testing | [`docs/test-plan.md`](test-plan.md) |
| Decisiones (ADR) | [`DECISIONS.md`](../DECISIONS.md) |
| Roadmap | [`ROADMAP.md`](../ROADMAP.md) |
| Agentes IA | [`AGENTS.md`](../AGENTS.md) |

---

## Fase 0 — Completada ✅

- [x] Estructura de proyecto, licencias, CI
- [x] Capas `domain` / `application` / `infrastructure` / `interfaces`
- [x] Documentación canónica en raíz

---

## Fase 1 — Parser ✅ (v0.1.0)

- [x] `infrastructure::parser` (nom 7), unfold/unescape, grouped props
- [x] `v3_compat`, `source`, `encoding`
- [x] Fixtures de integración

**Pendiente mayor:** migración nom 8 solo con ADR-0009 + PR dedicado.

---

## Fases 2–7 — MVP ✅ / parcial

Hecho en v0.1.0: screening C2–C6 / E1–E3, normalización FN/TEL/ORG, clasificación N1/N2, dedup Union-Find, writers VCF/CSV/JSON/TSV, CLI, config TOML base.

**Pendiente v0.2.0:**

- [ ] Reglas C1/C5/C7/E4/E6
- [ ] N3 / T4 / campo ADR (rama `feat/phase4-n3-t4-adr-clean`)
- [ ] `domain::verification` I1–I7 + pipeline TOML enriquecido (`feat/phase5-pipeline-toml`)
- [ ] OTel (`docs/otel.md`)

El checklist línea-a-línea anterior a 2026-08-15 está en el historial git de este archivo.
