/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

/// # on openapi.yaml
///
/// ```yaml
/// ResponseFormatJsonSchema:
///   type: object
///   title: JSON schema
///   description: |
///     JSON Schema response format. Used to generate structured JSON responses.
///     Learn more about [Structured Outputs](/docs/guides/structured-outputs).
///   properties:
///     type:
///       type: string
///       description: The type of response format being defined. Always `json_schema`.
///       enum:
///         - json_schema
///       x-stainless-const: true
///     json_schema:
///       type: object
///       title: JSON schema
///       description: |
///         Structured Outputs configuration options, including a JSON Schema.
///       properties:
///         description:
///           type: string
///           description: >
///             A description of what the response format is for, used by the
///             model to
///
///             determine how to respond in the format.
///         name:
///           type: string
///           description: >
///             The name of the response format. Must be a-z, A-Z, 0-9, or
///             contain
///
///             underscores and dashes, with a maximum length of 64.
///         schema:
///           $ref: "#/components/schemas/ResponseFormatJsonSchemaSchema"
///         strict:
///           type: boolean
///           nullable: true
///           default: false
///           description: >
///             Whether to enable strict schema adherence when generating the
///             output.
///
///             If set to true, the model will always follow the exact schema
///             defined
///
///             in the `schema` field. Only a subset of JSON Schema is supported
///             when
///
///             `strict` is `true`. To learn more, read the [Structured Outputs
///
///             guide](/docs/guides/structured-outputs).
///       required:
///         - name
///   required:
///     - type
///     - json_schema
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseFormatJsonSchema {
    #[serde(rename = "json_schema")]
    pub json_schema: crate::models::JsonSchema,
}
