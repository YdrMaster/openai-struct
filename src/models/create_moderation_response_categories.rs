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
pub struct CreateModerationResponseCategories {
    /// Content that expresses, incites, or promotes harassing language towards any target.
    #[serde(rename = "harassment")]
    pub harassment: bool,
    /// Harassment content that also includes violence or serious harm towards any target.
    #[serde(rename = "harassment/threatening")]
    pub harassmentthreatening: bool,
    /// Content that expresses, incites, or promotes hate based on race, gender, ethnicity, religion, nationality, sexual orientation, disability status, or caste. Hateful content aimed at non-protected groups (e.g., chess players) is harassment.
    #[serde(rename = "hate")]
    pub hate: bool,
    /// Hateful content that also includes violence or serious harm towards the targeted group based on race, gender, ethnicity, religion, nationality, sexual orientation, disability status, or caste.
    #[serde(rename = "hate/threatening")]
    pub hatethreatening: bool,
    /// Content that includes instructions or advice that facilitate the planning or execution of wrongdoing, or that gives advice or instruction on how to commit illicit acts. For example, \"how to shoplift\" would fit this category.
    #[serde(rename = "illicit")]
    pub illicit: bool,
    /// Content that includes instructions or advice that facilitate the planning or execution of wrongdoing that also includes violence, or that gives advice or instruction on the procurement of any weapon.
    #[serde(rename = "illicit/violent")]
    pub illicitviolent: bool,
    /// Content that promotes, encourages, or depicts acts of self-harm, such as suicide, cutting, and eating disorders.
    #[serde(rename = "self-harm")]
    pub self_harm: bool,
    /// Content that encourages performing acts of self-harm, such as suicide, cutting, and eating disorders, or that gives instructions or advice on how to commit such acts.
    #[serde(rename = "self-harm/instructions")]
    pub self_harminstructions: bool,
    /// Content where the speaker expresses that they are engaging or intend to engage in acts of self-harm, such as suicide, cutting, and eating disorders.
    #[serde(rename = "self-harm/intent")]
    pub self_harmintent: bool,
    /// Content meant to arouse sexual excitement, such as the description of sexual activity, or that promotes sexual services (excluding sex education and wellness).
    #[serde(rename = "sexual")]
    pub sexual: bool,
    /// Sexual content that includes an individual who is under 18 years old.
    #[serde(rename = "sexual/minors")]
    pub sexualminors: bool,
    /// Content that depicts death, violence, or physical injury.
    #[serde(rename = "violence")]
    pub violence: bool,
    /// Content that depicts death, violence, or physical injury in graphic detail.
    #[serde(rename = "violence/graphic")]
    pub violencegraphic: bool,
}
