/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

use crate::{
    DoneEvent, ErrorEvent, MessageStreamEvent, RunStepStreamEvent, RunStreamEvent,
    ThreadStreamEvent,
};

/// # on openapi.yaml
///
/// ```yaml
/// AssistantStreamEvent:
///   description: >
///     Represents an event emitted when streaming a Run.
///
///
///     Each event in a server-sent events stream has an `event` and `data`
///     property:
///
///
///     ```
///
///     event: thread.created
///
///     data: {"id": "thread_123", "object": "thread", ...}
///
///     ```
///
///
///     We emit events whenever a new object is created, transitions to a new
///     state, or is being
///
///     streamed in parts (deltas). For example, we emit `thread.run.created`
///     when a new run
///
///     is created, `thread.run.completed` when a run completes, and so on. When
///     an Assistant chooses
///
///     to create a message during a run, we emit a `thread.message.created
///     event`, a
///
///     `thread.message.in_progress` event, many `thread.message.delta` events,
///     and finally a
///
///     `thread.message.completed` event.
///
///
///     We may add additional events over time, so we recommend handling unknown
///     events gracefully
///
///     in your code. See the [Assistants API
///     quickstart](/docs/assistants/overview) to learn how to
///
///     integrate the Assistants API with streaming.
///   oneOf:
///     - $ref: "#/components/schemas/ThreadStreamEvent"
///     - $ref: "#/components/schemas/RunStreamEvent"
///     - $ref: "#/components/schemas/RunStepStreamEvent"
///     - $ref: "#/components/schemas/MessageStreamEvent"
///     - $ref: "#/components/schemas/ErrorEvent"
///     - $ref: "#/components/schemas/DoneEvent"
///   x-oaiMeta:
///     name: Assistant stream events
///     beta: true
/// ````
#[derive(Debug, Serialize, Deserialize)]
pub enum AssistantStreamEvent {
    ThreadStream(ThreadStreamEvent),
    RunStream(RunStreamEvent),
    RunStepStream(RunStepStreamEvent),
    MessageStream(MessageStreamEvent),
    Error(ErrorEvent),
    Done(DoneEvent),
}
