// Integration test for SigmaOS-arch-query
// Verifies the crate builds and links as a dependency.

use sigmaos_arch_query::{PackageRecord, QueryEngine};

#[test]
fn crate_links_and_runs() {
    // Arrange
    let mut qe = QueryEngine::new();
    qe.add_package(PackageRecord::new("bash", "5.2", "GNU Bourne Again Shell"));

    // Act
    let results = qe.search_by_name("bash");

    // Assert
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].name, "bash");
}
