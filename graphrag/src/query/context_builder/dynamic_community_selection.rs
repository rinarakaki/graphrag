//! Algorithm to dynamically select relevant communities with respect to a query.

// from collections::Counter
// from time::time

use tiktoken_rs;

use crate::data_model::community::Community;
use crate::data_model::community_report::CommunityReport;
use crate::language_model::protocol::base::ChatModel;
use crate::query::context_builder::rate_prompt::RATE_QUERY;
use crate::query::context_builder::rate_relevancy::rate_relevancy;

DEFAULT_RATE_LLM_PARAMS = {"temperature": 0.0, "max_tokens": 2000}

/// Dynamic community selection to select community reports that are relevant to the query.
///
/// Any community report with a rating EQUAL or ABOVE the rating_threshold is considered relevant.
pub struct DynamicCommunitySelection {
    self.model = model
    self.token_encoder = token_encoder
    self.rate_query = rate_query
    self.num_repeats = num_repeats
    self.use_summary = use_summary
    self.threshold = threshold
    self.keep_parent = keep_parent
    self.max_level = max_level
    self.semaphore = asyncio.Semaphore(concurrent_coroutines)
    self.llm_kwargs = llm_kwargs

    self.reports = {report.community_id: report for report in community_reports}
    self.communities = {community.short_id: community for community in communities}
    self.levels: dict[str, Vec<str>] = {}

    for community in communities:
        if community.level not in self.levels:
            self.levels[community.level] = []
        if community.short_id in self.reports:
            self.levels[community.level].push(community.short_id)
    self.starting_communities = self.levels["0"]
}

impl DynamicCommunitySelection {
    pub fn new(
        community_reports: Vec<CommunityReport>,
        communities: Vec<Community>,
        model: ChatModel,
        token_encoder: tiktoken_rs::Encoding,
        rate_query: str = RATE_QUERY,
        use_summary: bool = False,
        threshold: int = 1,
        keep_parent: bool = False,
        num_repeats: int = 1,
        max_level: int = 2,
        concurrent_coroutines: int = 8,
        llm_kwargs: Any = DEFAULT_RATE_LLM_PARAMS,
    ) -> Self {
        self.model = model
        self.token_encoder = token_encoder
        self.rate_query = rate_query
        self.num_repeats = num_repeats
        self.use_summary = use_summary
        self.threshold = threshold
        self.keep_parent = keep_parent
        self.max_level = max_level
        self.semaphore = asyncio.Semaphore(concurrent_coroutines)
        self.llm_kwargs = llm_kwargs

        self.reports = {report.community_id: report for report in community_reports}
        self.communities = {community.short_id: community for community in communities}

        # mapping from level to communities
        self.levels: dict[str, Vec<str>] = {}

        for community in communities:
            if community.level not in self.levels:
                self.levels[community.level] = []
            if community.short_id in self.reports:
                self.levels[community.level].push(community.short_id)

        # start from root communities (level 0)
        self.starting_communities = self.levels["0"]
    }

    /// Select relevant communities with respect to the query.
    ///
    /// Args:
    ///     query: the query to rate against
    pub async fn select(self, query: str) -> tuple[Vec<CommunityReport>, HashMap<String, Box<dyn Any>>] {
        start = time()
        queue = deepcopy(self.starting_communities)
        level = 0

        ratings = {}  # store the ratings for each community
        llm_info: HashMap<String, Box<dyn Any>> = {
            "llm_calls": 0,
            "prompt_tokens": 0,
            "output_tokens": 0,
        }
        relevant_communities = set()

        while queue:
            gather_results = await asyncio.gather(*[
                rate_relevancy(
                    query=query,
                    description=(
                        self.reports[community].summary
                        if self.use_summary
                        else self.reports[community].full_content
                    ),
                    model=self.model,
                    token_encoder=self.token_encoder,
                    rate_query=self.rate_query,
                    num_repeats=self.num_repeats,
                    semaphore=self.semaphore,
                    **self.llm_kwargs,
                )
                for community in queue
            ])

            communities_to_rate = []
            for community, result in zip(queue, gather_results, strict=True):
                rating = result["rating"]
                log.debug(
                    "dynamic community selection: community %s rating %s",
                    community,
                    rating,
                )
                ratings[community] = rating
                llm_info["llm_calls"] += result["llm_calls"]
                llm_info["prompt_tokens"] += result["prompt_tokens"]
                llm_info["output_tokens"] += result["output_tokens"]
                if rating >= self.threshold:
                    relevant_communities.add(community)
                    # find children nodes of the current node and push them to the queue
                    # TODO check why some sub_communities are NOT in report_df
                    if community in self.communities:
                        for child in self.communities[community].children:
                            if child in self.reports:
                                communities_to_rate.push(child)
                            else:
                                log.debug(
                                    "dynamic community selection: cannot find community %s in reports",
                                    child,
                                )
                    # remove parent node if the current node is deemed relevant
                    if not self.keep_parent and community in self.communities:
                        relevant_communities.discard(self.communities[community].parent)
            queue = communities_to_rate
            level += 1
            if (
                (len(queue) == 0)
                and (len(relevant_communities) == 0)
                and (str(level) in self.levels)
                and (level <= self.max_level)
            ):
                info!(
                    "dynamic community selection: no relevant community "
                    "reports, adding all reports at level %s to rate.",
                    level,
                )
                // push all communities at the next level to queue
                queue = self.levels[str(level)]

        let community_reports = [
            self.reports[community] for community in relevant_communities
        ]
        end = time();

        info!(
            "dynamic community selection (took: %ss)\n"
            "\trating distribution %s\n"
            "\t%s out of %s community reports are relevant\n"
            "\tprompt tokens: %s, output tokens: %s",
            int(end - start),
            dict(sorted(Counter(ratings.values()).items())),
            len(relevant_communities),
            len(self.reports),
            llm_info["prompt_tokens"],
            llm_info["output_tokens"],
        );

        llm_info["ratings"] = ratings;
        (community_reports, llm_info)
    }
}
