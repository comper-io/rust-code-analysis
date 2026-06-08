use std::path::PathBuf;

use rust_code_analysis::{get_function_spaces, read_file_with_eol, LANG};

#[test]
fn example_pm_metrics() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/example.pm");
    let source = read_file_with_eol(&path).unwrap().unwrap();
    let func_space = get_function_spaces(&LANG::Perl, source, &path, None).unwrap();

    insta::assert_yaml_snapshot!(func_space, {
        ".spaces[].**.metrics.*.*" => insta::rounded_redaction(3),
        ".metrics.*.*" => insta::rounded_redaction(3),
        ".name" => "[filepath]",
    });
}
