/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https:///platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https:///github.com/swagger-api/swagger-codegen.git
 */

/// # on openapi.yaml
///
/// ```yaml
/// ResponseModalities:
///   type: array
///   nullable: true
///   description: >
///     Output types that you would like the model to generate.
///
///     Most models are capable of generating text, which is the default:
///
///
///     `["text"]`
///
///
///     The `gpt-4o-audio-preview` model can also be used to
///
///     [generate audio](/docs/guides/audio). To request that this model
///     generate
///
///     both text and audio responses, you can use:
///
///
///     `["text", "audio"]`
///   items:
///     type: string
///     enum:
///       - text
///       - audio
/// ```
pub type ResponseModalities = Vec<ResponseModalitiesItem>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ResponseModalitiesItem {
    Text,
    Audio,
}
