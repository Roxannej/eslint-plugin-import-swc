use swc_common::FileName;
use swc_common::SourceMap;
use swc_ecma_parser::{lexer::Lexer, EsConfig, Parser, StringInput, Syntax};
use swc_ecma_transforms_testing::test_fixture;
use std::path::Path;
use std::sync::Arc;

use your_plugin::RemoveUnusedImports; // 假设你的插件在 your_plugin 模块中

#[test]
fn test_remove_unused_imports() {
    // 1. 定义输入和输出文件路径
    let input_file = Path::new("fixtures/input/test_file.tsx");
    let output_file = Path::new("fixtures/output/test_file.tsx");

    // 2. 通过 test_fixture 函数进行测试
    test_fixture(
        Syntax::Typescript(EsConfig {
            jsx: true, // 支持 TSX 文件
            ..Default::default()
        }),
        &RemoveUnusedImports,
        input_file,
        output_file,
    );
}
