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
/// RunStreamEvent:
///   oneOf:
///     - type: object
///       properties:
///         event:
///           type: string
///           enum:
///             - thread.run.created
///           x-stainless-const: true
///         data:
///           $ref: "#/components/schemas/RunObject"
///       required:
///         - event
///         - data
///       description: Occurs when a new [run](/docs/api-reference/runs/object) is created.
///       x-oaiMeta:
///         dataDescription: "`data` is a [run](/docs/api-reference/runs/object)"
///     - type: object
///       properties:
///         event:
///           type: string
///           enum:
///             - thread.run.queued
///           x-stainless-const: true
///         data:
///           $ref: "#/components/schemas/RunObject"
///       required:
///         - event
///         - data
///       description:
///         Occurs when a [run](/docs/api-reference/runs/object) moves to a
///         `queued` status.
///       x-oaiMeta:
///         dataDescription: "`data` is a [run](/docs/api-reference/runs/object)"
///     - type: object
///       properties:
///         event:
///           type: string
///           enum:
///             - thread.run.in_progress
///           x-stainless-const: true
///         data:
///           $ref: "#/components/schemas/RunObject"
///       required:
///         - event
///         - data
///       description:
///         Occurs when a [run](/docs/api-reference/runs/object) moves to an
///         `in_progress` status.
///       x-oaiMeta:
///         dataDescription: "`data` is a [run](/docs/api-reference/runs/object)"
///     - type: object
///       properties:
///         event:
///           type: string
///           enum:
///             - thread.run.requires_action
///           x-stainless-const: true
///         data:
///           $ref: "#/components/schemas/RunObject"
///       required:
///         - event
///         - data
///       description:
///         Occurs when a [run](/docs/api-reference/runs/object) moves to a
///         `requires_action` status.
///       x-oaiMeta:
///         dataDescription: "`data` is a [run](/docs/api-reference/runs/object)"
///     - type: object
///       properties:
///         event:
///           type: string
///           enum:
///             - thread.run.completed
///           x-stainless-const: true
///         data:
///           $ref: "#/components/schemas/RunObject"
///       required:
///         - event
///         - data
///       description: Occurs when a [run](/docs/api-reference/runs/object) is completed.
///       x-oaiMeta:
///         dataDescription: "`data` is a [run](/docs/api-reference/runs/object)"
///     - type: object
///       properties:
///         event:
///           type: string
///           enum:
///             - thread.run.incomplete
///           x-stainless-const: true
///         data:
///           $ref: "#/components/schemas/RunObject"
///       required:
///         - event
///         - data
///       description:
///         Occurs when a [run](/docs/api-reference/runs/object) ends with
///         status `incomplete`.
///       x-oaiMeta:
///         dataDescription: "`data` is a [run](/docs/api-reference/runs/object)"
///     - type: object
///       properties:
///         event:
///           type: string
///           enum:
///             - thread.run.failed
///           x-stainless-const: true
///         data:
///           $ref: "#/components/schemas/RunObject"
///       required:
///         - event
///         - data
///       description: Occurs when a [run](/docs/api-reference/runs/object) fails.
///       x-oaiMeta:
///         dataDescription: "`data` is a [run](/docs/api-reference/runs/object)"
///     - type: object
///       properties:
///         event:
///           type: string
///           enum:
///             - thread.run.cancelling
///           x-stainless-const: true
///         data:
///           $ref: "#/components/schemas/RunObject"
///       required:
///         - event
///         - data
///       description:
///         Occurs when a [run](/docs/api-reference/runs/object) moves to a
///         `cancelling` status.
///       x-oaiMeta:
///         dataDescription: "`data` is a [run](/docs/api-reference/runs/object)"
///     - type: object
///       properties:
///         event:
///           type: string
///           enum:
///             - thread.run.cancelled
///           x-stainless-const: true
///         data:
///           $ref: "#/components/schemas/RunObject"
///       required:
///         - event
///         - data
///       description: Occurs when a [run](/docs/api-reference/runs/object) is cancelled.
///       x-oaiMeta:
///         dataDescription: "`data` is a [run](/docs/api-reference/runs/object)"
///     - type: object
///       properties:
///         event:
///           type: string
///           enum:
///             - thread.run.expired
///           x-stainless-const: true
///         data:
///           $ref: "#/components/schemas/RunObject"
///       required:
///         - event
///         - data
///       description: Occurs when a [run](/docs/api-reference/runs/object) expires.
///       x-oaiMeta:
///         dataDescription: "`data` is a [run](/docs/api-reference/runs/object)"
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct RunStreamEvent {}

// todo: 得从头完全实现
