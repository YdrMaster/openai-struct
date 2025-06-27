/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

use crate::{CodeInterpreterFileOutput, CodeInterpreterTextOutput};

/// # on openapi.yaml
///
/// ```yaml
/// CodeInterpreterToolOutput:
///   oneOf:
///     - $ref: "#/components/schemas/CodeInterpreterTextOutput"
///     - $ref: "#/components/schemas/CodeInterpreterFileOutput"
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub enum CodeInterpreterToolOutput {
    TextOutput(CodeInterpreterTextOutput),
    FileOutput(CodeInterpreterFileOutput),
}
