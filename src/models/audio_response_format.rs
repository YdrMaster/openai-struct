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
/// AudioResponseFormat:
///   description: >
///     The format of the output, in one of these options: `json`, `text`,
///     `srt`, `verbose_json`, or `vtt`. For `gpt-4o-transcribe` and
///     `gpt-4o-mini-transcribe`, the only supported format is `json`.
///   type: string
///   enum:
///     - json
///     - text
///     - srt
///     - verbose_json
///     - vtt
///   default: json
/// ```
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AudioResponseFormat {
    Json,
    Text,
    Srt,
    VerboseJson,
    Vtt,
}

#[test]
fn test_snake() {
    assert_eq!(
        serde_json::to_string(&AudioResponseFormat::VerboseJson).unwrap(),
        r#""verbose_json""#,
    );
}
