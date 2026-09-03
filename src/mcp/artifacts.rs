use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ArtifactType {
    #[serde(rename = "function/python")]
    PythonFunction,
    #[serde(rename = "function/rust")]
    RustFunction,
    #[serde(rename = "function/perl")]
    PerlFunction,
    #[serde(rename = "function/ruby")]
    RubyFunction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artifact {
    pub artifact_type: ArtifactType,
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub version: String,
    pub code: String,
    pub dependencies: Vec<String>,
    pub parameters: serde_json::Value,
    pub metadata: Option<serde_json::Value>,
}

impl Artifact {
    pub fn new(
        artifact_type: ArtifactType,
        id: String,
        name: String,
        code: String,
    ) -> Self {
        Self {
            artifact_type,
            id,
            name,
            description: None,
            version: "0.1.0".to_string(),
            code,
            dependencies: vec![],
            parameters: serde_json::json!({}),
            metadata: None,
        }
    }

    pub fn with_dependencies(mut self, deps: Vec<String>) -> Self {
        self.dependencies = deps;
        self
    }

    pub fn with_description(mut self, desc: String) -> Self {
        self.description = Some(desc);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_artifact_creation() {
        let artifact = Artifact::new(
            ArtifactType::PythonFunction,
            "fn_123".to_string(),
            "my_function".to_string(),
            "def my_function(): pass".to_string(),
        );
        
        assert_eq!(artifact.name, "my_function");
        assert_eq!(artifact.artifact_type, ArtifactType::PythonFunction);
        assert_eq!(artifact.version, "0.1.0");
    }
}
