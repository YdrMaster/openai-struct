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
#[serde(rename_all = "lowercase")]
pub enum ServiceTier {
    Auto,
    Default,
    Flex,
}

impl Default for ServiceTier {
    fn default() -> ServiceTier {
        ServiceTier::Default
    }
}

#[test]
fn test_tier() {
    assert_eq!(
        serde_json::to_value(ServiceTier::Auto).unwrap().to_string(),
        r#""auto""#.to_string()
    );
}
