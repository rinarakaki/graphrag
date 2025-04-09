/// Base llm response protocol.

/// Protocol for Model response's output object.
pub trait ModelOutput {
    @property
    /// Return the textual content of the output.
    fn content(self) -> str;
}

/// Protocol for LLM response.
pub trait ModelResponse<T> {
    @property
    /// Return the output of the response.
    fn output(self) -> ModelOutput;

    @property
    /// Return the parsed response.
    fn parsed_response(self) -> T | None;

    @property
    /// Return the history of the response.
    fn history(self) -> list;
}


/// Base class for LLM output.
pub struct BaseModelOutput {
    /// The textual content of the output.
    content: String,
}

/// Base class for a Model response.
pub struct BaseModelResponse<T> {
    ///
    output: BaseModelOutput,
    /// Parsed response.
    parsed_response: Option<T>,
    /// History of the response.
    history: Vec<Any> = Field(default_factory=list)
    /// Tool calls required by the Model. These will be instances of the LLM tools (with filled parameters).
    tool_calls: list = Field(default_factory=list)
    /// Request/response metrics.
    metrics: Any | None = None
    /// Whether the response was a cache hit.
    cache_hit: bool | None = None
}
