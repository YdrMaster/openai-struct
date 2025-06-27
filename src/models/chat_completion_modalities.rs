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
/// ChatCompletionModalities:
///   type: array
///   nullable: true
///   description: >
///     Output types that you would like the model to generate for this request.
///
///     Most models are capable of generating text, which is the default:
///
///
///     `["text"]`
///
///
///     The `gpt-4o-audio-preview` model can also be used to [generate
///     audio](/docs/guides/audio). To
///
///     request that this model generate both text and audio responses, you can
///
///     use:
///
///
///     `["text", "audio"]`
///   items:
///     type: string
///     enum:
///       - text
///       - audio
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct ChatCompletionModalities(
    #[serde(default = "default_modalities")] Vec<ChatCompletionModality>,
);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ChatCompletionModality {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "audio")]
    Audio,
}

fn default_modalities() -> Vec<ChatCompletionModality> {
    vec![ChatCompletionModality::Text]
}

#[test]
fn test_mode() {
    assert_eq!(
        serde_json::to_string(&ChatCompletionModalities(vec![
            ChatCompletionModality::Text,
            ChatCompletionModality::Audio
        ]))
        .unwrap(),
        r#"["text","audio"]"#
    );
}
