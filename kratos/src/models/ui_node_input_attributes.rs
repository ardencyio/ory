/*
 * Ory Kratos
 *
 * Welcome to the Ory Kratos HTTP API documentation!
 *
 * The version of the OpenAPI document: latest
 * 
 * Generated by: https://openapi-generator.tech
 */

/// UiNodeInputAttributes : InputAttributes represents the attributes of an input node



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct UiNodeInputAttributes {
    /// Sets the input's disabled field to true or false.
    #[serde(rename = "disabled")]
    pub disabled: bool,
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<Box<crate::models::UiText>>,
    /// The input's element name.
    #[serde(rename = "name")]
    pub name: String,
    /// The input's pattern.
    #[serde(rename = "pattern", skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    /// Mark this input field as required.
    #[serde(rename = "required", skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(rename = "type")]
    pub _type: String,
    /// The input's value.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
}

impl UiNodeInputAttributes {
    /// InputAttributes represents the attributes of an input node
    pub fn new(disabled: bool, name: String, _type: String) -> UiNodeInputAttributes {
        UiNodeInputAttributes {
            disabled,
            label: None,
            name,
            pattern: None,
            required: None,
            _type,
            value: None,
        }
    }
}


