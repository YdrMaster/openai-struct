/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

// use crate::{
//     TranscriptTextDeltaEvent,
//     TranscriptTextDoneEvent,
// };

/// # on openapi.yaml
///
/// ```yaml
/// CreateTranscriptionResponseStreamEvent:
///   anyOf:
///     - $ref: "#/components/schemas/TranscriptTextDeltaEvent"
///     - $ref: "#/components/schemas/TranscriptTextDoneEvent"
///   discriminator:
///     propertyName: type
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub enum CreateTranscriptionResponseStreamEvent {}
