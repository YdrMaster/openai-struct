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
///     ApproximateLocation:
///       properties:
///         type:
///           type: string
///           enum:
///             - approximate
///           description: The type of location approximation. Always `approximate`.
///           default: approximate
///           x-stainless-const: true
///         country:
///           anyOf:
///             - type: string
///               description: The two-letter [ISO country
///                 code](https://en.wikipedia.org/wiki/ISO_3166-1) of the user,
///                 e.g. `US`.
///             - type: "null"
///         region:
///           anyOf:
///             - type: string
///               description: Free text input for the region of the user, e.g. `California`.
///             - type: "null"
///         city:
///           anyOf:
///             - type: string
///               description: Free text input for the city of the user, e.g. `San Francisco`.
///             - type: "null"
///         timezone:
///           anyOf:
///             - type: string
///               description: The [IANA
///                 timezone](https://timeapi.io/documentation/iana-timezones) of
///                 the user, e.g. `America/Los_Angeles`.
///             - type: "null"
///       type: object
///       required:
///         - type
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct ApproximateLocation {
    #[serde(rename = "city")]
    pub city: Option<String>,
    #[serde(rename = "country")]
    pub country: Option<String>,
    #[serde(rename = "region")]
    pub region: Option<String>,
    #[serde(rename = "timezone")]
    pub timezone: Option<String>,
    /// The type of location approximation. Always `approximate`.
    #[serde(rename = "type")]
    #[serde(default = "default_type")]
    pub _type: String,
}

fn default_type() -> String {
    "approximate".into()
}
