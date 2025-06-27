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
/// FunctionParameters:
///   type: object
///   description: >-
///     The parameters the functions accepts, described as a JSON Schema object.
///     See the [guide](/docs/guides/function-calling) for examples, and the
///     [JSON Schema reference](https:///json-schema.org/understanding-json-schema/) for
///     documentation about the format.
///
///
///     Omitting `parameters` defines a function with an empty parameter list.
///   additionalProperties: true
/// ```
pub type FunctionParameters = serde_json::Value;
