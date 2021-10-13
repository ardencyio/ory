/*
 * Ory Kratos
 *
 * Welcome to the Ory Kratos HTTP API documentation!
 *
 * The version of the OpenAPI document: latest
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UiText {
    /// The message's context. Useful when customizing messages.
    #[serde(rename = "context", skip_serializing_if = "Option::is_none")]
    pub context: Option<serde_json::Value>,
    #[serde(rename = "id")]
    pub id: i64,
    /// The message text. Written in american english.
    #[serde(rename = "text")]
    pub text: String,
    #[serde(rename = "type")]
    pub _type: String,
}

impl UiText {
    pub fn new(id: i64, text: String, _type: String) -> UiText {
        UiText {
            context: None,
            id,
            text,
            _type,
        }
    }
}

