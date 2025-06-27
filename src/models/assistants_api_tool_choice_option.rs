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
/// AssistantsApiToolChoiceOption:
///   description: >
///     Controls which (if any) tool is called by the model.
///
///     `none` means the model will not call any tools and instead generates a
///     message.
///
///     `auto` is the default value and means the model can pick between
///     generating a message or calling one or more tools.
///
///     `required` means the model must call one or more tools before responding
///     to the user.
///
///     Specifying a particular tool like `{"type": "file_search"}` or `{"type":
///     "function", "function": {"name": "my_function"}}` forces the model to
///     call that tool.
///   oneOf:
///     - type: string
///       description: >
///         `none` means the model will not call any tools and instead generates
///         a message. `auto` means the model can pick between generating a
///         message or calling one or more tools. `required` means the model
///         must call one or more tools before responding to the user.
///       enum:
///         - none
///         - auto
///         - required
///     - $ref: "#/components/schemas/AssistantsNamedToolChoice"
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AssistantsApiToolChoiceOption {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "required")]
    Required,
}
