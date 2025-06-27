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
pub struct CreateThreadRequest {
    /// A list of [messages](/docs/api-reference/messages) to start the thread with.
    #[serde(rename = "messages")]
    pub messages: Option<Vec<crate::models::CreateMessageRequest>>,
    #[serde(rename = "metadata")]
    pub metadata: Option<crate::models::Metadata>,
    #[serde(rename = "tool_resources")]
    pub tool_resources: Option<crate::models::CreateThreadRequestToolResources>,
}
