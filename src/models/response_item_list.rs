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
pub struct ResponseItemList {
    /// A list of items used to generate this response.
    #[serde(rename = "data")]
    pub data: Vec<crate::models::ItemResource>,
    /// The ID of the first item in the list.
    #[serde(rename = "first_id")]
    pub first_id: String,
    /// Whether there are more items available.
    #[serde(rename = "has_more")]
    pub has_more: bool,
    /// The ID of the last item in the list.
    #[serde(rename = "last_id")]
    pub last_id: String,
    /// The type of object returned, must be `list`.
    #[serde(rename = "object")]
    pub object: String,
}
