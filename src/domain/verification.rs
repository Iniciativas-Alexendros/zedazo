//! Verificación de invariantes del dominio.

use std::path::Path;

use crate::domain::contact::Contact;
use crate::domain::screening::ScreeningDecision;

/// Error de invariante crítica (I1, I2, I3) que debe abortar el pipeline en modo estricto.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvariantError {
    pub code: &'static str,
    pub message: String,
    pub contact_uid: Option<String>,
}

impl std::fmt::Display for InvariantError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(ref uid) = self.contact_uid {
            write!(f, "[{}] {} (uid={})", self.code, self.message, uid)
        } else {
            write!(f, "[{}] {}", self.code, self.message)
        }
    }
}

impl std::error::Error for InvariantError {}

/// Verifica las invariantes de dominio sobre la lista de contactos procesados.
///
/// En modo no estricto: retorna advertencias (I4-I7) pero no falla.
/// En modo estricto: falla en invariantes críticas (I1, I2, I3).
pub fn verify(
    contacts: &[Contact],
    input: Option<&Path>,
    output: Option<&Path>,
    strict: bool,
) -> Result<Vec<String>, Vec<InvariantError>> {
    let mut warnings = Vec::new();
    let mut critical_errors = Vec::new();

    for contact in contacts {
        // I1 — Integridad de contactos conservados (CRÍTICA)
        if contact.decision == ScreeningDecision::Conserved {
            if contact.uid.trim().is_empty() {
                let err = InvariantError {
                    code: "I1",
                    message: format!("contacto sin UID (FN='{}')", contact.fn_value),
                    contact_uid: Some(contact.uid.clone()),
                };
                if strict {
                    critical_errors.push(err);
                } else {
                    warnings.push(err.to_string());
                }
            }
            if !contact.categories.has_n1() {
                let err = InvariantError {
                    code: "I1",
                    message: format!("contacto conservado sin categoría N1 (uid={})", contact.uid),
                    contact_uid: Some(contact.uid.clone()),
                };
                if strict {
                    critical_errors.push(err);
                } else {
                    warnings.push(err.to_string());
                }
            }
            if matches!(contact.source_detail, crate::domain::contact::SourceDetail::Unknown(ref s) if s.is_empty())
            {
                let err = InvariantError {
                    code: "I1",
                    message: format!(
                        "contacto conservado sin source_detail (uid={})",
                        contact.uid
                    ),
                    contact_uid: Some(contact.uid.clone()),
                };
                if strict {
                    critical_errors.push(err);
                } else {
                    warnings.push(err.to_string());
                }
            }
        }

        // I2 — FN canónico (solo para contactos que se conservan/revisan) (CRÍTICA)
        if matches!(
            contact.decision,
            ScreeningDecision::Conserved | ScreeningDecision::NeedsReview(_)
        ) && contact.fn_value.contains('@')
        {
            let err = InvariantError {
                code: "I2",
                message: format!("FN contiene '@' en contacto uid={}", contact.uid),
                contact_uid: Some(contact.uid.clone()),
            };
            if strict {
                critical_errors.push(err);
            } else {
                warnings.push(err.to_string());
            }
        }

        // I3 — TEL E.164 o marcado como no normalizable (CRÍTICA)
        if contact.decision == ScreeningDecision::Conserved {
            for tel in &contact.tels {
                if tel.normalized && !tel.value.starts_with('+') {
                    let err = InvariantError {
                        code: "I3",
                        message: format!("TEL normalizado sin prefijo '+' (uid={})", contact.uid),
                        contact_uid: Some(contact.uid.clone()),
                    };
                    if strict {
                        critical_errors.push(err);
                    } else {
                        warnings.push(err.to_string());
                    }
                }
            }
        }
    }

    // I7 — No destrucción: la salida no debe coincidir con la entrada (NO CRÍTICA)
    if let (Some(input), Some(output)) = (input, output) {
        if input == output {
            warnings.push("I7: la ruta de salida coincide con la de entrada".into());
        }
    }

    if !critical_errors.is_empty() {
        Err(critical_errors)
    } else {
        Ok(warnings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::contact::{CategorySet, Contact, SourceDetail};
    use crate::domain::screening::ScreeningDecision;

    fn make_contact(decision: ScreeningDecision) -> Contact {
        Contact {
            uid: "u1".into(),
            fn_value: "Test".into(),
            structured_name: None,
            org: None,
            org_fullname: None,
            org_legal_form: None,
            emails: vec![],
            tels: vec![],
            title: None,
            role: None,
            note: None,
            addresses: vec![],
            categories: CategorySet::default(),
            source_detail: SourceDetail::Unknown("test".into()),
            decision,
            screening_rule: String::new(),
            merged_uids: vec![],
            adr_field: None,
        }
    }

    #[test]
    fn test_verify_valid_conserved() {
        let mut c = make_contact(ScreeningDecision::Conserved);
        c.categories.n1.insert("PERS".into());
        let warnings = verify(&[c], None, None, false).unwrap();
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_verify_missing_n1() {
        let c = make_contact(ScreeningDecision::Conserved);
        let warnings = verify(&[c], None, None, false).unwrap();
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].contains("I1"));
    }

    #[test]
    fn test_verify_fn_with_at() {
        let mut c = make_contact(ScreeningDecision::Conserved);
        c.fn_value = "info@test.com".into();
        c.categories.n1.insert("PERS".into());
        let warnings = verify(&[c], None, None, false).unwrap();
        assert!(warnings.iter().any(|w| w.contains("I2")));
    }

    #[test]
    fn test_verify_input_equals_output() {
        let c = make_contact(ScreeningDecision::Eliminated(
            crate::domain::screening::ElimCode::E1,
        ));
        let path = Path::new("/tmp/test.vcf");
        let warnings = verify(&[c], Some(path), Some(path), false).unwrap();
        assert!(warnings.iter().any(|w| w.contains("I7")));
    }

    #[test]
    fn test_verify_strict_fails_on_i1() {
        let c = make_contact(ScreeningDecision::Conserved);
        let result = verify(&[c], None, None, true);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| e.code == "I1"));
    }

    #[test]
    fn test_verify_strict_fails_on_i2() {
        let mut c = make_contact(ScreeningDecision::Conserved);
        c.fn_value = "info@test.com".into();
        c.categories.n1.insert("PERS".into());
        let result = verify(&[c], None, None, true);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.iter().any(|e| e.code == "I2"));
    }
}
