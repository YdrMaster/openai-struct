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
/// Certificate:
///   type: object
///   description: Represents an individual `certificate` uploaded to the organization.
///   properties:
///     object:
///       type: string
///       enum:
///         - certificate
///         - organization.certificate
///         - organization.project.certificate
///       description: >
///         The object type.
///
///
///         - If creating, updating, or getting a specific certificate, the
///         object type is `certificate`.
///
///         - If listing, activating, or deactivating certificates for the
///         organization, the object type is `organization.certificate`.
///
///         - If listing, activating, or deactivating certificates for a
///         project, the object type is `organization.project.certificate`.
///       x-stainless-const: true
///     id:
///       type: string
///       description: The identifier, which can be referenced in API endpoints
///     name:
///       type: string
///       description: The name of the certificate.
///     created_at:
///       type: integer
///       description: The Unix timestamp (in seconds) of when the certificate was uploaded.
///     certificate_details:
///       type: object
///       properties:
///         valid_at:
///           type: integer
///           description:
///             The Unix timestamp (in seconds) of when the certificate becomes
///             valid.
///         expires_at:
///           type: integer
///           description: The Unix timestamp (in seconds) of when the certificate expires.
///         content:
///           type: string
///           description: The content of the certificate in PEM format.
///     active:
///       type: boolean
///       description:
///         Whether the certificate is currently active at the specified scope.
///         Not returned when getting details for a specific certificate.
///   required:
///     - object
///     - id
///     - name
///     - created_at
///     - certificate_details
///   x-oaiMeta:
///     name: The certificate object
///     example: >
///       {
///         "object": "certificate",
///         "id": "cert_abc",
///         "name": "My Certificate",
///         "created_at": 1234567,
///         "certificate_details": {
///           "valid_at": 1234567,
///           "expires_at": 12345678,
///           "content": "-----BEGIN CERTIFICATE----- MIIGAjCCA...6znFlOW+ -----END CERTIFICATE-----"
///         }
///       }
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct Certificate {
    /// Whether the certificate is currently active at the specified scope. Not returned when getting details for a specific certificate.
    #[serde(rename = "active")]
    pub active: Option<bool>,
    #[serde(rename = "certificate_details")]
    pub certificate_details: crate::models::CertificateCertificateDetails,
    /// The Unix timestamp (in seconds) of when the certificate was uploaded.
    #[serde(rename = "created_at")]
    pub created_at: i32,
    /// The identifier, which can be referenced in API endpoints
    #[serde(rename = "id")]
    pub id: String,
    /// The name of the certificate.
    #[serde(rename = "name")]
    pub name: String,
    /// The object type.  - If creating, updating, or getting a specific certificate, the object type is `certificate`. - If listing, activating, or deactivating certificates for the organization, the object type is `organization.certificate`. - If listing, activating, or deactivating certificates for a project, the object type is `organization.project.certificate`.
    #[serde(rename = "object")]
    pub object: String,
}
