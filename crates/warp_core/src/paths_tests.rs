use dirs::home_dir;

use super::*;

#[test]
fn test_data_dir_path() {
    let home_dir = home_dir().expect("Should be able to compute home directory");
    cfg_if::cfg_if! {
        if #[cfg(target_os = "macos")] {
            assert_eq!(data_dir(), home_dir.join(".promptty"));
        } else if #[cfg(any(target_os = "linux", target_os = "freebsd"))] {
            assert_eq!(data_dir(), home_dir.join(".local/share/promptty"));
        } else if #[cfg(windows)] {
            assert_eq!(data_dir(), home_dir.join("AppData\\Roaming\\promptty\\PrompTTY\\data"));
        } else {
            unimplemented!("Need to update tests for current platform!");
        }
    }
}

#[test]
fn test_config_local_dir_path() {
    let home_dir = home_dir().expect("Should be able to compute home directory");
    cfg_if::cfg_if! {
        if #[cfg(target_os = "macos")] {
            assert_eq!(config_local_dir(), home_dir.join(".promptty"));
        } else if #[cfg(any(target_os = "linux", target_os = "freebsd"))] {
            assert_eq!(config_local_dir(), home_dir.join(".config/promptty"));
        } else if #[cfg(windows)] {
            assert_eq!(config_local_dir(), home_dir.join("AppData\\Local\\promptty\\PrompTTY\\config"));
        } else {
            unimplemented!("Need to update tests for current platform!");
        }
    }
}

#[cfg(target_os = "macos")]
#[test]
fn test_macos_config_dir_name_scopes_to_data_profile() {
    assert_eq!(
        macos_config_dir_name_for(Channel::Release, None),
        ".promptty"
    );
    assert_eq!(
        macos_config_dir_name_for(Channel::Integration, None),
        ".promptty-integration"
    );

    assert_eq!(
        macos_config_dir_name_for(Channel::Integration, Some("myprofile")),
        ".promptty-integration-myprofile"
    );
    assert_eq!(
        macos_config_dir_name_for(Channel::Release, Some("myprofile")),
        ".promptty-myprofile"
    );
}

#[test]
fn test_gui_app_id_maps_tui_to_gui() {
    let gui_app_id = gui_app_id_for_channel(
        Channel::Release,
        AppId::new("dev", "promptty", "PrompTTYTui"),
    );

    assert_eq!(gui_app_id.to_string(), "dev.promptty.PrompTTY");
}

#[test]
fn test_gui_config_and_mcp_paths_resolve_explicit_sources() {
    let home_dir = home_dir().expect("Should be able to compute home directory");
    let gui_config_dir = gui_config_local_dir().expect("GUI config path should resolve");

    cfg_if::cfg_if! {
        if #[cfg(target_os = "macos")] {
            assert_eq!(gui_config_dir, home_dir.join(".promptty"));
        } else if #[cfg(any(target_os = "linux", target_os = "freebsd"))] {
            assert_eq!(gui_config_dir, home_dir.join(".config/promptty"));
        } else if #[cfg(windows)] {
            assert_eq!(
                gui_config_dir,
                home_dir.join("AppData\\Local\\promptty\\PrompTTY\\config")
            );
        } else {
            unimplemented!("Need to update tests for current platform!");
        }
    }

    assert_eq!(gui_mcp_config_file_path(), warp_home_mcp_config_file_path());
}

#[test]
fn test_warp_home_config_dir_path() {
    let home_dir = home_dir().expect("Should be able to compute home directory");
    let expected_dir_name = match ChannelState::data_profile() {
        Some(data_profile) => format!(".promptty-{data_profile}"),
        None => ".promptty".to_string(),
    };

    assert_eq!(
        warp_home_config_dir(),
        Some(home_dir.join(expected_dir_name))
    );
}

#[test]
fn test_warp_home_skills_and_mcp_paths() {
    let Some(config_dir) = warp_home_config_dir() else {
        panic!("Should be able to compute Warp home config directory");
    };

    assert_eq!(warp_home_skills_dir(), Some(config_dir.join("skills")));
    assert_eq!(
        warp_home_mcp_config_file_path(),
        Some(config_dir.join(".mcp.json"))
    );
}

#[test]
fn test_tui_mcp_config_path_is_separate_from_gui() {
    let tui_mcp_path = tui_mcp_config_file_path();

    assert_eq!(tui_mcp_path, tui_config_local_dir().join(".mcp.json"));
    assert_ne!(
        Some(tui_mcp_path),
        warp_home_mcp_config_file_path(),
        "GUI and TUI MCP configuration must remain isolated"
    );
}

#[test]
fn test_tui_config_dir_is_promptty_tui() {
    let home_dir = home_dir().expect("Should be able to compute home directory");
    let expected_dir_name = match ChannelState::data_profile() {
        Some(data_profile) => format!(".promptty-tui-{data_profile}"),
        None => ".promptty-tui".to_string(),
    };
    assert_eq!(tui_config_local_dir(), home_dir.join(expected_dir_name));
}

#[test]
fn test_cache_dir_path() {
    let home_dir = home_dir().expect("Should be able to compute home directory");
    cfg_if::cfg_if! {
        if #[cfg(target_os = "macos")] {
            assert_eq!(cache_dir(), home_dir.join("Library/Application Support/dev.promptty.PrompTTY"));
        } else if #[cfg(any(target_os = "linux", target_os = "freebsd"))] {
            assert_eq!(cache_dir(), home_dir.join(".cache/promptty"));
        } else if #[cfg(windows)] {
            assert_eq!(cache_dir(), home_dir.join("AppData\\Local\\promptty\\PrompTTY\\cache"));
        } else {
            unimplemented!("Need to update tests for current platform!");
        }
    }
}

#[test]
fn test_state_dir_path() {
    let home_dir = home_dir().expect("Should be able to compute home directory");
    cfg_if::cfg_if! {
        if #[cfg(target_os = "macos")] {
            assert_eq!(state_dir(), home_dir.join("Library/Application Support/dev.promptty.PrompTTY"));
        } else if #[cfg(any(target_os = "linux", target_os = "freebsd"))] {
            assert_eq!(state_dir(), home_dir.join(".local/state/promptty"));
        } else if #[cfg(windows)] {
            assert_eq!(state_dir(), home_dir.join("AppData\\Local\\promptty\\PrompTTY\\data"));
        } else {
            unimplemented!("Need to update tests for current platform!");
        }
    }
}

#[test]
fn test_tui_state_dir_is_tui_subdir_of_gui_state_base() {
    let tui_dir = tui_state_dir();
    assert_eq!(tui_dir.file_name(), Some(std::ffi::OsStr::new("tui")));

    let gui_state_base = secure_state_dir().unwrap_or_else(state_dir);
    assert_eq!(tui_dir.parent(), Some(gui_state_base.as_path()));
}

#[test]
fn test_project_path_for_promptty_app_id() {
    let project_dirs = project_dirs_for_app_id(AppId::new("dev", "promptty", "PrompTTY"), None)
        .expect("should be able to compute project dirs");
    cfg_if::cfg_if! {
        if #[cfg(target_os = "macos")] {
            assert_eq!(project_dirs.project_path(), "dev.promptty.PrompTTY");
        } else if #[cfg(any(target_os = "linux", target_os = "freebsd"))] {
            assert_eq!(project_dirs.project_path(), "promptty");
        } else if #[cfg(windows)] {
            assert_eq!(project_dirs.project_path(), "promptty\\PrompTTY");
        } else {
            unimplemented!("Need to update tests for current platform!");
        }
    }
}
