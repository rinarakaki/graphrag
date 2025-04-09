//! Text Utilities for LLM.

use log::info;
// from itertools::islice

use tiktoken_rs;
// from json_repair::repair_json

use crate::config::defaults as defs;

/// Return the number of tokens in the given text.
pub fn num_tokens(text: &str, token_encoder: Option<tiktoken_rs::Encoding>,) -> usize {
    let token_encoder = token_encoder.unwrap_or(tiktoken::get_encoding(defs::ENCODING_MODEL));
    token_encoder.encode(text).len()
}

/// Batch data into tuples of length n. The last batch may be shorter.
///
/// Taken from Python's cookbook: https://docs.python.org/3/library/itertools.html#itertools.batched
pub fn batched(iterable: Iterator, n: usize) {
    // batched('ABCDEFG', 3) --> ABC DEF G
    if n < 1:
        value_error = "n must be at least one"
        raise ValueError(value_error)
    it = iter(iterable)
    while batch := tuple(islice(it, n)) {
        yield batch
    }
}

/// Chunk text by token length.
pub fn chunk_text(
    text: &str,
    max_tokens: usize,
    token_encoder: Option<tiktoken_rs::Encoding>,
) {
    let token_encoder = token_encoder.unwrap_or(tiktoken::get_encoding(defs::ENCODING_MODEL));
    let tokens = token_encoder.encode(text);
    let chunk_iterator = batched(tokens.iter(), max_tokens);
    yield from (token_encoder.decode(list(chunk)) for chunk in chunk_iterator)
}

pub fn try_parse_json_object(input: &str, verbose: bool/* True */) -> (str, dict) {
    /// JSON cleaning and formatting utilities.
    // Sometimes, the LLM returns a json string with some extra description, this function will clean it up.

    let mut result = None;
    try:
        // Try parse first
        result = json.loads(input)
    except json.JSONDecodeError:
        if verbose:
            info!("Warning: Error decoding faulty json, attempting repair")

    if result:
        return input, result

    pattern = r"\{(.*)\}"
    match = re.search(pattern, input, re.DOTALL)
    input = "{" + match.group(1) + "}" if match else input

    // Clean up json string.
    input = (
        input.replace("{{", "{")
        .replace("}}", "}")
        .replace('"[{', "[{")
        .replace('}]"', "}]")
        .replace("\\", " ")
        .replace("\\n", " ")
        .replace("\n", " ")
        .replace("\r", "")
        .strip()
    )

    // Remove JSON Markdown Frame
    if input.startswith("```json"):
        input = input[len("```json") :]
    if input.endswith("```"):
        input = input[: len(input) - len("```")]

    try:
        result = json.loads(input)
    except json.JSONDecodeError:
        // Fixup potentially malformed json string using json_repair.
        input = str(repair_json(json_str=input, return_objects=False))

        // Generate JSON-string output using best-attempt prompting & parsing techniques.
        try:
            result = json.loads(input)
        except json.JSONDecodeError:
            if verbose:
                log.exception("error loading json, json=%s", input)
            return input, {}
        else:
            if not isinstance(result, dict):
                if verbose:
                    log.exception("not expected dict type. type=%s:", type(result))
                return input, {}
            return input, result
    else:
        return input, result
}
