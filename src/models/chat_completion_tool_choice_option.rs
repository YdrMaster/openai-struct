/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https:///platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https:///github.com/swagger-api/swagger-codegen.git
 */

use crate::ChatCompletionNamedToolChoice;

/// # on openapi.yaml
///
/// ```yaml
/// ChatCompletionToolChoiceOption:
///   description: >
///     Controls which (if any) tool is called by the model.
///
///     `none` means the model will not call any tool and instead generates a
///     message.
///
///     `auto` means the model can pick between generating a message or calling
///     one or more tools.
///
///     `required` means the model must call one or more tools.
///
///     Specifying a particular tool via `{"type": "function", "function":
///     {"name": "my_function"}}` forces the model to call that tool.
///
///
///     `none` is the default when no tools are present. `auto` is the default
///     if tools are present.
///   oneOf:
///     - type: string
///       description: >
///         `none` means the model will not call any tool and instead generates
///         a message. `auto` means the model can pick between generating a
///         message or calling one or more tools. `required` means the model
///         must call one or more tools.
///       enum:
///         - none
///         - auto
///         - required
///     - $ref: "#/components/schemas/ChatCompletionNamedToolChoice"
/// ```
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatCompletionToolChoiceOption {
    /// Str
    String(ChatCompletionToolChoiceStrEnum),
    /// Named tool choice variant for specific tool selection
    VariantNamedToolChoice(ChatCompletionNamedToolChoice),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChatCompletionToolChoiceStrEnum {
    /// Model will not call any tool and instead generates a message
    #[serde(rename = "none")]
    None,
    /// Model can pick between generating a message or calling tools
    #[serde(rename = "auto")]
    Auto,
    /// Model must call one or more tools
    #[serde(rename = "required")]
    Required,
}

#[test]
fn test_cct_co() {
    assert_eq!(
        serde_json::to_string(&ChatCompletionToolChoiceOption::String(
            ChatCompletionToolChoiceStrEnum::None
        ))
        .unwrap(),
        r#""none""#
    );
    assert_eq!(
        serde_json::to_string(&ChatCompletionToolChoiceOption::VariantNamedToolChoice(
            ChatCompletionNamedToolChoice {
                function: crate::models::AssistantsNamedToolChoiceFunction { name: "Emm".into() },
                _type: "function".into()
            }
        ))
        .unwrap()
        .to_string(),
        r#"{"function":{"name":"Emm"},"type":"function"}"#
    );
}
