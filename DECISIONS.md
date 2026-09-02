---
version: "0.3.0"
date: "2026-09-02"
status: "Activo"
canonical: true
supersedes: "v0.1.x"
---

# DECISIONS.md

**Versión:** 0.3.0  
**Fecha:** 2026-09-02  
**Canónico:** este archivo. [`docs/adr/README.md`](docs/adr/README.md) conserva el texto histórico de ADR-0001…0005 y apunta aquí para IDs nuevos.

---

## Convenciones

- IDs secuenciales **ADR-XXXX**.
- Estados: propuesta | aceptada | sustituida | rechazada | retirada.
- Una decisión aceptada no se reescribe: se sustituye con otra.
- Decisor: Alexendros.

---

<details>
<summary><strong>ADR-0001</strong> — Salida canónica siempre vCard 4.0</summary>

- Estado: aceptada
- Fecha: 2026-07-07
- Contexto: Entradas 3.0 y 4.0; hace falta un formato de salida único.
- Decisión: Salida siempre vCard 4.0 (RFC 6350); `v3_compat` adapta 3.0→4.0.
- Consecuencias: Un solo writer; AGENT/LABEL/MAILER no se propagan.
- Relacionado: [SPECS.md](./SPECS.md) I4, I5.

</details>

<details>
<summary><strong>ADR-0002</strong> — Separación ParsedVCard (infra) / Contact (dominio)</summary>

- Estado: aceptada
- Fecha: 2026-07-07
- Decisión: Dos modelos; `ParsedVCard::into_contact()` traduce.
- Consecuencias: Dominio sin escapes RFC ni PHOTO raw.

</details>

<details>
<summary><strong>ADR-0003</strong> — Union-Find para deduplicación transitiva</summary>

- Estado: aceptada
- Fecha: 2026-07-07
- Decisión: DSU + materialización con índices descendentes.
- Consecuencias: Cierre transitivo D2; `merged_uids` conserva absorbidos.

</details>

<details>
<summary><strong>ADR-0004</strong> — std::sync::LazyLock, no once_cell</summary>

- Estado: aceptada
- Fecha: 2026-07-07
- Decisión: `LazyLock` (MSRV 1.80); no añadir `once_cell`.

</details>

<details>
<summary><strong>ADR-0005</strong> — jiff en lugar de chrono</summary>

- Estado: aceptada
- Fecha: 2026-07-07
- Decisión: Usar `jiff` para timestamps de auditoría/trace.

</details>

<details>
<summary><strong>ADR-0006</strong> — Transferencia del repositorio a Iniciativas-Alexendros</summary>

- Estado: aceptada
- Fecha: 2026-07
- Contexto: El repo vivía en la cuenta personal `Alexendros`.
- Decisión: Org `Iniciativas-Alexendros`; actualizar Cargo.toml, badges, changelog y Notion.
- Consecuencias: v0.1.1 republish para corregir crates.io/docs.rs; Coveralls bajo la org nueva.

</details>

<details>
<summary><strong>ADR-0007</strong> — Migración Dependabot → Renovate</summary>

- Estado: aceptada
- Fecha: 2026-07-31
- Decisión: Renovate con reglas en `.github/renovate.json`; eliminar `dependabot.yml`.
- Consecuencias: Presets compartidos cross-repo vía PR #17 (pendiente de merge).

</details>

<details>
<summary><strong>ADR-0008</strong> — Runners self-hosted `[self-hosted, ts]`</summary>

