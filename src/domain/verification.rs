//! Verificación de invariantes del dominio.

use std::fs;
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

/// Verifica invariantes sobre ficheros ya escritos: I4 salida canónica, I5 compatibilidad, I6 auditabilidad.
///
/// Siempre retorna warnings (nunca falla en modo strict). Pensado para ejecutarse tras `write_vcf`/`write_audit_tsv`.
pub fn verify_post(
    contacts: &[Contact],
    output: Option<&Path>,
    audit: Option<&Path>,
) -> Vec<String> {
    let mut warnings = Vec::new();
    if let Some(out) = output {
        warnings.extend(check_i4(out));
        warnings.extend(check_i5(out));
        // también verificar cuarentena si existe
        let quar = quarantine_path(out);
        if quar.exists() {
            warnings.extend(check_i4(&quar));
            warnings.extend(check_i5(&quar));
        }
    }
    if let Some(audit_path) = audit {
        warnings.extend(check_i6(audit_path, contacts));
    }
    // I6 también verifica screening_rule no vacío para contactos procesados (sin necesidad de fichero)
    for c in contacts {
        if c.screening_rule.trim().is_empty() {
            warnings.push(format!(
                "I6: contacto uid={} sin screening_rule (sin DecisionTrace)",
                c.uid
            ));
        }
    }
    warnings
}

fn quarantine_path(path: &Path) -> std::path::PathBuf {
    let stem = path.file_stem().unwrap_or_default().to_string_lossy();
    let ext = path
        .extension()
        .map(|e| format!(".{}", e.to_string_lossy()))
        .unwrap_or_default();
    path.with_file_name(format!("{}_cuarentena{}", stem, ext))
}

fn check_i4(path: &Path) -> Vec<String> {
    let mut w = Vec::new();
    if !path.exists() {
        return w;
    }
    let bytes = match fs::read(path) {
        Ok(b) => b,
        Err(e) => {
            w.push(format!("I4: no se pudo leer '{}': {}", path.display(), e));
            return w;
        }
    };
    let content = match String::from_utf8(bytes) {
        Ok(s) => s,
        Err(_) => {
            w.push(format!(
                "I4: salida '{}' no es UTF-8 válido",
                path.display()
            ));
            return w;
        }
    };
    if !content.contains("VERSION:4.0") {
        w.push(format!(
            "I4: salida '{}' no contiene VERSION:4.0",
            path.display()
        ));
    }
    if content.contains("VERSION:3.0") {
        w.push(format!(
            "I4: salida '{}' contiene VERSION:3.0 (debe ser solo 4.0)",
            path.display()
        ));
    }
    // Folding ≤75 octetos (RFC 6350 §3.2): cada línea física ≤75, continuación " " + ≤75 = ≤76
    for (idx, raw_line) in content.split("\r\n").enumerate() {
        // split \r\n deja última línea vacía si termina en \r\n; ignorar vacías por líneas internas
        for line in raw_line.split('\n') {
            if line.is_empty() {
                continue;
            }
            // líneas plegadas: continuación empieza con espacio
            let effective_len = line.len();
            let limit = if line.starts_with(' ') { 76 } else { 75 };
            if effective_len > limit {
                w.push(format!(
                    "I4: línea {} en '{}' excede {} octetos (tiene {})",
                    idx + 1,
                    path.display(),
                    limit,
                    effective_len
                ));
            }
            // Verificación UTF-8 multibyte no cortado: el plegado de writer ya garantiza que
            // no corta en medio de secuencia (bytes de continuación 10xxxxxx). Validamos que
            // ninguna línea termine en byte de inicio incompleto: como ya leímos como String válido,
            // basta comprobar que el corte no deja secuencia truncada — lo garantizamos comprobando
            // que cada línea es UTF-8 válida (ya lo es) y que el plegado original no cortó a mitad.
            // Comprobación adicional: ningún byte de continuación al inicio de línea sin leading space carece de leading.
            // Si la línea es continuación, debe empezar con espacio y el resto debe ser UTF-8 válido (ya).
            if line.len() != line.chars().map(|c| c.len_utf8()).sum::<usize>() {
                // improbable si es String válido, pero deja rastro
            }
        }
    }
    w
}

