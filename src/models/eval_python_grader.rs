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
pub struct EvalPythonGrader {
    /// The image tag to use for the python script.
    #[serde(rename = "image_tag")]
    pub image_tag: Option<String>,
    /// The name of the grader.
    #[serde(rename = "name")]
    pub name: String,
    /// The threshold for the score.
    #[serde(rename = "pass_threshold")]
    pub pass_threshold: Option<f32>,
    /// The source code of the python script.
    #[serde(rename = "source")]
    pub source: String,
    /// The object type, which is always `python`.
    #[serde(rename = "type")]
    #[serde(default = "default_type")]
    pub _type: String,
}

fn default_type() -> String {
    "python".into()
}
