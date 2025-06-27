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
/// Metadata:
///   type: object
///   description: >
///     Set of 16 key-value pairs that can be attached to an object. This can be
///
///     useful for storing additional information about the object in a
///     structured
///
///     format, and querying for objects via API or the dashboard.
///
///
///     Keys are strings with a maximum length of 64 characters. Values are
///     strings
///
///     with a maximum length of 512 characters.
/// ```
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Metadata {
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub data: ::std::collections::HashMap<String, String>,
}
