//! Query engine for SigmaOS package metadata.
//!
//! Provides search, filter, and retrieval over installed package records.
//! Supports package name, description, version, and dependency queries. Dependency-free.

use std::collections::HashMap;
use std::fmt;

/// Errors for query operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QueryError {
    EmptyQuery,
    PackageNotFound(String),
}

impl fmt::Display for QueryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyQuery => write!(f, "query must not be empty"),
            Self::PackageNotFound(n) => write!(f, "package not found: {n}"),
        }
    }
}

impl std::error::Error for QueryError {}

/// Package metadata record.
#[derive(Debug, Clone, PartialEq)]
pub struct PackageRecord {
    pub name: String,
    pub version: String,
    pub description: String,
    pub dependencies: Vec<String>,
}

impl PackageRecord {
    pub fn new(name: impl Into<String>, version: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            description: description.into(),
            dependencies: Vec::new(),
        }
    }

    pub fn add_dependency(&mut self, dep: impl Into<String>) {
        self.dependencies.push(dep.into());
    }
}

/// Query engine over package records.
#[derive(Debug, Clone, Default)]
pub struct QueryEngine {
    packages: HashMap<String, PackageRecord>,
}

impl QueryEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_package(&mut self, pkg: PackageRecord) {
        self.packages.insert(pkg.name.clone(), pkg);
    }

    pub fn get_package(&self, name: &str) -> Result<&PackageRecord, QueryError> {
        self.packages.get(name).ok_or_else(|| QueryError::PackageNotFound(name.to_string()))
    }

    pub fn search_by_name(&self, pattern: &str) -> Vec<&PackageRecord> {
        if pattern.trim().is_empty() {
            return Vec::new();
        }
        self.packages.values()
            .filter(|p| p.name.contains(pattern) || p.description.contains(pattern))
            .collect()
    }

    pub fn search_by_dependency(&self, dep: &str) -> Vec<&PackageRecord> {
        self.packages.values()
            .filter(|p| p.dependencies.iter().any(|d| d == dep))
            .collect()
    }

    pub fn list_all(&self) -> Vec<&PackageRecord> {
        self.packages.values().collect()
    }

    pub fn count(&self) -> usize {
        self.packages.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_by_name_aaa() {
        // Arrange
        let mut qe = QueryEngine::new();
        qe.add_package(PackageRecord::new("bash", "5.2", "GNU Bourne Again Shell"));
        qe.add_package(PackageRecord::new("vim", "9.0", "Vi IMproved text editor"));

        // Act
        let results = qe.search_by_name("sh");

        // Assert
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "bash");
    }

    #[test]
    fn test_search_by_dependency_aaa() {
        // Arrange
        let mut qe = QueryEngine::new();
        let mut pkg = PackageRecord::new("firefox", "120", "Web browser");
        pkg.add_dependency("gtk3");
        pkg.add_dependency("qt5");
        qe.add_package(pkg);

        // Act
        let results = qe.search_by_dependency("gtk3");

        // Assert
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "firefox");
    }

    #[test]
    fn test_get_package_aaa() {
        // Arrange
        let mut qe = QueryEngine::new();
        qe.add_package(PackageRecord::new("zsh", "5.9", "Z shell"));

        // Act
        let res = qe.get_package("zsh");

        // Assert
        assert!(res.is_ok());
        assert_eq!(res.unwrap().description, "Z shell");
        assert!(qe.get_package("nonexistent").is_err());
    }
}