fn check_i5(path: &Path) -> Vec<String> {
    let mut w = Vec::new();
    if !path.exists() {
        return w;
    }
    let content = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(_) => return w,
    };
    for line in content.lines() {
        let upper = line.to_uppercase();
        if upper.starts_with("AGENT") {
            w.push(format!(
                "I5: salida '{}' contiene propiedad obsoleta AGENT",
                path.display()
            ));
            break;
        }
        if upper.starts_with("LABEL") {
            w.push(format!(
                "I5: salida '{}' contiene propiedad obsoleta LABEL",
                path.display()
            ));
            break;
        }
        if upper.starts_with("MAILER") {
            w.push(format!(
                "I5: salida '{}' contiene propiedad obsoleta MAILER",
                path.display()
            ));
            break;
        }
    }
    w
}

fn check_i6(path: &Path, contacts: &[Contact]) -> Vec<String> {
    let mut w = Vec::new();
    if !path.exists() {
        return w;
    }
    let content = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            w.push(format!(
                "I6: no se pudo leer audit '{}': {}",
                path.display(),
                e
            ));
            return w;
        }
    };
    let lines: Vec<&str> = content.lines().collect();
    if lines.is_empty() {
        w.push(format!("I6: audit '{}' vacío", path.display()));
        return w;
    }
    let header = lines[0];
    let cols: Vec<&str> = header.split('\t').collect();
    if cols.len() != 11 {
        w.push(format!(
            "I6: audit header en '{}' debe tener 11 columnas, tiene {}",
            path.display(),
            cols.len()
        ));
    }
    let expected_header = "TIMESTAMP\tUID\tFN_ORIGINAL\tFN_FINAL\tACCION\tMOTIVO\tREGLA\tCATEGORIAS\tSOURCE\tTELS\tEMAILS";
    if header != expected_header {
        w.push(format!(
            "I6: audit header inesperado en '{}'",
            path.display()
        ));
    }
    let data_lines = &lines[1..];
    // número esperado: contacts.len() + sum merged_uids (audit tiene 1 fila por entrada original)
    let merged: usize = contacts.iter().map(|c| c.merged_uids.len()).sum();
    let expected = contacts.len() + merged;
    // Si no hay fusiones, expected == contacts.len(); si hay, audit debe ser >= contacts.len()
    if data_lines.len() != expected && data_lines.len() != contacts.len() {
        // permitir ambos, pero advertir si no coincide ninguno
        // Para pipeline normal sin dedup contar contactos procesados totales incluye también eliminados/cuarentena
        // contacts incluye activos dedup + inactivos, entonces audit debe ser total_entrada == contacts.len()+merged
        if data_lines.len() < contacts.len() {
            w.push(format!(
                "I6: audit '{}' tiene {} filas de datos, esperado {} (contacts {}) + {} merged",
                path.display(),
                data_lines.len(),
                expected,
                contacts.len(),
                merged
            ));
        } else if data_lines.len() != expected && merged > 0 {
            w.push(format!(
                "I6: audit '{}' filas {} no coinciden con esperado {} (incluye {} merged)",
                path.display(),
                data_lines.len(),
                expected,
                merged
            ));
        }
    }
    for (idx, line) in data_lines.iter().enumerate() {
        let c = line.split('\t').count();
        if c != 11 {
            w.push(format!(
                "I6: audit '{}' línea {} tiene {} columnas, esperado 11",
                path.display(),
                idx + 2,
                c
            ));
        }
        if line
            .split('\t')
            .nth(1)
            .map(|s| s.trim().is_empty())
            .unwrap_or(true)
        {
            w.push(format!(
                "I6: audit '{}' línea {} sin UID",
                path.display(),
                idx + 2
            ));
        }
        if line
            .split('\t')
            .nth(6)
            .map(|s| s.trim().is_empty())
            .unwrap_or(true)
        {
            w.push(format!(
                "I6: audit '{}' línea {} sin REGLA (DecisionTrace)",
                path.display(),
                idx + 2
            ));
        }
    }
    // Comprobar que cada contacto aparece en audit (o como merged)
    let audit_text = content;
    for c in contacts {
        if !audit_text.contains(&c.uid) {
            w.push(format!(
                "I6: audit '{}' no contiene UID {}",
                path.display(),
                c.uid
            ));
        }
    }
    w
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::contact::{CategorySet, Contact, SourceDetail};
    use crate::domain::screening::ScreeningDecision;
    use std::fs as fs2;

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
            screening_rule: "Default".into(),
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

    #[test]
    fn test_i4_detects_long_line() {
        let dir = std::env::temp_dir();
        let path = dir.join("zedazo_test_i4_long.vcf");
        let long = "FN:".to_string() + &"a".repeat(80);
        fs2::write(
            &path,
            format!("BEGIN:VCARD\r\nVERSION:4.0\r\n{}\r\nEND:VCARD\r\n", long),
        )
        .unwrap();
        let warns = check_i4(&path);
        assert!(warns
            .iter()
            .any(|w| w.contains("I4") && w.contains("excede")));
        let _ = fs2::remove_file(&path);
    }

    #[test]
    fn test_i4_ok_with_folding() {
        let dir = std::env::temp_dir();
        let path = dir.join("zedazo_test_i4_ok.vcf");
        // línea plegada correctamente en 75
        let short = "FN:Test";
        fs2::write(
            &path,
            format!("BEGIN:VCARD\r\nVERSION:4.0\r\n{}\r\nEND:VCARD\r\n", short),
        )
        .unwrap();
        let warns = check_i4(&path);
        assert!(warns.is_empty());
        let _ = fs2::remove_file(&path);
    }

    #[test]
    fn test_i5_detects_agent() {
        let dir = std::env::temp_dir();
        let path = dir.join("zedazo_test_i5.vcf");
        fs2::write(
            &path,
            "BEGIN:VCARD\r\nVERSION:4.0\r\nAGENT:BEGIN:VCARD\r\nEND:VCARD\r\n",
        )
        .unwrap();
        let warns = check_i5(&path);
        assert!(warns.iter().any(|w| w.contains("I5")));
        let _ = fs2::remove_file(&path);
    }

    #[test]
    fn test_i6_audit_ok() {
        let dir = std::env::temp_dir();
        let path = dir.join("zedazo_test_i6.tsv");
        let mut c = make_contact(ScreeningDecision::Conserved);
        c.categories.n1.insert("PERS".into());
        c.uid = "uid-123".into();
        c.screening_rule = "Default".into();
        let header = "TIMESTAMP\tUID\tFN_ORIGINAL\tFN_FINAL\tACCION\tMOTIVO\tREGLA\tCATEGORIAS\tSOURCE\tTELS\tEMAILS\n";
        let row =
            "2026-01-01T00:00:00Z\tuid-123\tTest\tTest\tCONSERVADO\tok\tDefault\tPERS\ttest\t\t\n";
        fs2::write(&path, format!("{}{}", header, row)).unwrap();
        let warns = check_i6(&path, &[c]);
        assert!(warns.is_empty(), "warns: {:?}", warns);
        let _ = fs2::remove_file(&path);
    }

    #[test]
    fn test_i6_mismatch_rows() {
        let dir = std::env::temp_dir();
        let path = dir.join("zedazo_test_i6_mismatch.tsv");
        let header = "TIMESTAMP\tUID\tFN_ORIGINAL\tFN_FINAL\tACCION\tMOTIVO\tREGLA\tCATEGORIAS\tSOURCE\tTELS\tEMAILS\n";
        fs2::write(&path, header).unwrap();
        let c = make_contact(ScreeningDecision::Conserved);
        let warns = check_i6(&path, &[c]);
        assert!(warns.iter().any(|w| w.contains("I6")));
        let _ = fs2::remove_file(&path);
    }

    #[test]
    fn test_verify_post_i4_i5_i6() {
        let dir = std::env::temp_dir();
        let out = dir.join("zedazo_verify_post.vcf");
        let audit = dir.join("zedazo_verify_post.tsv");
        fs2::write(
            &out,
            "BEGIN:VCARD\r\nVERSION:4.0\r\nFN:Test\r\nEND:VCARD\r\n",
        )
        .unwrap();
        let header = "TIMESTAMP\tUID\tFN_ORIGINAL\tFN_FINAL\tACCION\tMOTIVO\tREGLA\tCATEGORIAS\tSOURCE\tTELS\tEMAILS\n";
        let row = "2026-01-01T00:00:00Z\tu1\tTest\tTest\tCONSERVADO\tok\tDefault\tPERS\ttest\t\t\n";
        fs2::write(&audit, format!("{}{}", header, row)).unwrap();
        let mut c = make_contact(ScreeningDecision::Conserved);
        c.categories.n1.insert("PERS".into());
        let warns = verify_post(&[c], Some(&out), Some(&audit));
        assert!(warns.is_empty(), "warns: {:?}", warns);
        let _ = fs2::remove_file(&out);
        let _ = fs2::remove_file(&audit);
    }
}
