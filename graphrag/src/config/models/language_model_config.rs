//! Language model configuration.

use tiktoken_rs;

use crate::config::enums::{AsyncType, AuthType, ModelType};
// use crate::config::errors::{
//     ApiKeyMissingError, AzureApiBaseMissingError, AzureApiVersionMissingError,
//     AzureDeploymentNameMissingError, ConflictingSettingsError,
// };
use crate::language_model::factory::ModelFactory;

/// Language model configuration.
pub struct LanguageModelConfig {
    /// The API key to use for the LLM service.
    pub api_key: Option<String>,

    /// The authentication type.
    pub auth_type: AuthType,

    /// The type of LLM model to use.
    pub r#type: ModelType, // TODO(rinarakaki) | String

    /// The LLM model to use.
    pub model: String,

    /// The encoding model to use
    pub encoding_model: String,

    /// The base URL for the LLM API.
    pub api_base: Option<String>,

    /// The version of the LLM API to use.
    pub api_version: Option<String>,

    /// The deployment name to use for the LLM service.
    pub deployment_name: Option<String>,

    /// The organization to use for the LLM service.
    pub organization: Option<String>,

    /// The proxy to use for the LLM service.
    pub proxy: Option<String>,

    /// Azure resource URI to use with managed identity for the llm connection.
    pub audience: Option<String>,

    /// Whether the model supports JSON output mode.
    pub model_supports_json: Option<bool>,

    /// The request timeout to use.
    pub request_timeout: f64,

    /// The number of tokens per minute to use for the LLM service.
    pub tokens_per_minute: usize,

    /// The number of requests per minute to use for the LLM service.
    pub requests_per_minute: usize,

    /// The retry strategy to use for the LLM service.
    pub retry_strategy: String,

    /// The maximum number of retries to use for the LLM service.
    pub max_retries: usize,

    /// The maximum retry wait to use for the LLM service.
    pub max_retry_wait: f64,

    /// Whether to use concurrent requests for the LLM service.
    pub concurrent_requests: usize,

    /// The async mode to use.
    pub async_mode: AsyncType,

    /// Static responses to use in mock mode.
    pub responses: Option<Vec<String>>,

    /// The maximum number of tokens to generate.
    pub max_tokens: usize,

    /// The temperature to use for token generation.
    pub temperature: f64,

    /// The top-p value to use for token generation.
    pub top_p: f64,

    /// The number of completions to generate.
    pub n: usize,

    /// The frequency penalty to use for token generation.
    pub frequency_penalty: f64,

    /// The presence penalty to use for token generation.
    pub presence_penalty: f64,
}

// impl Default for LanguageModelConfig {
//     /// Default values for language model.
//     fn default() -> Self {
//         LanguageModelConfig {
//             api_key: None,
//             auth_type: AuthType::APIKey,
//             // r#type: ?,
//             // model: ?,
//             encoding_model: "".into(),
//             api_base: None,
//             api_version: None,
//             deployment_name: None,
//             organization: None,
//             proxy: None,
//             audience: None,
//             model_supports_json: None,
//             request_timeout: 180.0,
//             tokens_per_minute: 50_000,
//             requests_per_minute: 1_000,
//             retry_strategy: "native".into(),
//             max_retries: 10,
//             max_retry_wait: 10.0,
//             concurrent_requests: 25,
//             async_mode: AsyncType::Threaded,
//             responses: None,
//             max_tokens: 4000,
//             temperature: 0.0,
//             top_p: 1.0,
//             n: 1,
//             frequency_penalty: 0.0,
//             presence_penalty: 0.0,
//         }
//     }
// }

impl LanguageModelConfig {
    //     /**
    //     Validate the API key.

    //     API Key is required when using OpenAI API
    //     or when using Azure API with API Key authentication.
    //     For the time being, this check is extra verbose for clarity.
    //     It will also raise an exception if an API Key is provided
    //     when one is not expected such as the case of using Azure
    //     Managed Identity.

    //     Raises
    //     ------
    //     ApiKeyMissingError
    //         If the API key is missing and is required.
    //     */
    //     fn _validate_api_key(self) {
    //         if self.auth_type == AuthType.APIKey && (
    //             self.api_key.is_none() or self.api_key.strip() == ""
    //         ) {
    //             raise ApiKeyMissingError(
    //                 self.type,
    //                 self.auth_type.value,
    //             )
    //         }

