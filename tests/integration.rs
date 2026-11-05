use sigmaos_arch_signoff::*;

#[test]
fn component_is_constructible_and_runnable() {
    // Arrange
    let c = sigmaos_arch_signoff::new("sigmaos-arch-signoff").expect("valid name");
    // Act
    let out = c.run().expect("run succeeds");
    // Assert
    assert!(out.contains("sigmaos-arch-signoff"));
}
