use std::path::Path;

use anyhow::Result;
use via_core::{codegen, parser};

#[test]
fn generates_expected_outputs_for_article_fixture() -> Result<()> {
    let fixture = Path::new("tests/fixtures/article.via");
    let resources = parser::parse_file(fixture)?;
    assert_eq!(resources.len(), 1);

    let generation = codegen::generate(&resources)?;

    let mut files = generation.files;
    files.sort_by(|a, b| a.relative_path.cmp(&b.relative_path));

    for file in files {
        let path_str = file.relative_path.to_string_lossy().replace('/', "__");
        let snapshot_name = format!("article__{}", path_str);
        insta::assert_snapshot!(snapshot_name, file.contents);
    }

    Ok(())
}
