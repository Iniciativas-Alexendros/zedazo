# vcf-cribador (DEPRECATED)

> **Este crate se ha renombrado a [`zedazo`](https://crates.io/crates/zedazo).**
>
> Instala el reemplazo: `cargo install zedazo`
>
> Repositorio: https://github.com/Iniciativas-Alexendros/zedazo  
> Documentación: https://docs.rs/zedazo

Esta es la última publicación del nombre `vcf-cribador` (**0.1.2**). **No se hará yank** de versiones anteriores para no romper instalaciones existentes.

## Migración rápida

| Elemento | Antes (`vcf-cribador`) | Ahora (`zedazo`) |
|---|---|---|
| Instalar | `cargo install vcf-cribador` | `cargo install zedazo` |
| Invocación | `vcf-cribador cribar …` | `zedazo cribar …` |
| Props VCF | `X-CRIBADO-RESULT\|VERSION\|DATE` | `X-ZEDAZO-RESULT\|VERSION\|DATE` |
| TOML | `[cribado]` | `[zedazo]` (alias `[cribado]` deprecado) |
| Config (docs) | `cribador.toml` | `zedazo.toml` |
| JSON | `cribado_result` | `zedazo_result` |
| Subcomando | `cribar` | `cribar` (sin cambio) |

Detalles: [CHANGELOG 0.2.0](https://github.com/Iniciativas-Alexendros/zedazo/blob/main/CHANGELOG.md) · [ADR-0014](https://github.com/Iniciativas-Alexendros/zedazo/blob/main/DECISIONS.md)

## Nota

El binario y la API de esta versión 0.1.2 no cambian respecto a 0.1.1 salvo metadata/README de deprecación.
