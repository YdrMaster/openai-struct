/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

#[derive(Debug, Serialize, Deserialize)]
pub struct StaticChunkingStrategy {
    /// The number of tokens that overlap between chunks. The default value is `400`.  Note that the overlap must not exceed half of `max_chunk_size_tokens`.
    #[serde(rename = "chunk_overlap_tokens")]
    pub chunk_overlap_tokens: i32,
    /// The maximum number of tokens in each chunk. The default value is `800`. The minimum value is `100` and the maximum value is `4096`.
    #[serde(rename = "max_chunk_size_tokens")]
    pub max_chunk_size_tokens: i32,
}
