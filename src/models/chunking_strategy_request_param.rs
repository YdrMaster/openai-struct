/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

use crate::{AutoChunkingStrategyRequestParam, StaticChunkingStrategyRequestParam};

/// # on openapi.yaml
///
/// ```yaml
/// ChunkingStrategyRequestParam:
///   type: object
///   description:
///     The chunking strategy used to chunk the file(s). If not set, will
///     use the `auto` strategy.
///   oneOf:
///     - $ref: "#/components/schemas/AutoChunkingStrategyRequestParam"
///     - $ref: "#/components/schemas/StaticChunkingStrategyRequestParam"
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub enum ChunkingStrategyRequestParam {
    Auto(AutoChunkingStrategyRequestParam),
    Static(StaticChunkingStrategyRequestParam),
}
