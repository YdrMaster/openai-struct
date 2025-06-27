/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

use crate::{InputFileContent, InputImageContent, InputTextContent};

/// # on openapi.yaml
///
/// ```yaml
/// InputContent:
///   oneOf:
///     - $ref: "#/components/schemas/InputTextContent"
///     - $ref: "#/components/schemas/InputImageContent"
///     - $ref: "#/components/schemas/InputFileContent"
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub enum InputContent {
    Text(InputTextContent),
    Image(InputImageContent),
    File(InputFileContent),
}
