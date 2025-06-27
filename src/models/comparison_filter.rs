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
pub struct ComparisonFilter {
    /// The key to compare against the value.
    #[serde(rename = "key")]
    pub key: String,
    /// Specifies the comparison operator: `eq`, `ne`, `gt`, `gte`, `lt`, `lte`. - `eq`: equals - `ne`: not equal - `gt`: greater than - `gte`: greater than or equal - `lt`: less than - `lte`: less than or equal
    #[serde(rename = "type")]
    pub _type: String,
    /// The value to compare against the attribute key; supports string, number, or boolean types.
    #[serde(rename = "value")]
    pub value: ComparisonFilterValue,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ComparisonFilterValue {
    String(String),
    Number(isize),
    Boolean(bool),
}
