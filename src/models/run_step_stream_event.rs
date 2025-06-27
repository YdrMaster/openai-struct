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
/// RunStepStreamEvent:
///   oneOf:
///     - type: object
///       properties:
///         event:
///           type: string
///           enum:
///             - thread.run.step.created
///           x-stainless-const: true
///         data:
///           $ref: "#/components/schemas/RunStepObject"
///       required:
///         - event
///         - data
///       description:
///         Occurs when a [run step](/docs/api-reference/run-steps/step-object)
///         is created.
///       x-oaiMeta:
///         dataDescription: "`data` is a [run step](/docs/api-reference/run-steps/step-object)"
///     - type: object
///       properties:
///         event:
///           type: string
///           enum:
///             - thread.run.step.in_progress
///           x-stainless-const: true
///         data:
///           $ref: "#/components/schemas/RunStepObject"
///       required:
///         - event
///         - data
///       description:
///         Occurs when a [run step](/docs/api-reference/run-steps/step-object)
///         moves to an `in_progress` state.
///       x-oaiMeta:
///         dataDescription: "`data` is a [run step](/docs/api-reference/run-steps/step-object)"
///     - type: object
///       properties:
///         event:
///           type: string
///           enum:
///             - thread.run.step.delta
///           x-stainless-const: true
///         data:
///           $ref: "#/components/schemas/RunStepDeltaObject"
///       required:
///         - event
///         - data
///       description: Occurs when parts of a [run
///         step](/docs/api-reference/run-steps/step-object) are being streamed.
///       x-oaiMeta:
///         dataDescription: "`data` is a [run step
///           delta](/docs/api-reference/assistants-streaming/run-step-delta-ob\
///           ject)"
///     - type: object
///       properties:
///         event:
///           type: string
///           enum:
///             - thread.run.step.completed
///           x-stainless-const: true
///         data:
///           $ref: "#/components/schemas/RunStepObject"
///       required:
///         - event
///         - data
///       description:
///         Occurs when a [run step](/docs/api-reference/run-steps/step-object)
///         is completed.
///       x-oaiMeta:
///         dataDescription: "`data` is a [run step](/docs/api-reference/run-steps/step-object)"
///     - type: object
///       properties:
///         event:
///           type: string
///           enum:
///             - thread.run.step.failed
///           x-stainless-const: true
///         data:
///           $ref: "#/components/schemas/RunStepObject"
///       required:
///         - event
///         - data
///       description:
///         Occurs when a [run step](/docs/api-reference/run-steps/step-object)
///         fails.
///       x-oaiMeta:
///         dataDescription: "`data` is a [run step](/docs/api-reference/run-steps/step-object)"
///     - type: object
///       properties:
///         event:
///           type: string
///           enum:
///             - thread.run.step.cancelled
///           x-stainless-const: true
///         data:
///           $ref: "#/components/schemas/RunStepObject"
///       required:
///         - event
///         - data
///       description:
///         Occurs when a [run step](/docs/api-reference/run-steps/step-object)
///         is cancelled.
///       x-oaiMeta:
///         dataDescription: "`data` is a [run step](/docs/api-reference/run-steps/step-object)"
///     - type: object
///       properties:
///         event:
///           type: string
///           enum:
///             - thread.run.step.expired
///           x-stainless-const: true
///         data:
///           $ref: "#/components/schemas/RunStepObject"
///       required:
///         - event
///         - data
///       description:
///         Occurs when a [run step](/docs/api-reference/run-steps/step-object)
///         expires.
///       x-oaiMeta:
///         dataDescription: "`data` is a [run step](/docs/api-reference/run-steps/step-object)"
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct RunStepStreamEvent {}

// todo: 之后逐个实现
