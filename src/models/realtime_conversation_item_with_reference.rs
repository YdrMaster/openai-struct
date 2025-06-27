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
pub struct RealtimeConversationItemWithReference {
    /// The arguments of the function call (for `function_call` items).
    #[serde(rename = "arguments")]
    pub arguments: Option<String>,
    /// The ID of the function call (for `function_call` and  `function_call_output` items). If passed on a `function_call_output`  item, the server will check that a `function_call` item with the same  ID exists in the conversation history.
    #[serde(rename = "call_id")]
    pub call_id: Option<String>,
    /// The content of the message, applicable for `message` items.  - Message items of role `system` support only `input_text` content - Message items of role `user` support `input_text` and `input_audio`    content - Message items of role `assistant` support `text` content.
    #[serde(rename = "content")]
    pub content: Option<Vec<crate::models::RealtimeConversationItemContent>>,
    /// For an item of type (`message` | `function_call` | `function_call_output`) this field allows the client to assign the unique ID of the item. It is not required because the server will generate one if not provided.  For an item of type `item_reference`, this field is required and is a reference to any item that has previously existed in the conversation.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// The name of the function being called (for `function_call` items).
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// Identifier for the API object being returned - always `realtime.item`.
    #[serde(rename = "object")]
    pub object: Option<String>,
    /// The output of the function call (for `function_call_output` items).
    #[serde(rename = "output")]
    pub output: Option<String>,
    /// The role of the message sender (`user`, `assistant`, `system`), only  applicable for `message` items.
    #[serde(rename = "role")]
    pub role: Option<String>,
    /// The status of the item (`completed`, `incomplete`). These have no effect  on the conversation, but are accepted for consistency with the  `conversation.item.created` event.
    #[serde(rename = "status")]
    pub status: Option<String>,
    /// The type of the item (`message`, `function_call`, `function_call_output`, `item_reference`).
    #[serde(rename = "type")]
    pub _type: Option<String>,
}
