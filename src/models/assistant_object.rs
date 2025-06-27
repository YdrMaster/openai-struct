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
/// AssistantObject:
///   type: object
///   title: Assistant
///   description: Represents an `assistant` that can call the model and use tools.
///   properties:
///     id:
///       description: The identifier, which can be referenced in API endpoints.
///       type: string
///     object:
///       description: The object type, which is always `assistant`.
///       type: string
///       enum:
///         - assistant
///       x-stainless-const: true
///     created_at:
///       description: The Unix timestamp (in seconds) for when the assistant was created.
///       type: integer
///     name:
///       description: |
///         The name of the assistant. The maximum length is 256 characters.
///       type: string
///       maxLength: 256
///       nullable: true
///     description:
///       description: >
///         The description of the assistant. The maximum length is 512
///         characters.
///       type: string
///       maxLength: 512
///       nullable: true
///     model:
///       description: >
///         ID of the model to use. You can use the [List
///         models](/docs/api-reference/models/list) API to see all of your
///         available models, or see our [Model overview](/docs/models) for
///         descriptions of them.
///       type: string
///     instructions:
///       description: >
///         The system instructions that the assistant uses. The maximum length
///         is 256,000 characters.
///       type: string
///       maxLength: 256000
///       nullable: true
///     tools:
///       description: >
///         A list of tool enabled on the assistant. There can be a maximum of
///         128 tools per assistant. Tools can be of types `code_interpreter`,
///         `file_search`, or `function`.
///       default: []
///       type: array
///       maxItems: 128
///       items:
///         oneOf:
///           - $ref: "#/components/schemas/AssistantToolsCode"
///           - $ref: "#/components/schemas/AssistantToolsFileSearch"
///           - $ref: "#/components/schemas/AssistantToolsFunction"
///     tool_resources:
///       type: object
///       description: >
///         A set of resources that are used by the assistant's tools. The
///         resources are specific to the type of tool. For example, the
///         `code_interpreter` tool requires a list of file IDs, while the
///         `file_search` tool requires a list of vector store IDs.
///       properties:
///         code_interpreter:
///           type: object
///           properties:
///             file_ids:
///               type: array
///               description: >
///                 A list of [file](/docs/api-reference/files) IDs made
///                 available to the `code_interpreter`` tool. There can be a
///                 maximum of 20 files associated with the tool.
///               default: []
///               maxItems: 20
///               items:
///                 type: string
///         file_search:
///           type: object
///           properties:
///             vector_store_ids:
///               type: array
///               description: >
///                 The ID of the [vector
///                 store](/docs/api-reference/vector-stores/object) attached to
///                 this assistant. There can be a maximum of 1 vector store
///                 attached to the assistant.
///               maxItems: 1
///               items:
///                 type: string
///       nullable: true
///     metadata:
///       $ref: "#/components/schemas/Metadata"
///     temperature:
///       description: >
///         What sampling temperature to use, between 0 and 2. Higher values
///         like 0.8 will make the output more random, while lower values like
///         0.2 will make it more focused and deterministic.
///       type: number
///       minimum: 0
///       maximum: 2
///       default: 1
///       example: 1
///       nullable: true
///     top_p:
///       type: number
///       minimum: 0
///       maximum: 1
///       default: 1
///       example: 1
///       nullable: true
///       description: >
///         An alternative to sampling with temperature, called nucleus
///         sampling, where the model considers the results of the tokens with
///         top_p probability mass. So 0.1 means only the tokens comprising the
///         top 10% probability mass are considered.
///
///
///         We generally recommend altering this or temperature but not both.
///     response_format:
///       $ref: "#/components/schemas/AssistantsApiResponseFormatOption"
///       nullable: true
///   required:
///     - id
///     - object
///     - created_at
///     - name
///     - description
///     - model
///     - instructions
///     - tools
///     - metadata
///   x-oaiMeta:
///     name: The assistant object
///     beta: true
///     example: >
///       {
///         "id": "asst_abc123",
///         "object": "assistant",
///         "created_at": 1698984975,
///         "name": "Math Tutor",
///         "description": null,
///         "model": "gpt-4o",
///         "instructions": "You are a personal math tutor. When asked a question, write and run Python code to answer the question.",
///         "tools": [
///           {
///             "type": "code_interpreter"
///           }
///         ],
///         "metadata": {},
///         "top_p": 1.0,
///         "temperature": 1.0,
///         "response_format": "auto"
///       }
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct AssistantObject {
    /// The Unix timestamp (in seconds) for when the assistant was created.
    #[serde(rename = "created_at")]
    pub created_at: i32,
    /// The description of the assistant. The maximum length is 512 characters.
    #[serde(rename = "description")]
    pub description: String,
    /// The identifier, which can be referenced in API endpoints.
    #[serde(rename = "id")]
    pub id: String,
    /// The system instructions that the assistant uses. The maximum length is 256,000 characters.
    #[serde(rename = "instructions")]
    pub instructions: String,
    #[serde(rename = "metadata")]
    pub metadata: crate::models::Metadata,
    /// ID of the model to use. You can use the [List models](/docs/api-reference/models/list) API to see all of your available models, or see our [Model overview](/docs/models) for descriptions of them.
    #[serde(rename = "model")]
    pub model: String,
    /// The name of the assistant. The maximum length is 256 characters.
    #[serde(rename = "name")]
    pub name: String,
    /// The object type, which is always `assistant`.
    #[serde(rename = "object")]
    #[serde(default = "default_object")]
    pub object: String,
    #[serde(rename = "response_format")]
    pub response_format: Option<crate::models::AssistantsApiResponseFormatOption>,
    /// What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic.
    #[serde(rename = "temperature")]
    pub temperature: Option<f32>,
    #[serde(rename = "tool_resources")]
    pub tool_resources: Option<crate::models::AssistantObjectToolResources>,
    /// A list of tool enabled on the assistant. There can be a maximum of 128 tools per assistant. Tools can be of types `code_interpreter`, `file_search`, or `function`.
    #[serde(rename = "tools")]
    pub tools: Vec<AssistantObjectTool>,
    /// An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass. So 0.1 means only the tokens comprising the top 10% probability mass are considered.  We generally recommend altering this or temperature but not both.
    #[serde(rename = "top_p")]
    pub top_p: Option<f32>,
}

fn default_object() -> String {
    "assistant".into()
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AssistantObjectTool {
    Code(crate::AssistantToolsCode),
    FileSearch(crate::AssistantToolsFileSearch),
    Function(crate::AssistantToolsFunction),
}
