use plugin_react_const_elements::react_const_elements;
use std::path::PathBuf;
use swc_core::{
    common::{chain, Mark},
    ecma::{
        parser::{EsConfig, Syntax},
        transforms::{base::resolver, testing::test_fixture},
        visit::as_folder,
    },
};

#[testing::fixture("tests/fixtures/**/input.js")]
#[testing::fixture("tests/fixtures/**/input.mjs")]
fn fixture(input: PathBuf) {
    let output = {
        let path = input.parent().unwrap().join("output.js");
        if !path.exists() {
            input.parent().unwrap().join("output.mjs")
        } else {
            path
        }
    };

    test_fixture(
        Syntax::Es(EsConfig {
            jsx: true,
            ..Default::default()
        }),
        &|_| {
            let unresolved_mark = Mark::new();
            let top_level_mark = Mark::new();

            as_folder(chain!(
                resolver(unresolved_mark, top_level_mark, false),
                react_const_elements()
            ))
        },
        &input,
        &output,
        Default::default(),
    );
}
