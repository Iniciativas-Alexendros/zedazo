//! Traits de pipeline para inversión de dependencias (domain ← infrastructure).

use crate::domain::rules::ClassificationRule;
use crate::domain::screening::ScreeningConfig;

/// Proveedor de configuración de cribado.
/// Permite al dominio no depender de la infraestructura concreta.
pub trait ScreeningConfigProvider {
    fn screening_config(&self) -> &ScreeningConfig;
}

/// Proveedor de reglas de clasificación.
pub trait ClassificationRuleProvider {
    fn classification_rules(&self) -> &[ClassificationRule];
}

/// Proveedor combinado para el pipeline completo.
pub trait PipelineConfigProvider: ScreeningConfigProvider + ClassificationRuleProvider {}
