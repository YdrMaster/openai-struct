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
pub struct InviteListResponse {
    #[serde(rename = "data")]
    pub data: Vec<crate::models::Invite>,
    /// The first `invite_id` in the retrieved `list`
    #[serde(rename = "first_id")]
    pub first_id: Option<String>,
    /// The `has_more` property is used for pagination to indicate there are additional results.
    #[serde(rename = "has_more")]
    pub has_more: Option<bool>,
    /// The last `invite_id` in the retrieved `list`
    #[serde(rename = "last_id")]
    pub last_id: Option<String>,
    /// The object type, which is always `list`
    #[serde(rename = "object")]
    pub object: String,
}
