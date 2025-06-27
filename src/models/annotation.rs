/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

use crate::{FileCitationBody, FilePath, UrlCitationBody};

/// # on openapi.yaml
///
/// ```yaml
/// Annotation:
///   oneOf:
///     - $ref: "#/components/schemas/FileCitationBody"
///     - $ref: "#/components/schemas/UrlCitationBody"
///     - $ref: "#/components/schemas/FilePath"
///   discriminator:
///     propertyName: type
/// ```
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Annotation {
    #[serde(rename = "file_citation")]
    FileCitation(FileCitationBody),
    #[serde(rename = "url_citation")]
    UrlCitation(UrlCitationBody),
    #[serde(rename = "file_path")]
    FilePath(FilePath),
}
