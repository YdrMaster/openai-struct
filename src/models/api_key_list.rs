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
/// ApiKeyList:
///   type: object
///   properties:
///     object:
///       type: string
///       example: list
///     data:
///       type: array
///       items:
///         $ref: "#/components/schemas/AdminApiKey"
///     has_more:
///       type: boolean
///       example: false
///     first_id:
///       type: string
///       example: key_abc
///     last_id:
///       type: string
///       example: key_xyz
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiKeyList {
    #[serde(rename = "data")]
    pub data: Option<Vec<crate::models::AdminApiKey>>,
    #[serde(rename = "first_id")]
    pub first_id: Option<String>,
    #[serde(rename = "has_more")]
    pub has_more: Option<bool>,
    #[serde(rename = "last_id")]
    pub last_id: Option<String>,
    #[serde(rename = "object")]
    pub object: Option<String>,
}
