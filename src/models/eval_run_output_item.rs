/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

// #[derive(Debug, Serialize, Deserialize)]
// pub struct EvalRunOutputItem {
//   /// Unix timestamp (in seconds) when the evaluation run was created.
//   #[serde(rename = "created_at")]
//   pub created_at: i32,
//   /// Details of the input data source item.
//   #[serde(rename = "datasource_item")]
//   pub datasource_item: ::std::collections::HashMap<String, pub crate::models::Object>,
//   /// The identifier for the data source item.
//   #[serde(rename = "datasource_item_id")]
//   pub datasource_item_id: i32,
//   /// The identifier of the evaluation group.
//   #[serde(rename = "eval_id")]
//   pub eval_id: String,
//   /// Unique identifier for the evaluation run output item.
//   #[serde(rename = "id")]
//   pub id: String,
//   /// The type of the object. Always \"eval.run.output_item\".
//   #[serde(rename = "object")]
//   pub object: String,
//   /// A list of results from the evaluation run.
//   #[serde(rename = "results")]
//   pub results: Vec<::std::collections::HashMap<String, pub crate::models::Object>>,
//   /// The identifier of the evaluation run associated with this output item.
//   #[serde(rename = "run_id")]
//   pub run_id: String,
//   #[serde(rename = "sample")]
//   pub sample: crate::models::EvalRunOutputItemSample,
//   /// The status of the evaluation run.
//   #[serde(rename = "status")]
//   pub status: String
// }
