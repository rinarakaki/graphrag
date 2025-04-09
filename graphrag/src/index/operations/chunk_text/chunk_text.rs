//! A module containing _get_num_total, chunk, run_strategy and load_strategy methods definitions.

use polars::prelude::{LazyFrame, Series};

use crate::callbacks::workflow_callbacks::WorkflowCallbacks;
use crate::config::models::chunking_config::{ChunkStrategyType, ChunkingConfig};
use crate::index::operations::chunk_text::typing::{ChunkInput, ChunkStrategy};
use crate::logger::progress::{ProgressTicker, progress_ticker};

/**
Chunk a piece of text into smaller pieces.

## Usage
```yaml
args:
    column: <column name> # The name of the column containing the text to chunk, this can either be a column with text, or a column with a list[tuple[doc_id, str]]
    strategy: <strategy config> # The strategy to use to chunk the text, see below for more details
```

## Strategies
The text chunk verb uses a strategy to chunk the text. The strategy is an object which defines the strategy to use. The following strategies are available:

### tokens
This strategy uses the [tokens] library to chunk a piece of text. The strategy config is as follows:

```yaml
strategy: tokens
size: 1200 # Optional, The chunk size to use, default: 1200
overlap: 100 # Optional, The chunk overlap to use, default: 100
```

### sentence
This strategy uses the nltk library to chunk a piece of text into sentences. The strategy config is as follows:

```yaml
strategy: sentence
```
*/
pub fn chunk_text(
    input: LazyFrame,
    column: &str,
    size: usize,
    overlap: usize,
    encoding_model: String,
    strategy: ChunkStrategyType,
    callbacks: impl WorkflowCallbacks,
) -> Series {
    let strategy_exec = load_strategy(strategy);

    let num_total = _get_num_total(input, column);
    let tick = progress_ticker(Some(callbacks.progress), num_total);

    // collapse the config back to a single object to support "polymorphic" function call
    let config = ChunkingConfig {
        size,
        overlap,
        encoding_model,
        ..Default::default()
    };

    return cast(
        "Series",
        input.apply(
            cast(
                "Any",
                |x| run_strategy(
                    strategy_exec,
                    x[column],
                    config,
                    tick,
                ),
            ),
            axis=1,
        ),
    )
}

// def run_strategy(
//     strategy_exec: ChunkStrategy,
//     input: ChunkInput,
//     config: ChunkingConfig,
//     tick: ProgressTicker,
// ) -> list[str | tuple[Vec<String> | None, str, int]]:
//     /// Run strategy method definition.
//     if isinstance(input, str):
//         return [item.text_chunk for item in strategy_exec([input], config, tick)]

//     # We can work with both just a list of text content
//     # or a list of tuples of (document_id, text content)
//     # text_to_chunk = '''
//     texts = [item if isinstance(item, str) else item[1] for item in input]

//     strategy_results = strategy_exec(texts, config, tick)

//     results = []
//     for strategy_result in strategy_results:
//         doc_indices = strategy_result.source_doc_indices
//         if isinstance(input[doc_indices[0]], str):
//             results.push(strategy_result.text_chunk)
//         else:
//             doc_ids = [input[doc_idx][0] for doc_idx in doc_indices]
//             results.push((
//                 doc_ids,
//                 strategy_result.text_chunk,
//                 strategy_result.n_tokens,
//             ))
//     return results

/// Load strategy method definition.
pub fn load_strategy(strategy: ChunkStrategyType) -> ChunkStrategy {
    match strategy {
        ChunkStrategyType::Tokens => {
            use crate::index::operations::chunk_text::strategies::run_tokens;

            return run_tokens
        }
        ChunkStrategyType::Sentence => {
            // NLTK
            use crate::index::operations::chunk_text::bootstrap::bootstrap;
            use crate::index::operations::chunk_text::strategies::run_sentences;

            bootstrap();
            return run_sentences
        }
        _ => {
            let msg = format!("Unknown strategy: {strategy}");
            raise ValueError(msg)
        }
    }
}

fn _get_num_total(output: LazyFrame, column: &str) -> usize {
    let mut num_total = 0;
    for row in output[column] {
        if isinstance(row, str) {
            num_total += 1;
        } else {
            num_total += row.len();
        }
    }
    num_total
}
