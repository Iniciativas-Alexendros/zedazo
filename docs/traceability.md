# Matriz de Trazabilidad SPECS → Módulo → Test

**Generado automáticamente:** 2026-08-20 20:53 UTC

| Criterio SPECS | Módulo(s) | Tests Unitarios | Tests Integración | Estado |
|----------------|-----------|-----------------|-------------------|--------|
| **O1** Parsear VCF 4.0/3.0 | `infrastructure::parser`, `infrastructure::v3_compat`, `infrastructure::encoding`, `infrastructure::source` | `test_parse_single_vcard_4_0`, `test_parse_multi_vcard_4_0`, `test_parse_single_vcard_3_0`, `test_utf8_passthrough`, `test_iso_to_utf8`, `test_invalid_utf8_replacement`, `test_detect_proton_prodid`, `test_detect_google_prodid`, `test_detect_apple_prodid` | `test_cribar_proton_real`, `test_cribar_google_v3`, `test_cribar_apple_v3`, `test_encoding_iso` | ✅ |
| **O2** Cribado reglas E*/C* | `domain::screening`, `domain::rules` | `test_c2_juzgado_overrides_e1`, `test_e2_sanitize_but_preserve`, `test_e1_email_only_no_rescue`, `test_e3_huerfano`, `test_e4_quarantine`, `test_default_conserved`, `test_precedence_order`, `test_all_conserved_have_n1` | `test_cribar_proton_real`, `test_cribar_google_v3`, `test_cribar_apple_v3` | ⚠️ Parcial (faltan C1,C5,C7,E6) |
| **O3** Normalizar FN,N,TEL,ADR,ORG | `domain::normalization`, `domain::contact` | `test_structured_name_from_n`, `test_tel_e164`, `test_tel_non_normalizable`, `test_fn_no_at_sign` | `test_pipeline_completo` | ✅ |
| **O4** Clasificar taxonomía N1/N2/N3 | `domain::classification`, `domain::rules`, `infrastructure::config` | `test_juzgado_matched`, `test_domain_matched`, `test_multiple_categories`, `test_crypto_matched`, `test_no_match_defaults`, `test_all_rules_compile` | `test_cribar_proton_real`, `test_config_custom_rules` | ⚠️ N3 pendiente |
| **O5** Fusionar duplicados D1-D2 transitivo | `domain::identity` | `test_d1_same_uid`, `test_d2_same_fn_tel`, `test_d2_same_fn_email`, `test_d2_transitive`, `test_d2_transitive_complex`, `test_action_preserved_on_merge`, `test_no_duplicates`, `test_merge_preserves_most_complete_fn`, `test_merge_union_tels`, `test_merge_union_emails` | `test_dedup_transitivo_real` | ✅ |
| **O6** Auditoría TSV trazable | `domain::audit`, `infrastructure::tsv_writer` | (tests en `domain::audit` + `tsv_writer`) | `test_audit_tsv_completeness`, `test_pipeline_completo` | ✅ |
| **O7** Detectar fuente y versión | `infrastructure::source`, `infrastructure::parser` | `test_detect_proton_prodid`, `test_detect_google_prodid`, `test_detect_apple_prodid`, `test_detect_proton_autosave_uid`, `test_detect_proton_import_uid`, `test_detect_proton_web_uid`, `test_detect_unknown`, `test_parse_single_vcard_4_0`, `test_parse_single_vcard_3_0` | `test_cribar_proton_real`, `test_cribar_google_v3`, `test_cribar_apple_v3` | ✅ |
| **O8** Exportar CSV/JSON/VCF | `infrastructure::writer`, `infrastructure::csv_writer`, `infrastructure::json_writer` | `test_write_vcf_4_0_version`, `test_folding_75_octets`, `test_folding_no_multibyte_split`, `test_photo_roundtrip`, `test_v3_props_not_emitted` | `test_export_csv_roundtrip`, `test_export_json_roundtrip` | ✅ |
| **O9** Cargar reglas TOML | `infrastructure::config`, `domain::rules` | `test_all_rules_compile` | `test_config_custom_rules` | ✅ |
| **I1** Integridad contactos conservados | `domain::verification`, `domain::contact`, `domain::screening` | `test_verify_valid_conserved`, `test_verify_missing_n1` | `test_pipeline_completo` | ⚠️ Verificación no integrada en pipeline |
| **I2** FN canónico sin @ | `domain::verification`, `domain::normalization` | `test_verify_fn_with_at`, `test_fn_no_at_sign` | `test_pipeline_completo` | ⚠️ Verificación no integrada en pipeline |
| **I3** TEL E.164 o non_normalizable | `domain::verification`, `domain::normalization` | `test_tel_e164`, `test_tel_non_normalizable` | `test_pipeline_completo` | ⚠️ Verificación no integrada en pipeline |
| **I4** Salida VCF 4.0 canónica | `infrastructure::writer`, `domain::contact` | `test_write_vcf_4_0_version`, `test_folding_75_octets`, `test_folding_no_multibyte_split` | `test_pipeline_completo` | ✅ |
| **I5** Compatibilidad entrada 3.0 | `infrastructure::v3_compat`, `infrastructure::parser` | `test_parse_single_vcard_3_0`, `test_into_contact_drops_agent`, `test_v3_props_not_emitted` | `test_cribar_google_v3`, `test_cribar_apple_v3` | ✅ |
| **I6** Auditabilidad DecisionTrace | `domain::screening`, `domain::audit` | `test_decision_trace_immutable`, `test_all_conserved_have_n1` | `test_audit_tsv_completeness` | ✅ |
| **I7** No destrucción VCF original | `domain::verification`, `application::cribar` | `test_verify_input_equals_output` | (manual) | ⚠️ Verificación no integrada en pipeline |

---

## Leyenda
- ✅ = Implementado y testeado
- ⚠️ = Parcial o pendiente (ver comentarios)
- ❌ = No implementado

## Cobertura de tests por módulo

| Módulo | Tests unitarios (código) | Tests plan (doc) |
|--------|------------------------|------------------|
| `domain::contact` | 00 | (ver tabla arriba) |
| `domain::screening` | 16 | (ver tabla arriba) |
| `domain::classification` | 6 | (ver tabla arriba) |
| `domain::identity` | 7 | (ver tabla arriba) |
| `domain::normalization` | 19 | (ver tabla arriba) |
| `domain::rules` | 00 | (ver tabla arriba) |
| `domain::audit` | 00 | (ver tabla arriba) |
| `domain::verification` | 6 | (ver tabla arriba) |
| `infrastructure::parser` | 30 | (ver tabla arriba) |
| `infrastructure::writer` | 19 | (ver tabla arriba) |
| `infrastructure::encoding` | 4 | (ver tabla arriba) |
| `infrastructure::v3_compat` | 4 | (ver tabla arriba) |
| `infrastructure::source` | 9 | (ver tabla arriba) |
| `infrastructure::config` | 6 | (ver tabla arriba) |
| `infrastructure::csv_writer` | 2 | (ver tabla arriba) |
| `infrastructure::json_writer` | 2 | (ver tabla arriba) |
| `infrastructure::tsv_writer` | 3 | (ver tabla arriba) |
| `application::cribar` | 00 | (ver tabla arriba) |
| `application::audit` | 00 | (ver tabla arriba) |
| `application::stats` | 5 | (ver tabla arriba) |
| `tests/integration_test.rs` | 17 | 14 (plan) |

---
*Generado por `scripts/traceability.sh`* - actualizar tras cambios en SPECS/tests
