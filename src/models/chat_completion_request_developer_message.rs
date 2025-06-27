/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

use crate::ChatCompletionRequestMessageContentPartText;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ChatCompletionRequestDeveloperMessage {
    /// The contents of the developer message.
    #[serde(rename = "content")]
    pub content: ChatCompletionRequestDeveloperMessageContent,
    /// An optional name for the participant. Provides the model information to differentiate between participants of the same role.
    #[serde(rename = "name")]
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ChatCompletionRequestDeveloperMessageContent {
    Text(String),
    Array(Vec<ChatCompletionRequestMessageContentPartText>),
}

#[test]
fn test_message_developer_message_content() {
    assert_eq!(
        serde_json::to_string(&ChatCompletionRequestDeveloperMessageContent::Text(
            "qwe".into()
        ))
        .unwrap(),
        "\"qwe\"".to_string()
    );
}
