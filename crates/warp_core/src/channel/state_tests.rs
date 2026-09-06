use super::{derive_http_origin_from_ws_url, origin_from_server_root_url};
use url::Url;

#[test]
fn wss_becomes_https_and_strips_path() {
    let got = derive_http_origin_from_ws_url("wss://rtc.app.warp.dev/graphql/v2");
    assert_eq!(got.as_deref(), Some("https://rtc.app.warp.dev"));
}

#[test]
fn ws_becomes_http_and_preserves_port() {
    let got = derive_http_origin_from_ws_url("ws://localhost:8080/graphql/v2");
    assert_eq!(got.as_deref(), Some("http://localhost:8080"));
}

#[test]
fn unparseable_input_returns_none() {
    assert!(derive_http_origin_from_ws_url("not a url").is_none());
    assert!(derive_http_origin_from_ws_url("https://app.warp.dev").is_none());
}

#[test]
fn empty_server_root_url_uses_loopback_origin() {
    let got = origin_from_server_root_url("");
    let want = Url::parse("http://127.0.0.1")
        .expect("fallback origin is valid")
        .origin();
    assert_eq!(got, want);
}

#[test]
fn invalid_server_root_url_uses_loopback_origin() {
    let got = origin_from_server_root_url("not a url");
    let want = Url::parse("http://127.0.0.1")
        .expect("fallback origin is valid")
        .origin();
    assert_eq!(got, want);
}
