#![no_main]

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;

use deno_ast::parse_module;
use deno_ast::MediaType;
use deno_ast::ParseParams;
use deno_ast::SourceTextInfo;

/*
use deno_ast::parse_module;
use deno_ast::MediaType;
use deno_ast::ParseParams;
use deno_ast::SourceTextInfo;

let source_text = "class MyClass {}";
let text_info = SourceTextInfo::new(source_text.into());
let parsed_source = parse_module(ParseParams {
  specifier: "file:///my_file.ts".to_string(),
  media_type: MediaType::TypeScript,
  text_info,
  capture_tokens: true,
  maybe_syntax: None,
  scope_analysis: false,
}).expect("should parse");

// returns the comments
parsed_source.comments();
// returns the tokens if captured
parsed_source.tokens();
// returns the module (AST)
parsed_source.module();
// returns the `SourceTextInfo`
parsed_source.text_info();
 */

fuzz_target!(|data: &[u8]| {
    let source_text = String::from_utf8_lossy(data);
    let text_info = SourceTextInfo::new(source_text.into());

    let parse_params = ParseParams {
        specifier: "file:///my_file.ts".to_string(),
        media_type: MediaType::TypeScript,
        text_info,
        capture_tokens: true,
        maybe_syntax: None,
        scope_analysis: false,
    };

    let _ = parse_module(parse_params);
});