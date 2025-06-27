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
pub struct WebSearchLocation {
    /// Free text input for the city of the user, e.g. `San Francisco`.
    #[serde(rename = "city")]
    pub city: Option<String>,
    /// The two-letter  [ISO country code](https://en.wikipedia.org/wiki/ISO_3166-1) of the user, e.g. `US`.
    #[serde(rename = "country")]
    pub country: Option<String>,
    /// Free text input for the region of the user, e.g. `California`.
    #[serde(rename = "region")]
    pub region: Option<String>,
    /// The [IANA timezone](https://timeapi.io/documentation/iana-timezones)  of the user, e.g. `America/Los_Angeles`.
    #[serde(rename = "timezone")]
    pub timezone: Option<String>,
}