    //         if (self.auth_type == AuthType.AzureManagedIdentity) && (
    //             self.api_key is not None && self.api_key.strip() != ""
    //         ) {
    //             msg = "API Key should not be provided when using Azure Managed Identity. Please rerun `graphrag init` and remove the api_key when using Azure Managed Identity."
    //             raise ConflictingSettingsError(msg)
    //         }
    //     }

    //     fn _validate_auth_type(self) -> None:
    //         """Validate the authentication type.

    //         auth_type must be api_key when using OpenAI and
    //         can be either api_key or azure_managed_identity when using AOI.

    //         Raises
    //         ------
    //         ConflictingSettingsError
    //             If the Azure authentication type conflicts with the model being used.
    //         """
    //         if self.auth_type == AuthType.AzureManagedIdentity and (
    //             self.type == ModelType.OpenAIChat or self.type == ModelType.OpenAIEmbedding
    //         ):
    //             msg = format!("auth_type of azure_managed_identity is not supported for model type {self.type}. Please rerun `graphrag init` and set the auth_type to api_key.")
    //             raise ConflictingSettingsError(msg)

    //     fn _validate_type(self) -> None:
    //         """Validate the model type.

    //         Raises
    //         ------
    //         KeyError
    //             If the model name is not recognized.
    //         """
    //         # Type should be contained by the registered models
    //         if not ModelFactory.is_supported_model(self.type):
    //             msg = format!("Model type {self.type} is not recognized, must be one of {ModelFactory.get_chat_models() + ModelFactory.get_embedding_models()}.")
    //             raise KeyError(msg)

    /**
    Validate the encoding model.

    Raises
    ------
    KeyError
        If the model name is not recognized.
     */
    fn _validate_encoding_model(&mut self) {
        if self.encoding_model.trim() == "" {
            self.encoding_model = tiktoken_rs::get_bpe_from_model(&self.model);
        }
    }

    //     fn _validate_api_base(self) -> None:
    //         """Validate the API base.

    //         Required when using AOI.

    //         Raises
    //         ------
    //         AzureApiBaseMissingError
    //             If the API base is missing and is required.
    //         """
    //         if (
    //             self.type == ModelType.AzureOpenAIChat
    //             or self.type == ModelType.AzureOpenAIEmbedding
    //         ) and (self.api_base.is_none() or self.api_base.strip() == ""):
    //             raise AzureApiBaseMissingError(self.type)

    //     fn _validate_api_version(self) -> None:
    //         """Validate the API version.

    //         Required when using AOI.

    //         Raises
    //         ------
    //         AzureApiBaseMissingError
    //             If the API base is missing and is required.
    //         """
    //         if (
    //             self.type == ModelType.AzureOpenAIChat
    //             or self.type == ModelType.AzureOpenAIEmbedding
    //         ) and (self.api_version.is_none() or self.api_version.strip() == ""):
    //             raise AzureApiVersionMissingError(self.type)

    //             fn _validate_deployment_name(self) -> None:
    //         """Validate the deployment name.

    //         Required when using AOI.

    //         Raises
    //         ------
    //         AzureDeploymentNameMissingError
    //             If the deployment name is missing and is required.
    //         """
    //         if (
    //             self.type == ModelType.AzureOpenAIChat
    //             or self.type == ModelType.AzureOpenAIEmbedding
    //         ) and (self.deployment_name.is_none() or self.deployment_name.strip() == ""):
    //             raise AzureDeploymentNameMissingError(self.type)

    //     fn _validate_azure_settings(self) -> None:
    //         """Validate the Azure settings.

    //         Raises
    //         ------
    //         AzureApiBaseMissingError
    //             If the API base is missing and is required.
    //         AzureApiVersionMissingError
    //             If the API version is missing and is required.
    //         AzureDeploymentNameMissingError
    //             If the deployment name is missing and is required.
    //         """
    //         self._validate_api_base()
    //         self._validate_api_version()
    //         self._validate_deployment_name()

    //     @model_validator(mode="after")
    //     def _validate_model(self):
    //         self._validate_type()
    //         self._validate_auth_type()
    //         self._validate_api_key()
    //         self._validate_azure_settings()
    //         self._validate_encoding_model()
    //         return self
}
