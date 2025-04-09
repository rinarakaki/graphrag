//! Common field name definitions for data frames.

pub const ID: &str = "id";
pub const SHORT_ID: &str = "human_readable_id";
pub const TITLE: &str = "title";
pub const DESCRIPTION: &str = "description";

pub const TYPE: &str = "type";

// POST-PREP NODE TABLE SCHEMA
pub const NODE_DEGREE: &str = "degree";
pub const NODE_FREQUENCY: &str = "frequency";
pub const NODE_DETAILS: &str = "node_details";
pub const NODE_X: &str = "x";
pub const NODE_Y: &str = "y";

// POST-PREP EDGE TABLE SCHEMA
pub const EDGE_SOURCE: &str = "source";
pub const EDGE_TARGET: &str = "target";
pub const EDGE_DEGREE: &str = "combined_degree";
pub const EDGE_DETAILS: &str = "edge_details";
pub const EDGE_WEIGHT: &str = "weight";

// POST-PREP CLAIM TABLE SCHEMA
pub const CLAIM_SUBJECT: &str = "subject_id";
pub const CLAIM_STATUS: &str = "status";
pub const CLAIM_DETAILS: &str = "claim_details";

// COMMUNITY HIERARCHY TABLE SCHEMA
pub const SUB_COMMUNITY: &str = "sub_community";

// COMMUNITY CONTEXT TABLE SCHEMA
pub const ALL_CONTEXT: &str = "all_context";
pub const CONTEXT_STRING: &str = "context_string";
pub const CONTEXT_SIZE: &str = "context_size";
pub const CONTEXT_EXCEED_FLAG: &str = "context_exceed_limit";

// COMMUNITY REPORT TABLE SCHEMA
pub const COMMUNITY_ID: &str = "community";
pub const COMMUNITY_LEVEL: &str = "level";
pub const COMMUNITY_PARENT: &str = "parent";
pub const COMMUNITY_CHILDREN: &str = "children";
// pub const TITLE: &str = "title";
pub const SUMMARY: &str = "summary";
pub const FINDINGS: &str = "findings";
pub const RATING: &str = "rank";
pub const EXPLANATION: &str = "rating_explanation";
pub const FULL_CONTENT: &str = "full_content";
pub const FULL_CONTENT_JSON: &str = "full_content_json";

pub const ENTITY_IDS: &str = "entity_ids";
pub const RELATIONSHIP_IDS: &str = "relationship_ids";
pub const TEXT_UNIT_IDS: &str = "text_unit_ids";
pub const COVARIATE_IDS: &str = "covariate_ids";
pub const DOCUMENT_IDS: &str = "document_ids";

pub const PERIOD: &str = "period";
pub const SIZE: &str = "size";

// text units
pub const ENTITY_DEGREE: &str = "entity_degree";
pub const ALL_DETAILS: &str = "all_details";
pub const TEXT: &str = "text";
pub const N_TOKENS: &str = "n_tokens";

pub const CREATION_DATE: &str = "creation_date";
pub const METADATA: &str = "metadata";

// the following lists define the final content and ordering of columns in the data model parquet outputs
pub const ENTITIES_FINAL_COLUMNS: [&str; 10] = [
    ID,
    SHORT_ID,
    TITLE,
    TYPE,
    DESCRIPTION,
    TEXT_UNIT_IDS,
    NODE_FREQUENCY,
    NODE_DEGREE,
    NODE_X,
    NODE_Y,
];

pub const RELATIONSHIPS_FINAL_COLUMNS: [&str; 8] = [
    ID,
    SHORT_ID,
    EDGE_SOURCE,
    EDGE_TARGET,
    DESCRIPTION,
    EDGE_WEIGHT,
    EDGE_DEGREE,
    TEXT_UNIT_IDS,
];

pub const COMMUNITIES_FINAL_COLUMNS: [&str; 12] = [
    ID,
    SHORT_ID,
    COMMUNITY_ID,
    COMMUNITY_LEVEL,
    COMMUNITY_PARENT,
    COMMUNITY_CHILDREN,
    TITLE,
    ENTITY_IDS,
    RELATIONSHIP_IDS,
    TEXT_UNIT_IDS,
    PERIOD,
    SIZE,
];

pub const COMMUNITY_REPORTS_FINAL_COLUMNS: [&str; 15] = [
    ID,
    SHORT_ID,
    COMMUNITY_ID,
    COMMUNITY_LEVEL,
    COMMUNITY_PARENT,
    COMMUNITY_CHILDREN,
    TITLE,
    SUMMARY,
    FULL_CONTENT,
    RATING,
    EXPLANATION,
    FINDINGS,
    FULL_CONTENT_JSON,
    PERIOD,
    SIZE,
];

pub const COVARIATES_FINAL_COLUMNS: [&str; 12] = [
    ID,
    SHORT_ID,
    "covariate_type",
    TYPE,
    DESCRIPTION,
    "subject_id",
    "object_id",
    "status",
    "start_date",
    "end_date",
    "source_text",
    "text_unit_id",
];

pub const TEXT_UNITS_FINAL_COLUMNS: [&str; 8] = [
    ID,
    SHORT_ID,
    TEXT,
    N_TOKENS,
    DOCUMENT_IDS,
    ENTITY_IDS,
    RELATIONSHIP_IDS,
    COVARIATE_IDS,
];

pub const DOCUMENTS_FINAL_COLUMNS: [&str; 7] = [
    ID,
    SHORT_ID,
    TITLE,
    TEXT,
    TEXT_UNIT_IDS,
    CREATION_DATE,
    METADATA,
];
