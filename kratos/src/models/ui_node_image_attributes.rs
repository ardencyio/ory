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
pub struct UiNodeImageAttributes {
    /// The image's source URL.  format: uri
    #[serde(rename = "src")]
    pub src: String,
}

impl UiNodeImageAttributes {
    pub fn new(src: String) -> UiNodeImageAttributes {
        UiNodeImageAttributes {
            src,
        }
    }
}


