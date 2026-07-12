#![no_main]

use libfuzzer_sys::fuzz_target;

use deno_ast::parse_module;
use deno_ast::MediaType;
use deno_ast::ModuleSpecifier;
use deno_ast::ParseParams;

fuzz_target!(|data: &[u8]| {
    let source_text = String::from_utf8_lossy(data);

    let parse_params = ParseParams {
        specifier: ModuleSpecifier::parse("file:///my_file.ts").unwrap(),
        media_type: MediaType::TypeScript,
        text: source_text.as_ref().into(),
        capture_tokens: true,
        maybe_syntax: None,
        scope_analysis: false,
    };

    let _ = parse_module(parse_params);
});
