# AGENTS.md

**Versión:** 0.2.0  
**Fecha:** 2026-08-15  
**Propósito:** Contrato operativo para agentes de código en este repo.

---

## 1. Destinatario

- Implementas en Rust. El humano dirige, revisa diffs y fusiona.
- Español en commits, PRs y mensajes al humano.
- Una sesión = una unidad cohesiva. PR pequeño. CI verde antes de pedir revisión.

## 2. Fuentes de verdad (orden)

1. [README.md](./README.md)
2. Este archivo
3. Fase activa de [ROADMAP.md](./ROADMAP.md)
4. Requisitos citados en [SPECS.md](./SPECS.md)
5. Capas/CI en [ARCHITECTURE.md](./ARCHITECTURE.md)
6. [DECISIONS.md](./DECISIONS.md) antes de deps nuevas, majors, runners o `panic`/`profile`

NO inventes requisitos. Si falta ancla, paras y preguntas.

## 3. Ficha de unidad de trabajo

```
Objetivo: <resultado verificable>
Traza: <SPECS / ADR / issue>
Alcance: <archivos>
Exclusiones: <qué no harás>
Dependencias: <PR/rama previa>
Pruebas: make ci / cargo test --all-features
Criterio de cierre: CI verde + criterio SPECS
```

## 4. Autonomía

**Puedes sin preguntar**

- Implementar ítems ya especificados en SPECS + fase activa
- Tests que fijan comportamiento aceptado
- Corregir fmt/clippy/CI causados por tu cambio
- Refactors locales sin cambiar CLI pública

**Requiere confirmación**

- Dependencia nueva o major (sobre todo nom/toml/chardetng)
- Cambiar `panic`/`profile.release` o runners
- Features de red (CardDAV, watch)
- Alterar un ADR aceptado
- Publicar crates.io / tags de release (el humano lanza o confirma)

## 5. Working agreement

- Capas: `domain` puro; sin I/O. `unwrap` solo en tests.
- MSRV 1.80. `cargo clippy -- -D warnings`.
- Fixtures 100 % sintéticos; nunca PII real.
- No mezclar upgrade de parser (nom 8) con features de dominio.
- Hooks: `make hooks` instala pre-commit (fmt + clippy). Documentado en CONTRIBUTING.

## 6. Comandos útiles

```bash
make hooks    # .githooks/install.sh
make ci       # fmt-check + clippy + test + check + doc
make release  # binario release
cargo llvm-cov --lcov --output-path coverage/lcov.info
```

## 7. Definition of Done

- Criterios de aceptación de la traza cumplidos
- `make ci` verde
- Docs canónicos actualizados si cambia contrato
- Sin secretos en el diff
