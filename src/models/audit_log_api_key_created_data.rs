/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLogApiKeyCreatedData {
    /// A list of scopes allowed for the API key, e.g. `[\"api.model.request\"]`
    #[serde(rename = "scopes")]
    pub scopes: Option<Vec<String>>,
}
