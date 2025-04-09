//! LocalSearch implementation.

use log;
use tiktoken_rs;

use crate::callbacks::query_callbacks::QueryCallbacks;
use crate::language_model::protocol::base::ChatModel;
use crate::prompts::query::local_search_system_prompt::LOCAL_SEARCH_SYSTEM_PROMPT;
use crate::query::context_builder::builders::LocalContextBuilder;
use crate::query::context_builder::conversation_history::ConversationHistory;
use crate::query::llm::text_utils::num_tokens;
use crate::query::structured_search::base::{BaseSearch, SearchResult};

DEFAULT_LLM_PARAMS = {
    "max_tokens": 1500,
    "temperature": 0.0,
}

/// Search orchestration for local search mode.
pub struct LocalSearch(BaseSearch<LocalContextBuilder>) {

}

impl LocalSearch {
    pub fn new(
        model: ChatModel,
        context_builder: LocalContextBuilder,
        token_encoder: Option<tiktoken_rs::Encoding>,
        system_prompt: Option<String> /* = None */,
        response_type: str = "multiple paragraphs",
        callbacks: Vec<QueryCallbacks> | None = None,
        model_params: HashMap<String, Box<dyn Any>> = DEFAULT_LLM_PARAMS,
        context_builder_params: Option<HashMap>, /* = None */,
    ):
        super().__init__(
            model=model,
            context_builder=context_builder,
            token_encoder=token_encoder,
            model_params=model_params,
            context_builder_params=context_builder_params or {},
        )
        self.system_prompt = system_prompt or LOCAL_SEARCH_SYSTEM_PROMPT
        self.callbacks = callbacks or []
        self.response_type = response_type

    /// Build local search context that fits a single context window and generate answer for the user query.
    pub async fn search(
        self,
        query: str,
        conversation_history: ConversationHistory | None = None,
        **kwargs,
    ) -> SearchResult {
        start_time = time.time()
        search_prompt = ""
        llm_calls, prompt_tokens, output_tokens = {}, {}, {}
        context_result = self.context_builder.build_context(
            query=query,
            conversation_history=conversation_history,
            **kwargs,
            **self.context_builder_params,
        )
        llm_calls["build_context"] = context_result.llm_calls
        prompt_tokens["build_context"] = context_result.prompt_tokens
        output_tokens["build_context"] = context_result.output_tokens

        info!("GENERATE ANSWER: %s. QUERY: %s", start_time, query)
        try:
            if "drift_query" in kwargs:
                drift_query = kwargs["drift_query"]
                search_prompt = self.system_prompt.format(
                    context_data=context_result.context_chunks,
                    response_type=self.response_type,
                    global_query=drift_query,
                )
            else:
                search_prompt = self.system_prompt.format(
                    context_data=context_result.context_chunks,
                    response_type=self.response_type,
                )
            history_messages = [
                {"role": "system", "content": search_prompt},
            ]

            full_response = ""

            async for response in self.model.achat_stream(
                prompt=query,
                history=history_messages,
                model_parameters=self.model_params,
            ):
                full_response += response
                for callback in self.callbacks:
                    callback.on_llm_new_token(response)

            llm_calls["response"] = 1
            prompt_tokens["response"] = num_tokens(search_prompt, self.token_encoder)
            output_tokens["response"] = num_tokens(full_response, self.token_encoder)

            for callback in self.callbacks{
                callback.on_context(context_result.context_records)
            }

            return SearchResult(
                response=full_response,
                context_data=context_result.context_records,
                context_text=context_result.context_chunks,
                completion_time=time.time() - start_time,
                llm_calls=sum(llm_calls.values()),
                prompt_tokens=sum(prompt_tokens.values()),
                output_tokens=sum(output_tokens.values()),
                llm_calls_categories=llm_calls,
                prompt_tokens_categories=prompt_tokens,
                output_tokens_categories=output_tokens,
            )

        except Exception:
            log.exception("Exception in _asearch")
            return SearchResult(
                response="",
                context_data=context_result.context_records,
                context_text=context_result.context_chunks,
                completion_time=time.time() - start_time,
                llm_calls=1,
                prompt_tokens=num_tokens(search_prompt, self.token_encoder),
                output_tokens=0,
            )
    }

    /// Build local search context that fits a single context window and generate answer for the user query.
    pub async fn stream_search(
        self,
        query: str,
        conversation_history: ConversationHistory | None = None,
    ) -> AsyncGenerator {
        start_time = time.time()

        context_result = self.context_builder.build_context(
            query=query,
            conversation_history=conversation_history,
            **self.context_builder_params,
        )
        info!("GENERATE ANSWER: %s. QUERY: %s", start_time, query)
        search_prompt = self.system_prompt.format(
            context_data=context_result.context_chunks, response_type=self.response_type
        )
        history_messages = [
            {"role": "system", "content": search_prompt},
        ]

        for callback in self.callbacks:
            callback.on_context(context_result.context_records)

        async for response in self.model.achat_stream(
            prompt=query,
            history=history_messages,
            model_parameters=self.model_params,
        ):
            for callback in self.callbacks:
                callback.on_llm_new_token(response)
            yield response
    }
}
