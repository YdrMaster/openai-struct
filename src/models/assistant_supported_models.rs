/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// # on openapi.yaml
///
/// ```yaml
/// AssistantSupportedModels:
///   type: string
///   enum:
///     - gpt-4.1
///     - gpt-4.1-mini
///     - gpt-4.1-nano
///     - gpt-4.1-2025-04-14
///     - gpt-4.1-mini-2025-04-14
///     - gpt-4.1-nano-2025-04-14
///     - o3-mini
///     - o3-mini-2025-01-31
///     - o1
///     - o1-2024-12-17
///     - gpt-4o
///     - gpt-4o-2024-11-20
///     - gpt-4o-2024-08-06
///     - gpt-4o-2024-05-13
///     - gpt-4o-mini
///     - gpt-4o-mini-2024-07-18
///     - gpt-4.5-preview
///     - gpt-4.5-preview-2025-02-27
///     - gpt-4-turbo
///     - gpt-4-turbo-2024-04-09
///     - gpt-4-0125-preview
///     - gpt-4-turbo-preview
///     - gpt-4-1106-preview
///     - gpt-4-vision-preview
///     - gpt-4
///     - gpt-4-0314
///     - gpt-4-0613
///     - gpt-4-32k
///     - gpt-4-32k-0314
///     - gpt-4-32k-0613
///     - gpt-3.5-turbo
///     - gpt-3.5-turbo-16k
///     - gpt-3.5-turbo-0613
///     - gpt-3.5-turbo-1106
///     - gpt-3.5-turbo-0125
///     - gpt-3.5-turbo-16k-0613
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AssistantSupportedModels {
    #[serde(rename = "gpt-4.1")]
    Gpt4_1,
    #[serde(rename = "gpt-4.1-mini")]
    Gpt4_1Mini,
    #[serde(rename = "gpt-4.1-nano")]
    Gpt4_1Nano,
    #[serde(rename = "gpt-4.1-2025-04-14")]
    Gpt4_120250414,
    #[serde(rename = "gpt-4.1-mini-2025-04-14")]
    Gpt4_1Mini20250414,
    #[serde(rename = "gpt-4.1-nano-2025-04-14")]
    Gpt4_1Nano20250414,
    #[serde(rename = "o3-mini")]
    O3Mini,
    #[serde(rename = "o3-mini-2025-01-31")]
    O3Mini20250131,
    #[serde(rename = "o1")]
    O1,
    #[serde(rename = "o1-2024-12-17")]
    O120241217,
    #[serde(rename = "gpt-4o")]
    Gpt4o,
    #[serde(rename = "gpt-4o-2024-11-20")]
    Gpt4o20241120,
    #[serde(rename = "gpt-4o-2024-08-06")]
    Gpt4o20240806,
    #[serde(rename = "gpt-4o-2024-05-13")]
    Gpt4o20240513,
    #[serde(rename = "gpt-4o-mini")]
    Gpt4oMini,
    #[serde(rename = "gpt-4o-mini-2024-07-18")]
    Gpt4oMini20240718,
    #[serde(rename = "gpt-4.5-preview")]
    Gpt4_5Preview,
    #[serde(rename = "gpt-4.5-preview-2025-02-27")]
    Gpt4_5Preview20250227,
    #[serde(rename = "gpt-4-turbo")]
    Gpt4Turbo,
    #[serde(rename = "gpt-4-turbo-2024-04-09")]
    Gpt4Turbo20240409,
    #[serde(rename = "gpt-4-0125-preview")]
    Gpt40125Preview,
    #[serde(rename = "gpt-4-turbo-preview")]
    Gpt4TurboPreview,
    #[serde(rename = "gpt-4-1106-preview")]
    Gpt41106Preview,
    #[serde(rename = "gpt-4-vision-preview")]
    Gpt4VisionPreview,
    #[serde(rename = "gpt-4")]
    Gpt4,
    #[serde(rename = "gpt-4-0314")]
    Gpt40314,
    #[serde(rename = "gpt-4-0613")]
    Gpt40613,
    #[serde(rename = "gpt-4-32k")]
    Gpt432k,
    #[serde(rename = "gpt-4-32k-0314")]
    Gpt432k0314,
    #[serde(rename = "gpt-4-32k-0613")]
    Gpt432k0613,
    #[serde(rename = "gpt-3.5-turbo")]
    Gpt3_5Turbo,
    #[serde(rename = "gpt-3.5-turbo-16k")]
    Gpt3_5Turbo16k,
    #[serde(rename = "gpt-3.5-turbo-0613")]
    Gpt3_5Turbo0613,
    #[serde(rename = "gpt-3.5-turbo-1106")]
    Gpt3_5Turbo1106,
    #[serde(rename = "gpt-3.5-turbo-0125")]
    Gpt3_5Turbo0125,
    #[serde(rename = "gpt-3.5-turbo-16k-0613")]
    Gpt3_5Turbo16k0613,
}

#[test]
fn test_serialize() {
    assert_eq!(
        serde_json::to_string(&AssistantSupportedModels::Gpt4_1Mini).unwrap(),
        r#""gpt-4.1-mini""#
    );
}
