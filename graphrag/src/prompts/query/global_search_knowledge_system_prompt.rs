//! Global Search system prompts.

pub const GENERAL_KNOWLEDGE_INSTRUCTION: &str = r#"
The response may also include relevant real-world knowledge outside the dataset, but it must be explicitly annotated with a verification tag [LLM: verify]. For example:
"This is an example sentence supported by real-world knowledge [LLM: verify]."
"#;
