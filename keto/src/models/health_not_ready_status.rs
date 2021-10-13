/*
 * ORY Keto
 *
 * Ory Keto is a cloud native access control server providing best-practice patterns (RBAC, ABAC, ACL, AWS IAM Policies, Kubernetes Roles, ...) via REST APIs.
 *
 * The version of the OpenAPI document: Latest
 * Contact: hi@ory.sh
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct HealthNotReadyStatus {
    /// Errors contains a list of errors that caused the not ready status.
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<::std::collections::HashMap<String, String>>,
}

impl HealthNotReadyStatus {
    pub fn new() -> HealthNotReadyStatus {
        HealthNotReadyStatus {
            errors: None,
        }
    }
}


