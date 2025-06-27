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
pub struct CreateModerationResponseCategoryScores {
    /// The score for the category 'harassment'.
    #[serde(rename = "harassment")]
    pub harassment: f32,
    /// The score for the category 'harassment/threatening'.
    #[serde(rename = "harassment/threatening")]
    pub harassmentthreatening: f32,
    /// The score for the category 'hate'.
    #[serde(rename = "hate")]
    pub hate: f32,
    /// The score for the category 'hate/threatening'.
    #[serde(rename = "hate/threatening")]
    pub hatethreatening: f32,
    /// The score for the category 'illicit'.
    #[serde(rename = "illicit")]
    pub illicit: f32,
    /// The score for the category 'illicit/violent'.
    #[serde(rename = "illicit/violent")]
    pub illicitviolent: f32,
    /// The score for the category 'self-harm'.
    #[serde(rename = "self-harm")]
    pub self_harm: f32,
    /// The score for the category 'self-harm/instructions'.
    #[serde(rename = "self-harm/instructions")]
    pub self_harminstructions: f32,
    /// The score for the category 'self-harm/intent'.
    #[serde(rename = "self-harm/intent")]
    pub self_harmintent: f32,
    /// The score for the category 'sexual'.
    #[serde(rename = "sexual")]
    pub sexual: f32,
    /// The score for the category 'sexual/minors'.
    #[serde(rename = "sexual/minors")]
    pub sexualminors: f32,
    /// The score for the category 'violence'.
    #[serde(rename = "violence")]
    pub violence: f32,
    /// The score for the category 'violence/graphic'.
    #[serde(rename = "violence/graphic")]
    pub violencegraphic: f32,
}
