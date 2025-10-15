use xml_structer::validate;

#[test]
fn test_valid_tei_golden_file() {
    let report = validate("tests/valid_tei.xml").unwrap();
    assert!(report.is_valid(), "Valid TEI file should pass validation");
    assert_eq!(
        report.errors.len(),
        0,
        "Valid TEI file should have no errors"
    );
    assert_eq!(
        report.warnings.len(),
        0,
        "Valid TEI file should have no warnings"
    );
}

#[test]
fn test_invalid_tei_golden_file() {
    let report = validate("tests/invalid_tei.xml").unwrap();
    assert!(
        !report.is_valid(),
        "Invalid TEI file should fail validation"
    );

    // Should have at least 3 errors:
    // 1. Wrong root element (warning)
    // 2. <pb> missing @ed (error)
    // 3. <pb> missing @n (error)
    // 4. <head> outside <div> (warning)
    let error_count = report.errors.len();
    let warning_count = report.warnings.len();

    assert!(
        error_count >= 2,
        "Should have at least 2 errors for <pb> missing attributes"
    );
    assert!(
        warning_count >= 1,
        "Should have at least 1 warning for wrong root or head outside div"
    );

    // Check for specific error patterns
    let has_pb_errors = report
        .errors
        .iter()
        .any(|e| e.text.contains("<pb> missing"));
    assert!(has_pb_errors, "Should have <pb> missing attribute errors");

    let has_wrong_root = report.warnings.iter().any(|w| w.text.contains("Root is"));
    let has_head_warning = report
        .warnings
        .iter()
        .any(|w| w.text.contains("<head> should be inside <div>"));

    assert!(
        has_wrong_root || has_head_warning,
        "Should have root or head warnings"
    );
}
