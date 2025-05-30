//! Contains algorithms to build context data for global search prompt.

use tiktoken_rs;

use crate::data_model::community::Community;
use crate::data_model::community_report::CommunityReport;
use crate::data_model::entity::Entity;
use crate::query::context_builder::builders::ContextBuilderResult;
use crate::query::context_builder::community_context::build_community_context;
use crate::query::context_builder::conversation_history::ConversationHistory;
use crate::query::context_builder::dynamic_community_selection::DynamicCommunitySelection;
use crate::query::structured_search::base::GlobalContextBuilder;

/// GlobalSearch community context builder.
pub struct GlobalCommunityContext {
    community_reports: Vec<CommunityReport>,
    entities: Option<Vec<Entity>>,
    token_encoder: Option<tiktoken_rs::Encoding>,
    dynamic_community_selection: Option<DynamicCommunitySelection>,
    random_state: usize,
}

// (GlobalContextBuilder)
impl GlobalCommunityContext {
    pub fn new(
        community_reports: Vec<CommunityReport>,
        communities: Vec<Community>,
        entities: Option<Vec<Entity>>, // = None,
        token_encoder: Option<tiktoken_rs::Encoding>, // = None,
        dynamic_community_selection: bool, // = False,
        dynamic_community_selection_kwargs: HashMap<String, Box<dyn Any>> | None = None,
        random_state: usize, // = 86,
    ) -> Self {
        GlobalCommunityContext {

        }
        self.community_reports = community_reports
        self.entities = entities
        self.token_encoder = token_encoder
        self.dynamic_community_selection = None
        if dynamic_community_selection and isinstance(
            dynamic_community_selection_kwargs, dict
        ):
            self.dynamic_community_selection = DynamicCommunitySelection(
                community_reports=community_reports,
                communities=communities,
                model=dynamic_community_selection_kwargs.pop("model"),
                token_encoder=dynamic_community_selection_kwargs.pop("token_encoder"),
                **dynamic_community_selection_kwargs,
            )
        self.random_state = random_state
    }

    pub async fn build_context(
        &self,
        query: str,
        conversation_history: ConversationHistory | None = None,
        use_community_summary: bool = True,
        column_delimiter: str = "|",
        shuffle_data: bool = True,
        include_community_rank: bool = False,
        min_community_rank: int = 0,
        community_rank_name: str = "rank",
        include_community_weight: bool = True,
        community_weight_name: str = "occurrence",
        normalize_community_weight: bool = True,
        max_tokens: int = 8000,
        context_name: str = "Reports",
        conversation_history_user_turns_only: bool = True,
        conversation_history_max_turns: int | None = 5,
        **kwargs: Any,
    ) -> ContextBuilderResult {
        /// Prepare batches of community report data table as context data for global search.
        let conversation_history_context = "";
        let final_context_data = {};
        let (llm_calls, prompt_tokens, output_tokens) = (0, 0, 0);
        if conversation_history {
            // build conversation history context
            (
                conversation_history_context,
                conversation_history_context_data,
            ) = conversation_history.build_context(
                include_user_turns_only=conversation_history_user_turns_only,
                max_qa_turns=conversation_history_max_turns,
                column_delimiter=column_delimiter,
                max_tokens=max_tokens,
                recency_bias=False,
            )
            if conversation_history_context != "":
                final_context_data = conversation_history_context_data
        }

        let community_reports = self.community_reports
        if self.dynamic_community_selection is not None {
            (
                community_reports,
                dynamic_info,
            ) = await self.dynamic_community_selection.select(query)
            llm_calls += dynamic_info["llm_calls"]
            prompt_tokens += dynamic_info["prompt_tokens"]
            output_tokens += dynamic_info["output_tokens"]
        }

        let (community_context, community_context_data) = build_community_context(
            community_reports=community_reports,
            entities=self.entities,
            token_encoder=self.token_encoder,
            use_community_summary=use_community_summary,
            column_delimiter=column_delimiter,
            shuffle_data=shuffle_data,
            include_community_rank=include_community_rank,
            min_community_rank=min_community_rank,
            community_rank_name=community_rank_name,
            include_community_weight=include_community_weight,
            community_weight_name=community_weight_name,
            normalize_community_weight=normalize_community_weight,
            max_tokens=max_tokens,
            single_batch=False,
            context_name=context_name,
            random_state=self.random_state,
        );

        // Prepare context_prefix based on whether conversation_history_context exists
        let context_prefix = if conversation_history_context {
            format!("{conversation_history_context}\n\n")
        } else {
            ""
        };

        let final_context = (
            [format!("{context_prefix}{context}") for context in community_context]
            if isinstance(community_context, list)
            else format!("{context_prefix}{community_context}")
        );

        // Update the final context data with the provided community_context_data
        final_context_data.update(community_context_data);

        ContextBuilderResult {
            context_chunks: final_context,
            context_records: final_context_data,
            llm_calls,
            prompt_tokens,
            output_tokens,
        }
    }
}
