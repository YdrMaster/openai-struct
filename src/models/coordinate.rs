/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https:///platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https:///github.com/swagger-api/swagger-codegen.git
 */

/// # on openapi.yaml
///
/// ```yaml
/// Coordinate:
///   type: object
///   title: Coordinate
///   description: |
///     An x/y coordinate pair, e.g. `{ x: 100, y: 200 }`.
///   properties:
///     x:
///       type: integer
///       description: |
///         The x-coordinate.
///     y:
///       type: integer
///       description: |
///         The y-coordinate.
///   required:
///     - x
///     - y
/// ```
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Coordinate {
    /// The x-coordinate.
    #[serde(rename = "x")]
    pub x: i32,
    /// The y-coordinate.
    #[serde(rename = "y")]
    pub y: i32,
}
