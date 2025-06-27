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
/// CompoundFilter:
///   $recursiveAnchor: true
///   type: object
///   additionalProperties: false
///   title: Compound Filter
///   description: Combine multiple filters using `and` or `or`.
///   properties:
///     type:
///       type: string
///       description: "Type of operation: `and` or `or`."
///       enum:
///         - and
///         - or
///     filters:
///       type: array
///       description:
///         Array of filters to combine. Items can be `ComparisonFilter` or
///         `CompoundFilter`.
///       items:
///         oneOf:
///           - $ref: "#/components/schemas/ComparisonFilter"
///           - $recursiveRef: "#"
///   required:
///     - type
///     - filters
///   x-oaiMeta:
///     name: CompoundFilter
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct CompoundFilter {
    /// Array of filters to combine. Items can be `ComparisonFilter` or `CompoundFilter`.
    #[serde(rename = "filters")]
    pub filters: Vec<CompoundFilterItem>,
    /// Type of operation: `and` or `or`.
    #[serde(rename = "type")]
    pub _type: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CompoundFilterItem {
    Comparison(crate::ComparisonFilter),
    Compound(CompoundFilter),
}
