use syn::spanned::Spanned;

#[derive(Debug)]
pub struct CodeChunk {
    pub file_path: String,
    pub fn_name: String,
    pub start_line: usize,
    pub end_line: usize,
    pub content: String,
}

pub fn chunk_file(path: &str, source: &str) -> Vec<CodeChunk> {
    let mut chunks = Vec::new();

    let parsed = match syn::parse_file(source) {
        Ok(parsed) => parsed,
        Err(_) => return chunks,
    };

    let lines: Vec<&str> = source.lines().collect();

    for item in parsed.items {
        if let syn::Item::Fn(func) = item {
            let span = func.span();
            let start = span.start().line;
            let end = span.end().line;

            let content = lines[start - 1..end].join("\n");

            chunks.push(CodeChunk {
                file_path: path.to_string(),
                fn_name: func.sig.ident.to_string(),
                start_line: start,
                end_line: end,
                content,
            });
        }
    }

    return chunks;
}
