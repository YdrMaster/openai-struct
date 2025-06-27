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
pub struct ModifyThreadRequestToolResources {
    #[serde(rename = "code_interpreter")]
    pub code_interpreter: Option<crate::models::CreateAssistantRequestToolResourcesCodeInterpreter>,
    #[serde(rename = "file_search")]
    pub file_search: Option<crate::models::ModifyThreadRequestToolResourcesFileSearch>,
}
