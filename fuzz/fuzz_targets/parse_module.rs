#![no_main]

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;

use deno_ast::parse_module;
use deno_ast::MediaType;
use deno_ast::ParseParams;
use deno_ast::SourceTextInfo;

#[derive(Arbitrary, Debug, Clone)]
struct FuzzData {
    source_code: String,
    media_type_id: u8,
    capture_tokens: bool,
    scope_analysis: bool,
}

fn to_media_type(id: u8) -> MediaType {
    match id % 15 {
        0 => MediaType::JavaScript,
        1 => MediaType::Jsx,
        2 => MediaType::Mjs,
        3 => MediaType::Cjs,
        4 => MediaType::TypeScript,
        5 => MediaType::Mts,
        6 => MediaType::Cts,
        7 => MediaType::Dts,
        8 => MediaType::Dmts,
        9 => MediaType::Dcts,
        10 => MediaType::Tsx,
        11 => MediaType::Json,
        12 => MediaType::Wasm,
        13 => MediaType::TsBuildInfo,
        14 => MediaType::SourceMap,
        _ => MediaType::Unknown,
    }
}

fuzz_target!(|data: FuzzData| {
    let text_info = SourceTextInfo::new(data.source_code.into());
    let media_type = to_media_type(data.media_type_id);

    let parse_params = ParseParams {
        specifier: "file:///test.ts".to_string(),
        media_type,
        text_info,
        capture_tokens: data.capture_tokens,
        scope_analysis: data.scope_analysis,
        maybe_syntax: None,
    };

    let _ = parse_module(parse_params);
});