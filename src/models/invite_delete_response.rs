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
pub struct InviteDeleteResponse {
    #[serde(rename = "deleted")]
    pub deleted: bool,
    #[serde(rename = "id")]
    pub id: String,
    /// The object type, which is always `organization.invite.deleted`
    #[serde(rename = "object")]
    pub object: String,
}
