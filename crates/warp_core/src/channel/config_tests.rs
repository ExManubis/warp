use super::{ChannelConfig, WarpServerConfig};
use crate::AppId;

#[test]
fn local_only_config_has_no_warp_hosts() {
    let config = ChannelConfig::local_only(AppId::new("dev", "warp", "WarpOss"), "warp-oss.log");

    assert!(!config.cloud_enabled());
    assert!(config.server_config.is_none());
    assert!(config.oz_config.is_none());

    let json = serde_json::to_string(&config).expect("channel config should serialize");
    assert!(
        !json.contains("warp.dev"),
        "OSS config must not embed Warp hosts: {json}"
    );
    assert!(
        !json.contains("AIza"),
        "OSS config must not embed a Firebase API key: {json}"
    );
}

#[test]
fn missing_server_and_oz_fields_deserialize_as_disabled() {
    let json = r#"{
        "app_id": "dev.warp.WarpOss",
        "logfile_name": "warp-oss.log"
    }"#;

    let config: ChannelConfig =
        serde_json::from_str(json).expect("partial config should deserialize");
    assert!(!config.cloud_enabled());
    assert!(config.oz_config.is_none());
}

#[test]
fn present_server_config_deserializes_as_enabled() {
    let json = r#"{
        "app_id": "dev.warp.WarpOss",
        "logfile_name": "warp-oss.log",
        "server_config": {
            "server_root_url": "http://192.0.2.0:9",
            "rtc_server_url": "ws://192.0.2.0:9/graphql/v2",
            "session_sharing_server_url": null,
            "firebase_auth_api_key": ""
        }
    }"#;

    let config: ChannelConfig =
        serde_json::from_str(json).expect("server config should deserialize");
    assert!(config.cloud_enabled());
    let server: &WarpServerConfig = config
        .server_config
        .as_ref()
        .expect("server config present");
    assert_eq!(server.server_root_url, "http://192.0.2.0:9");
}
