# Changelog

Todas las modificaciones notables de este proyecto se documentan en este archivo.

El formato sigue [Keep a Changelog](https://keepachangelog.com/es-ES/1.1.0/),
y este proyecto adhiere a [Semantic Versioning](https://semver.org/lang/es/).

## [Unreleased]

### Changed
- Columna CSV: `CRIBADO_RESULT` → `CLASSIFY_RESULT` (inglés, coherente con cabeceras vCard; ADR-0014 I-12). JSON `zedazo_result` y props `X-ZEDAZO-*` sin cambio.
- README: badge Coveralls oculto hasta [#25](https://github.com/Iniciativas-Alexendros/zedazo/issues/25).

## [0.2.0] - 2026-08-15

### Changed
- **Rename de producto:** `vcf-cribador` → **Zedazo** (crate/binario `zedazo`). Ver [ADR-0014](DECISIONS.md).
- Props VCF de salida: `X-CRIBADO-*` → `X-ZEDAZO-*`
- Sección TOML canónica: `[zedazo]` (alias `[cribado]` con warning de deprecación)
- Campo JSON export: `cribado_result` → `zedazo_result`
- Convención de config documentada: `zedazo.toml` (path libre vía `-c`)
- Help CLI: sufijo de salida `<input>_zedazo.vcf`
- URLs de repo/docs → `Iniciativas-Alexendros/zedazo`
- ROADMAP: rename = **v0.2.0**; integraciones → **v0.4.0**

### Añadido
- Taxonomía de clasificación ampliada a tres niveles (N1/N2/N3)
- Soporte completo para tipos de teléfono T4 con mapeo desde vCard
- Soporte para propiedades ADR (direcciones postales) en parser, `Contact`, writer y exports
- Regla E3 de huérfanos actualizada para considerar direcciones
- Módulos `domain::audit` y `domain::verification` (invariantes)
- Pipeline de aplicación conectado de extremo a extremo con configuración TOML enriquecida

### Migración desde vcf-cribador

| Elemento | Antes | Ahora |
|---|---|---|
| Instalar | `cargo install vcf-cribador` | `cargo install zedazo` |
| Invocación | `vcf-cribador cribar …` | `zedazo cribar …` |
| Props VCF | `X-CRIBADO-RESULT\|VERSION\|DATE` | `X-ZEDAZO-RESULT\|VERSION\|DATE` |
| TOML | `[cribado]` | `[zedazo]` (alias `[cribado]` deprecado) |
| Config file (docs) | `cribador.toml` | `zedazo.toml` |
| JSON | `cribado_result` | `zedazo_result` |
| Subcomando | `cribar` | `cribar` (sin cambio) |

El crate `vcf-cribador` en crates.io permanece publicado (sin yank); la última línea 0.1.x apunta a `zedazo`.

## [0.1.1] - 2026-08-15

### Corregido
- Metadata `repository`/`homepage` en `Cargo.toml` tras transferencia a la org `Iniciativas-Alexendros`
- Badges y enlaces del README apuntando a la cuenta personal antigua
- Enlace de licencia dual (`LICENSE-MIT` / `LICENSE-APACHE`)
- Fecha incorrecta de `0.1.0` en el changelog (2025 → 2026)
- Panic potencial en `normalize_fn` al extraer roles (C-01)
- Panic potencial en `deduplicate` al materializar clusters (C-02)

## [0.1.0] - 2026-07-08

### Añadido
- Pipeline completo de cribado: parsing → normalización → clasificación → screening → dedup → escritura
- Comando `cribar` con soporte `--dry-run`
- Comando `audit` para auditoría standalone sin modificar VCF
- Comando `stats` con formatos texto, JSON y Markdown
- Comando `export` con formatos CSV y JSON
- Parser vCard 4.0/3.0 RFC 6350 con unfold, desescapado y propiedades agrupadas
- Compatibilidad automática vCard 3.0 → 4.0 (Google, Apple, Proton)
- Detección automática de fuente (ProtonAutosave, GoogleContacts, Apple)
- Transcodificación ISO-8859-1 → UTF-8 (chardetng + encoding_rs)
- Escritor VCF 4.0 con folding a 75 octetos respetando UTF-8 multibyte
- Escritor TSV de auditoría con 11 columnas (trazabilidad completa)
- Clasificación automática C2-C6 con 16 categorías N2
- Eliminación E1-E3 (huerfanos, spam, email-only)
- Deduplicación con Union-Find (cierre transitivo TEL + EMAIL + FN)
- Normalización E.164 para teléfonos españoles (+34)
- Normalización de nombres propios (capitalización, títulos, cargos)
- Configuración externa TOML con soporte replace/append
- CI/CD con GitHub Actions (check + fmt + clippy + test + doc)
- Release automatizado con binario + SHA256
- 129 tests unitarios y de integración

[Unreleased]: https://github.com/Iniciativas-Alexendros/zedazo/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/Iniciativas-Alexendros/zedazo/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/Iniciativas-Alexendros/zedazo/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/Iniciativas-Alexendros/zedazo/releases/tag/v0.1.0
