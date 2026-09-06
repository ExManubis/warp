use super::*;

#[test]
fn shared_sessions_and_oz_require_warp_cloud() {
    assert!(FeatureFlag::CreatingSharedSessions.requires_warp_cloud());
    assert!(FeatureFlag::ViewingSharedSessions.requires_warp_cloud());
    assert!(FeatureFlag::CloudMode.requires_warp_cloud());
    assert!(FeatureFlag::OzLaunchModal.requires_warp_cloud());
    assert!(FeatureFlag::FactoryMcp.requires_warp_cloud());
    assert!(!FeatureFlag::AgentMode.requires_warp_cloud());
    assert!(!FeatureFlag::McpServer.requires_warp_cloud());
    assert!(!FeatureFlag::SoloUserByok.requires_warp_cloud());
    assert!(!FeatureFlag::LocalComputerUse.requires_warp_cloud());
}

#[test]
fn local_child_harnesses_are_not_in_debug_flags() {
    assert!(!DEBUG_FLAGS.contains(&FeatureFlag::LocalClaudeCodexChildHarnesses));
}
