/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https:///platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https:///github.com/swagger-api/swagger-codegen.git
 */

/// # on openapi.yaml
///
/// ```yaml
/// CostsResult:
///   type: object
///   description: The aggregated costs details of the specific time bucket.
///   properties:
///     object:
///       type: string
///       enum:
///         - organization.costs.result
///       x-stainless-const: true
///     amount:
///       type: object
///       description: The monetary value in its associated currency.
///       properties:
///         value:
///           type: number
///           description: The numeric value of the cost.
///         currency:
///           type: string
///           description: Lowercase ISO-4217 currency e.g. "usd"
///     line_item:
///       type: string
///       nullable: true
///       description:
///         When `group_by=line_item`, this field provides the line item of the
///         grouped costs result.
///     project_id:
///       type: string
///       nullable: true
///       description:
///         When `group_by=project_id`, this field provides the project ID of
///         the grouped costs result.
///   required:
///     - object
///   x-oaiMeta:
///     name: Costs object
///     example: |
///       {
///           "object": "organization.costs.result",
///           "amount": {
///             "value": 0.06,
///             "currency": "usd"
///           },
///           "line_item": "Image models",
///           "project_id": "proj_abc"
///       }
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct CostsResult {
    #[serde(rename = "amount")]
    pub amount: Option<crate::models::CostsResultAmount>,
    /// When `group_by=line_item`, this field provides the line item of the grouped costs result.
    #[serde(rename = "line_item")]
    pub line_item: Option<String>,
    #[serde(rename = "object")]
    pub object: String,
    /// When `group_by=project_id`, this field provides the project ID of the grouped costs result.
    #[serde(rename = "project_id")]
    pub project_id: Option<String>,
}
