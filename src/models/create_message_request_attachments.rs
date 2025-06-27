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
pub struct CreateMessageRequestAttachments {
    /// The ID of the file to attach to the message.
    #[serde(rename = "file_id")]
    pub file_id: Option<String>,
    /// The tools to add this file to.
    #[serde(rename = "tools")]
    pub tools: Option<Vec<CreateMessageRequestAttachmentsTool>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateMessageRequestAttachmentsTool {
    Code(crate::AssistantToolsCode),
    FileSearchTypeOnly(crate::AssistantToolsFileSearchTypeOnly),
}
