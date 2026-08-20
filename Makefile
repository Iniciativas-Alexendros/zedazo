.PHONY: all check test build release clean hooks version bump fmt clippy doc deny help docs-validate

CARGO := cargo
BINARY := target/release/zedazo

##@ Desarrollo

all: check test build

check: ## Analiza el código sin compilar binario final
	$(CARGO) check --all-features

test: ## Ejecuta todos los tests
	$(CARGO) test --all-features

build: ## Compila en modo debug
	$(CARGO) build

release: ## Compila en modo release
	$(CARGO) build --release --locked

clean: ## Limpia artefactos de compilación
	$(CARGO) clean

##@ Calidad

deny: ## Licencias/advisories (cargo-deny; ADR-0013)
	@command -v cargo-deny >/dev/null || (echo "Instala: cargo install cargo-deny" && exit 1)
	cargo deny check

docs-validate: ## Valida documentación canónica (frontmatter, enlaces, stubs, trazabilidad)
	@echo "→ Validando frontmatter en docs canónicos..."
	@for f in SPECS.md ROADMAP.md DECISIONS.md ARCHITECTURE.md AGENTS.md; do \
		if ! head -20 "$$f" | grep -q "^version:"; then \
			echo "❌ $$f: falta frontmatter"; exit 1; \
		fi; \
		echo "✓ $$f"; \
	done
	@echo "→ Validando stubs en docs/..."
	@for f in docs/spec.md docs/architecture.md docs/roadmap.md docs/tasks.md; do \
		if ! grep -q "redirige aquí\|canónico" "$$f"; then \
			echo "❌ $$f: no redirige a doc canónico"; exit 1; \
		fi; \
		echo "✓ $$f"; \
	done
	@echo "→ Validando enlaces internos..."
	@grep -r '\[.*\](\.?/.*\.md)' --include='*.md' . | grep -v '^Binary' | while read line; do \
		file=$$(echo "$$line" | cut -d: -f1); \
		link=$$(echo "$$line" | grep -o '](.*)' | sed 's/]//;s/(//;s/)//'); \
		if [[ "$$link" == http* ]]; then continue; fi; \
		target=$$(dirname "$$file")/$$link; \
		if [[ ! -f "$$target" ]]; then \
			echo "❌ Enlace roto en $$file: $$link"; exit 1; \
		fi; \
	done
	@echo "✓ Enlaces internos OK"
	@echo "→ Validando trazabilidad SPECS↔ROADMAP↔DECISIONS..."
	@if ! grep -q "SPECS.md" ROADMAP.md || ! grep -q "DECISIONS.md" ROADMAP.md; then \
		echo "❌ ROADMAP.md no referencia SPECS/DECISIONS"; exit 1; \
	fi
	@if ! grep -q "ROADMAP.md" SPECS.md || ! grep -q "DECISIONS.md" SPECS.md; then \
		echo "❌ SPECS.md no referencia ROADMAP/DECISIONS"; exit 1; \
	fi
	@if ! grep -q "SPECS.md" DECISIONS.md || ! grep -q "ROADMAP.md" DECISIONS.md; then \
		echo "❌ DECISIONS.md no referencia SPECS/ROADMAP"; exit 1; \
	fi
	@if ! grep -q "ROADMAP.md" ARCHITECTURE.md || ! grep -q "DECISIONS.md" ARCHITECTURE.md; then \
		echo "❌ ARCHITECTURE.md no referencia ROADMAP/DECISIONS"; exit 1; \
	fi
	@echo "✓ Trazabilidad OK"
	@echo "✓ docs-validate completado"

traceability: ## Genera matriz de trazabilidad SPECS→Módulo→Test
	@sh scripts/traceability.sh

fmt: ## Formatea el código
	$(CARGO) fmt

fmt-check: ## Verifica el formateo (CI)
	$(CARGO) fmt --all -- --check

clippy: ## Linter estricto
	$(CARGO) clippy --all-features -- -D warnings

lint: fmt clippy ## Formatea + linter

##@ Git

hooks: ## Instala hooks pre-commit
	@sh .githooks/install.sh

##@ Versionado

version: ## Muestra la versión actual desde Cargo.toml
	@grep '^version' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/'

bump-patch: ## Incrementa versión patch (0.1.0 → 0.1.1)
	@sh scripts/bump.sh patch

bump-minor: ## Incrementa versión minor (0.1.0 → 0.2.0)
	@sh scripts/bump.sh minor

bump-major: ## Incrementa versión major (0.1.0 → 1.0.0)
	@sh scripts/bump.sh major

##@ CI local

ci: fmt-check clippy test check doc docs-validate ## Simula CI completa

doc: ## Genera documentación
	$(CARGO) doc --no-deps --document-private-items
	@echo "Documentación generada en target/doc/"

##@ Otros

completions: ## Genera scripts de autocompletado (bash/zsh/fish)
	$(CARGO) build --release
	mkdir -p completions
	$(BINARY) completions bash > completions/zedazo.bash
	$(BINARY) completions zsh  > completions/_zedazo
	$(BINARY) completions fish > completions/zedazo.fish
	@echo "Completions generados en completions/"
	@echo "  bash: source completions/zedazo.bash"
	@echo "  zsh:  fpath+=(completions/_zedazo)"
	@echo "  fish: cp completions/zedazo.fish ~/.config/fish/completions/"

help: ## Muestra esta ayuda
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'