- Estado: aceptada (enmienda operativa 2026-08-15)
- Fecha: 2026-07
- Decisión: CI/release Linux preferentemente en runners propios; `publish` a crates.io en `ubuntu-latest`.
- Enmienda 2026-08-15: con **0 runners registrados**, los workflows pasaron a `ubuntu-latest` (#30) para desbloquear CI/release. Restaurar `[self-hosted, ts]` cuando el runner `ts` vuelva a estar online.
- Consecuencias: Sin matrix multi-OS en CI; cross-compile v1.0 requiere ADR-0012.
- Relacionado: [ARCHITECTURE.md](./ARCHITECTURE.md), PR #30.

</details>

<details>
<summary><strong>ADR-0009</strong> — Rechazo temporal de nom 8, toml 1.x y chardetng 1.0</summary>

- Estado: aceptada
- Fecha: 2026-07
- Contexto: PRs Dependabot/Renovate de majors; impacto concentrado en `parser.rs` (nom 8: trait `Parser`).
- Decisión: Congelar con `allowedVersions` en Renovate (`nom<8`, `toml<1`, `chardetng<1`).
- Consecuencias: Upgrade solo con PR dedicado + migración; no mezclar con features.

</details>

<details>
<summary><strong>ADR-0010</strong> — panic = "abort" en release</summary>

- Estado: aceptada
- Fecha: 2026-07 / reafirmada 2026-08-15
- Decisión: Mantener `panic = "abort"` en `[profile.release]` por tamaño/binario.
- Consecuencias: El dominio DEBE ser total (sin `unwrap` de fallo). Fixes C-01/C-02 en v0.1.1.
- Alternativa rechazada: `unwind` (más código, poco beneficio en CLI batch).

</details>

<details>
<summary><strong>ADR-0011</strong> — Cobertura con cargo-llvm-cov + Coveralls</summary>

- Estado: aceptada
- Fecha: 2026-07
- Decisión: Job `coverage` en CI genera LCOV con `cargo llvm-cov` y sube a Coveralls.
- Consecuencias: Badge Coveralls debe apuntar a la org correcta; umbral mínimo opcional (issue).

</details>

<details>
<summary><strong>ADR-0012</strong> — Release multiplataforma (propuesta)</summary>

- Estado: propuesta
- Fecha: 2026-08-15
- Contexto: v1.0 promete macOS/Windows; runners actuales solo Linux.
- Opciones a decidir: (A) matrix `ubuntu/macos/windows-latest` solo en `release.yml`; (B) `cargo-dist`.
- Bloquea: hito v1.0.0 cross-compile.

</details>

<details>
<summary><strong>ADR-0013</strong> — Supply chain: cargo-deny + SBOM (propuesta)</summary>

- Estado: propuesta
- Fecha: 2026-08-15
- Decisión propuesta: Añadir `cargo deny check` en CI/Makefile; SBOM en release.
- Relacionado: [ROADMAP.md](./ROADMAP.md) calidad post-0.1.1.

</details>

<details>
<summary><strong>ADR-0014</strong> — Rename producto/crate a Zedazo</summary>

- Estado: aceptada
- Fecha: 2026-08-15
- Contexto: `vcf-cribador` colisiona semánticamente con el verbo de dominio *cribar*; se busca marca de producto distinta (patrón Atlaps). `cedazo` descartado. Gate crates.io: `zedazo` libre (`ze/da/zedazo` → 404). TMview UE pendiente humano.
- Decisión: Renombrar producto/crate/binario a **Zedazo** (`zedazo`) en release **v0.2.0** solo rename+migración. Internos de dominio (`CribaError`, módulo `cribar`, «cribado») sin rename.
- Relacionado: issue [#32](https://github.com/Iniciativas-Alexendros/zedazo/issues/32), [ROADMAP.md](./ROADMAP.md), [CHANGELOG.md](./CHANGELOG.md).

| # | Elemento | Actual | Decisión | Tipo |
|---|---|---|---|---|
| I-01 | Crate + binario | `vcf-cribador` | `zedazo` | Breaking |
| I-02 | Props VCF | `X-CRIBADO-*` | `X-ZEDAZO-*` | Breaking |
| I-03 | Sección TOML | `[cribado]` | `[zedazo]` + alias `[cribado]` con warning deprecación | Suave |
| I-04 | Subcomando CLI | `cribar` | **Mantener** (verbo de dominio) | Sin cambio |
| I-05 | Versión debut | — | **v0.2.0** = solo rename/migración | Estrategia |
| I-06 | Crate antiguo | `vcf-cribador` 0.1.0/0.1.1 en crates.io | Publicar **0.1.2** final con aviso → `zedazo`; **sin yank** | Estrategia |
| I-07 | Metadata org | Ya `Iniciativas-Alexendros` | Solo actualizar path a `/zedazo` | Fix menor |
| I-08 | Config file | `cribador.toml` | Doc → `zedazo.toml`; path libre vía `-c` | Docs + convención |
| I-09 | Sufijo salida help | `<input>_cribado.vcf` | `<input>_zedazo.vcf` | Cosmético |
| I-10 | JSON export | `cribado_result` | `zedazo_result` | Breaking suave |
| I-11 | Internos | `CribaError`, módulo `cribar`, dominio «cribado» | **Sin rename** | Explícito |
| I-12 | CSV export | `CRIBADO_RESULT` | `CLASSIFY_RESULT` (inglés, coherente con cabeceras vCard; no marca) | Breaking suave |

- Roadmap en el mismo PR: calidad (ex-v0.2.0) → **v0.3.0**; CardDAV/watch → **v0.4.0**.
- Consecuencias: Breaking en crate name, props VCF y campo JSON; configs `[cribado]` siguen funcionando con deprecación; repo GitHub → `zedazo` (redirects).

</details>
