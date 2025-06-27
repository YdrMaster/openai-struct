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
pub struct CreateModerationResponseCategoryAppliedInputTypes {
    /// The applied input type(s) for the category 'harassment'.
    #[serde(rename = "harassment")]
    pub harassment: Vec<String>,
    /// The applied input type(s) for the category 'harassment/threatening'.
    #[serde(rename = "harassment/threatening")]
    pub harassmentthreatening: Vec<String>,
    /// The applied input type(s) for the category 'hate'.
    #[serde(rename = "hate")]
    pub hate: Vec<String>,
    /// The applied input type(s) for the category 'hate/threatening'.
    #[serde(rename = "hate/threatening")]
    pub hatethreatening: Vec<String>,
    /// The applied input type(s) for the category 'illicit'.
    #[serde(rename = "illicit")]
    pub illicit: Vec<String>,
    /// The applied input type(s) for the category 'illicit/violent'.
    #[serde(rename = "illicit/violent")]
    pub illicitviolent: Vec<String>,
    /// The applied input type(s) for the category 'self-harm'.
    #[serde(rename = "self-harm")]
    pub self_harm: Vec<String>,
    /// The applied input type(s) for the category 'self-harm/instructions'.
    #[serde(rename = "self-harm/instructions")]
    pub self_harminstructions: Vec<String>,
    /// The applied input type(s) for the category 'self-harm/intent'.
    #[serde(rename = "self-harm/intent")]
    pub self_harmintent: Vec<String>,
    /// The applied input type(s) for the category 'sexual'.
    #[serde(rename = "sexual")]
    pub sexual: Vec<String>,
    /// The applied input type(s) for the category 'sexual/minors'.
    #[serde(rename = "sexual/minors")]
    pub sexualminors: Vec<String>,
    /// The applied input type(s) for the category 'violence'.
    #[serde(rename = "violence")]
    pub violence: Vec<String>,
    /// The applied input type(s) for the category 'violence/graphic'.
    #[serde(rename = "violence/graphic")]
    pub violencegraphic: Vec<String>,
}
