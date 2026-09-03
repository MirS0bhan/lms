/// Function Registry
/// 
/// Central registry for managing registered functions,
/// their metadata, and dependencies.

use dashmap::DashMap;
use std::sync::Arc;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use crate::error::{LmsError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionMetadata {
    pub id: String,
    pub name: String,
    pub runtime: String,
    pub version: String,
    pub dependencies: Vec<String>,
    pub whitelist_mode: bool,
    pub created_at: String,
    pub updated_at: String,
}

impl FunctionMetadata {
    pub fn new(name: String, runtime: String) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            runtime,
            version: "0.1.0".to_string(),
            dependencies: vec![],
            whitelist_mode: true,
            created_at: now.clone(),
            updated_at: now,
        }
    }
}

pub struct FunctionRegistry {
    functions: DashMap<String, FunctionMetadata>,
}

impl FunctionRegistry {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            functions: DashMap::new(),
        })
    }

    pub fn register(&self, metadata: FunctionMetadata) -> Result<String> {
        if self.functions.contains_key(&metadata.name) {
            return Err(LmsError::FunctionAlreadyExists(metadata.name));
        }
        
        let id = metadata.id.clone();
        self.functions.insert(metadata.name, metadata);
        Ok(id)
    }

    pub fn unregister(&self, name: &str) -> Result<()> {
        self.functions
            .remove(name)
            .ok_or_else(|| LmsError::FunctionNotFound(name.to_string()))?;
        Ok(())
    }

    pub fn get(&self, name: &str) -> Result<FunctionMetadata> {
        self.functions
            .get(name)
            .map(|entry| entry.clone())
            .ok_or_else(|| LmsError::FunctionNotFound(name.to_string()))
    }

    pub fn list(&self) -> Vec<FunctionMetadata> {
        self.functions
            .iter()
            .map(|entry| entry.value().clone())
            .collect()
    }

    pub fn exists(&self, name: &str) -> bool {
        self.functions.contains_key(name)
    }
}

impl Default for FunctionRegistry {
    fn default() -> Self {
        Self {
            functions: DashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_function() {
        let registry = FunctionRegistry::new();
        let metadata = FunctionMetadata::new("test_fn".to_string(), "python".to_string());
        
        let result = registry.register(metadata.clone());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), metadata.id);
    }

    #[test]
    fn test_duplicate_registration() {
        let registry = FunctionRegistry::new();
        let metadata = FunctionMetadata::new("test_fn".to_string(), "python".to_string());
        
        let _ = registry.register(metadata.clone());
        let result = registry.register(metadata);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_function() {
        let registry = FunctionRegistry::new();
        let metadata = FunctionMetadata::new("test_fn".to_string(), "python".to_string());
        
        registry.register(metadata.clone()).unwrap();
        let retrieved = registry.get("test_fn").unwrap();
        assert_eq!(retrieved.name, "test_fn");
    }
}
