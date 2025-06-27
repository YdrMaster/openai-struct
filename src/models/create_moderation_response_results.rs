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
pub struct CreateModerationResponseResults {
    #[serde(rename = "categories")]
    pub categories: crate::models::CreateModerationResponseCategories,
    #[serde(rename = "category_applied_input_types")]
    pub category_applied_input_types:
        crate::models::CreateModerationResponseCategoryAppliedInputTypes,
    #[serde(rename = "category_scores")]
    pub category_scores: crate::models::CreateModerationResponseCategoryScores,
    /// Whether any of the below categories are flagged.
    #[serde(rename = "flagged")]
    pub flagged: bool,
}
