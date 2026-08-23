use malarky::domain_contract::{SourceSpan, Utf8SourceMap};

fn main() {
    let source_map = Utf8SourceMap::new("text");
    let Some(span) = SourceSpan::new(&source_map, 0, 4) else {
        return;
    };
    let _start = span.start;
}
