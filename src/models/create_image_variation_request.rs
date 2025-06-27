/*
 * OpenAI API
 *
 * The OpenAI REST API. Please see pub https://platform.openai.com/docs/api-reference for more details.
 *
 * OpenAPI spec pub version: 2.3.0
 *
 * Generated pub by: https://github.com/swagger-api/swagger-codegen.git
 */

use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateImageVariationRequest {
    /// The image to use as the basis for the variation(s). Must be a valid PNG file, less than 4MB, and square.
    #[serde(rename = "image")]
    pub image: Vec<u8>,
    /// The model to use for image generation. Only `dall-e-2` is supported at this time.
    #[serde(rename = "model")]
    pub model: Option<Value>,
    /// The number of images to generate. Must be between 1 and 10.
    #[serde(rename = "n")]
    pub n: Option<i32>,
    /// The format in which the generated images are returned. Must be one of `url` or `b64_json`. URLs are only valid for 60 minutes after the image has been generated.
    #[serde(rename = "response_format")]
    pub response_format: Option<String>,
    /// The size of the generated images. Must be one of `256x256`, `512x512`, or `1024x1024`.
    #[serde(rename = "size")]
    pub size: Option<String>,
    /// A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse. [Learn more](/docs/guides/safety-best-practices#end-user-ids).
    #[serde(rename = "user")]
    pub user: Option<String>,
}
