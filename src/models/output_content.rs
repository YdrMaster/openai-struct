/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

use crate::{OutputTextContent, RefusalContent};

/// # on openapi.yaml
///
/// ```yaml
/// OutputContent:
///   oneOf:
///     - $ref: "#/components/schemas/OutputTextContent"
///     - $ref: "#/components/schemas/RefusalContent"
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub enum OutputContent {
    OutputText(OutputTextContent),
    RefusalContent(RefusalContent),
}
