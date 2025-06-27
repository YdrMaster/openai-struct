/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

use crate::{InputContent, OutputContent};

/// # on openapi.yaml
///
/// ```yaml
/// Content:
///   description: |
///     Multi-modal input and output contents.
///   oneOf:
///     - title: Input content types
///       $ref: "#/components/schemas/InputContent"
///     - title: Output content types
///       $ref: "#/components/schemas/OutputContent"
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub enum Content {
    Input(InputContent),
    Output(OutputContent),
}
