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
pub struct EvalTextSimilarityGrader {
    /// The evaluation metric to use. One of `fuzzy_match`, `bleu`, `gleu`, `meteor`, `rouge_1`, `rouge_2`, `rouge_3`, `rouge_4`, `rouge_5`, or `rouge_l`.
    #[serde(rename = "evaluation_metric")]
    pub evaluation_metric: String,
    /// The text being graded.
    #[serde(rename = "input")]
    pub input: String,
    /// The name of the grader.
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// A float score where a value greater than or equal indicates a passing grade.
    #[serde(rename = "pass_threshold")]
    pub pass_threshold: f32,
    /// The text being graded against.
    #[serde(rename = "reference")]
    pub reference: String,
    /// The type of grader.
    #[serde(rename = "type")]
    pub _type: String,
}
