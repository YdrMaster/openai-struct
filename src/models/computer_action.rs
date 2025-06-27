/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https:///platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https:///github.com/swagger-api/swagger-codegen.git
 */

use crate::{
    Click,
    DoubleClick,
    Drag,
    KeyPress,
    Screenshot,
    Scroll,
    Wait,
    // Type, Move,
};

/// # on openapi.yaml
///
/// ```yaml
/// ComputerAction:
///   oneOf:
///     - $ref: "#/components/schemas/Click"
///     - $ref: "#/components/schemas/DoubleClick"
///     - $ref: "#/components/schemas/Drag"
///     - $ref: "#/components/schemas/KeyPress"
///     - $ref: "#/components/schemas/Move"
///     - $ref: "#/components/schemas/Screenshot"
///     - $ref: "#/components/schemas/Scroll"
///     - $ref: "#/components/schemas/Type"
///     - $ref: "#/components/schemas/Wait"
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub enum ComputerAction {
    Click(Click),
    DoubleClick(DoubleClick),
    Drag(Drag),
    KeyPress(KeyPress),
    Screenshot(Screenshot),
    Scroll(Scroll),
    Wait(Wait),
    // todo: 这俩类型没自动生成
    Type,
    Move,
}
