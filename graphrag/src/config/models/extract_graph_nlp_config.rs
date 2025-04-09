//! Parameterization settings for the default configuration.

use std::collections::HashMap;

use crate::config::enums::NounPhraseExtractorType;

/// Configuration section for NLP text analyzer.
pub struct TextAnalyzerConfig {
    /// The noun phrase extractor type.
    pub extractor_type: NounPhraseExtractorType,

    /// The SpaCy model name.
    pub model_name: String,

    /// The max word length for NLP parsing.
    pub max_word_length: usize,

    /// The delimiter for splitting words.
    pub word_delimiter: String,

    /// Whether to include named entities in noun phrases.
    pub include_named_entities: bool,

    /// The list of excluded nouns (i.e., stopwords). If None, will use a default stopword list
    pub exclude_nouns: Option<Vec<String>>,

    /// The list of named entity tags to exclude in noun phrases.
    pub exclude_entity_tags: Vec<String>,

    /// The list of part-of-speech tags to remove in noun phrases.
    pub exclude_pos_tags: Vec<String>,

    /// The list of noun phrase tags.
    pub noun_phrase_tags: Vec<String>,

    /// The CFG for matching noun phrases. The key is a tuple of POS tags and the value is the grammar.
    pub noun_phrase_grammars: HashMap<String, String>,
}

impl Default for TextAnalyzerConfig {
    /// Default values for text analyzer.
    fn default() -> Self {
        TextAnalyzerConfig {
            extractor_type: NounPhraseExtractorType::RegexEnglish,
            model_name: "en_core_web_md".into(),
            max_word_length: 15,
            word_delimiter: " ".into(),
            include_named_entities: true,
            exclude_nouns: None,
            exclude_entity_tags: vec!["DATE".into()],
            exclude_pos_tags: vec!["DET".into(), "PRON".into(), "INTJ".into(), "X".into()],
            noun_phrase_tags: vec!["PROPN".into(), "NOUNS".into()],
            noun_phrase_grammars: HashMap::from([
                ("PROPN,PROPN".into(), "PROPN".into()),
                ("NOUN,NOUN".into(), "NOUNS".into()),
                ("NOUNS,NOUN".into(), "NOUNS".into()),
                ("ADJ,ADJ".into(), "ADJ".into()),
                ("ADJ,NOUN".into(), "NOUNS".into()),
            ]),
        }
    }
}

/// Configuration section for graph extraction via NLP.
pub struct ExtractGraphNLPConfig {
    /// Whether to normalize edge weights.
    pub normalize_edge_weights: bool,

    /// The text analyzer configuration.
    pub text_analyzer: TextAnalyzerConfig,

    /// The number of threads to use for the extraction process.
    pub concurrent_requests: usize,
}

impl Default for ExtractGraphNLPConfig {
    /// Default values for NLP graph extraction.
    fn default() -> Self {
        ExtractGraphNLPConfig {
            normalize_edge_weights: true,
            text_analyzer: TextAnalyzerConfig::default(),
            concurrent_requests: 25,
        }
    }
}
