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
/// FunctionObject:
///   type: object
///   properties:
///     description:
///       type: string
///       description:
///         A description of what the function does, used by the model to
///         choose when and how to call the function.
///     name:
///       type: string
///       description:
///         The name of the function to be called. Must be a-z, A-Z, 0-9, or
///         contain underscores and dashes, with a maximum length of 64.
///     parameters:
///       $ref: "#/components/schemas/FunctionParameters"
///     strict:
///       type: boolean
///       nullable: true
///       default: false
///       description:
///         Whether to enable strict schema adherence when generating the
///         function call. If set to true, the model will follow the exact
///         schema defined in the `parameters` field. Only a subset of JSON
///         Schema is supported when `strict` is `true`. Learn more about
///         Structured Outputs in the [function calling
///         guide](docs/guides/function-calling).
///   required:
///     - name
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionObject {
    #[serde(rename = "parameters")]
    pub parameters: Option<crate::models::FunctionParameters>,
    /// A description of what the function does, used by the model to choose when and how to call the function.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// The name of the function to be called. Must be a-z, A-Z, 0-9, or contain underscores and dashes, with a maximum length of 64.
    #[serde(rename = "name")]
    pub name: String,
    /// Whether to enable strict schema adherence when generating the function call. If set to true, the model will follow the exact schema defined in the `parameters` field. Only a subset of JSON Schema is supported when `strict` is `true`. Learn more about Structured Outputs in the [function calling guide](docs/guides/function-calling).
    #[serde(rename = "strict")]
    #[serde(default = "default_strict")]
    pub strict: Option<bool>,
}

fn default_strict() -> Option<bool> {
    Some(false)
}
