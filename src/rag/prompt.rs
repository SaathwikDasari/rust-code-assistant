use crate::search::similarity::SearchResult;

pub fn build_prompt(query: &str, results: &[SearchResult]) -> String {
    let mut context = String::new();

    for result in results {
        context.push_str(&format!(
            "// File: {} | fn {} (lines {}-{})\n{}\n\n",
            result.file_path, result.fn_name, result.start_line, result.end_line, result.content
        ));
    }

    format!(
        "You are an expert Rust developer assistant helping someone understand a Rust codebase.

Using ONLY the following code snippets as context, answer the question clearly and concisely.
If the answer is not in the provided code, say so.

<code>
{}
</code>

Question: {}

Answer:",
        context, query
    )
}
