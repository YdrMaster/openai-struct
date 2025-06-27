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
pub struct ResponseTextAnnotationDeltaEvent {
    #[serde(rename = "annotation")]
    pub annotation: crate::models::Annotation,
    /// The index of the annotation that was added.
    #[serde(rename = "annotation_index")]
    pub annotation_index: i32,
    /// The index of the content part that the text annotation was added to.
    #[serde(rename = "content_index")]
    pub content_index: i32,
    /// The ID of the output item that the text annotation was added to.
    #[serde(rename = "item_id")]
    pub item_id: String,
    /// The index of the output item that the text annotation was added to.
    #[serde(rename = "output_index")]
    pub output_index: i32,
    /// The type of the event. Always `response.output_text.annotation.added`.
    #[serde(rename = "type")]
    pub _type: String,
}
