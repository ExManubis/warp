use super::*;

#[test]
#[ignore = "CORE-3768 - need to clean up PREVIEW_FLAGS, but this is a temporary fix for the cluttered changelog"]
fn test_all_preview_flags_have_a_description() {
    for flag in PREVIEW_FLAGS {
        assert!(
            flag.flag_description()
                .is_some_and(|description| !description.is_empty()),
            "Missing description for preview-enabled flag {flag:?}"
        );
    }
}

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
fn local_child_harnesses_are_local_only_by_default() {
    assert!(LOCAL_FLAGS.contains(&FeatureFlag::LocalClaudeCodexChildHarnesses));
    assert!(!DEBUG_FLAGS.contains(&FeatureFlag::LocalClaudeCodexChildHarnesses));
    assert!(!DOGFOOD_FLAGS.contains(&FeatureFlag::LocalClaudeCodexChildHarnesses));
}
