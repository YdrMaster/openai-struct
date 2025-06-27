/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

use crate::ChatCompletionRequestMessageContentPartText;

/// # on openapi.yaml
///
/// ```yaml
/// ChatCompletionRequestSystemMessageContentPart:
///   oneOf:
///     - $ref: "#/components/schemas/ChatCompletionRequestMessageContentPartText"
/// ```
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum ChatCompletionRequestSystemMessageContentPart {
    Text(ChatCompletionRequestMessageContentPartText),
}
