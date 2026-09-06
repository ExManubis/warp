use super::*;

#[test]
fn test_repo_name() {
    assert_eq!(repo_name(Channel::Release), "promptty");
    assert_eq!(repo_name(Channel::Integration), "promptty-integration");
}
