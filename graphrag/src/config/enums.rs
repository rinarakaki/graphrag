//! A module containing config enums.

/// The cache configuration type for the pipeline.
pub enum CacheType {
    /// The file cache configuration type.
    File,
    /// The memory cache configuration type.
    Memory,
    /// The none cache configuration type.
    None,
    /// The blob cache configuration type.
    Blob,
    /// The cosmosdb cache configuration type
    Cosmosdb,
}

impl CacheType {
    pub fn as_str(&self) -> &str {
        match self {
            CacheType::File => "file",
            CacheType::Memory => "memory",
            CacheType::None => "none",
            CacheType::Blob => "blob",
            CacheType::Cosmosdb => "cosmosdb",
        }
    }
}

impl std::fmt::Debug for CacheType {
    /// Get a string representation.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// The input file type for the pipeline.
pub enum InputFileType {
    /// The CSV input type.
    Csv,
    /// The text input type.
    Text,
    /// The JSON input type.
    Json,
}

impl InputFileType {
    pub fn as_str(&self) -> &str {
        match self {
            InputFileType::Csv => "csv",
            InputFileType::Text => "text",
            InputFileType::Json => "json",
        }
    }
}

impl std::fmt::Debug for InputFileType {
    /// Get a string representation.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// The input type for the pipeline.
pub enum InputType {
    /// The file storage type.
    File,
    /// The blob storage type.
    Blob,
}

impl InputType {
    pub fn as_str(&self) -> &str {
        match self {
            InputType::File => "file",
            InputType::Blob => "blob",
        }
    }
}

impl std::fmt::Debug for InputType {
    /// Get a string representation.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// The output type for the pipeline.
pub enum OutputType {
    /// The file output type.
    File,
    /// The memory output type.
    Memory,
    /// The blob output type.
    Blob,
    /// The cosmosdb output type
    Cosmosdb,
}

impl OutputType {
    pub fn as_str(&self) -> &str {
        match self {
            OutputType::File => "file",
            OutputType::Memory => "memory",
            OutputType::Blob => "blob",
            OutputType::Cosmosdb => "cosmosdb",
        }
    }
}

impl std::fmt::Debug for OutputType {
    /// Get a string representation.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// The reporting configuration type for the pipeline.
pub enum ReportingType {
    /// The file reporting configuration type.
    File,
    /// The console reporting configuration type.
    Console,
    /// The blob reporting configuration type.
    Blob,
}

impl ReportingType {
    pub fn as_str(&self) -> &str {
        match self {
            ReportingType::File => "file",
            ReportingType::Console => "console",
            ReportingType::Blob => "blob",
        }
    }
}

impl std::fmt::Debug for ReportingType {
    /// Get a string representation.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// The target to use for text embeddings.
pub enum TextEmbeddingTarget {
    All,
    Required,
    Selected,
    None,
}

impl TextEmbeddingTarget {
    pub fn as_str(&self) -> &str {
        match self {
            TextEmbeddingTarget::All => "all",
            TextEmbeddingTarget::Required => "required",
            TextEmbeddingTarget::Selected => "selected",
            TextEmbeddingTarget::None => "none",
        }
    }
}

impl std::fmt::Debug for TextEmbeddingTarget {
    /// Get a string representation.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// LLMType enum class definition.
pub enum ModelType {
    // Embeddings
    OpenAIEmbedding,
    AzureOpenAIEmbedding,

    // Chat Completion
    OpenAIChat,
    AzureOpenAIChat,

    // Debug
    MockChat,
    MockEmbedding,
}

impl ModelType {
    pub fn as_str(&self) -> &str {
        match self {
            ModelType::OpenAIEmbedding => "openai_embedding",
            ModelType::AzureOpenAIEmbedding => "azure_openai_embedding",
            ModelType::OpenAIChat => "openai_chat",
            ModelType::AzureOpenAIChat => "azure_openai_chat",
            ModelType::MockChat => "mock_chat",
            ModelType::MockEmbedding => "mock_embedding",
        }
    }
}


impl std::fmt::Debug for ModelType {
    /// Get a string representation.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// AuthType enum class definition.
pub enum AuthType {
    APIKey,
    AzureManagedIdentity,
}

impl AuthType {
    pub fn as_str(&self) -> &str {
        match self {
            AuthType::APIKey => "api_key",
            AuthType::AzureManagedIdentity => "azure_managed_identity",
        }
    }
}

/// Enum for the type of async to use.
pub enum AsyncType {
    AsyncIO,
    Threaded,
}

impl AsyncType {
    pub fn as_str(&self) -> &str {
        match self {
            AsyncType::AsyncIO => "asyncio",
            AsyncType::Threaded => "threaded",
        }
    }
}

/// ChunkStrategy class definition.
pub enum ChunkStrategyType {
    Tokens,
    Sentence,
}

impl ChunkStrategyType {
    pub fn as_str(&self) -> &str {
        match self {
            ChunkStrategyType::Tokens => "tokens",
            ChunkStrategyType::Sentence => "sentence",
        }
    }
}

impl std::fmt::Debug for ChunkStrategyType {
    /// Get a string representation.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// The type of search to run.
pub enum SearchMethod {
    Local,
    Global,
    Drift,
    Basic,
}

impl SearchMethod {
    pub fn as_str(&self) -> &str {
        match self {
            SearchMethod::Local => "local",
            SearchMethod::Global => "global",
            SearchMethod::Drift => "drift",
            SearchMethod::Basic => "basic",
        }
    }
}

impl std::fmt::Debug for SearchMethod {
    /// Return the string representation of the enum value.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Enum for the type of indexing to perform.
pub enum IndexingMethod {
    /// Traditional GraphRAG indexing, with all graph construction and summarization performed by a language model.
    Standard,
    /// Fast indexing, using NLP for graph construction and language model for summarization.
    Fast,
}

impl IndexingMethod {
    pub fn as_str(&self) -> &str {
        match self {
            IndexingMethod::Standard => "standard",
            IndexingMethod::Fast => "fast",
        }
    }
}

/// Enum for the noun phrase extractor options.
pub enum NounPhraseExtractorType {
    /// Standard extractor using regex. Fastest, but limited to English.
    RegexEnglish,
    /// Noun phrase extractor based on dependency parsing and NER using SpaCy.
    Syntactic,
    /// Noun phrase extractor combining CFG-based noun-chunk extraction and NER.
    Cfg,
}

impl NounPhraseExtractorType {
    pub fn as_str(&self) -> &str {
        match self {
            NounPhraseExtractorType::RegexEnglish => "regex_english",
            NounPhraseExtractorType::Syntactic => "syntactic_parser",
            NounPhraseExtractorType::Cfg => "cfg",
        }
    }
}
