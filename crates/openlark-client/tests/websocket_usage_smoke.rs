#![cfg(feature = "websocket")]
//! WebSocket 用法 smoke test（不走真实网络）
//!
//! ### 运行方式（仓库根目录）
//! `cargo test -p openlark-client --features websocket --test websocket_usage_smoke -- --nocapture`
//!
//! 说明：
//! - 这个测试不会去请求飞书的 `/callback/ws/endpoint`，因此不需要网络与真实凭证。
//! - 主要验证：`ws_client` 模块能被启用、Frame 构造/处理流程可运行。

use lark_websocket_protobuf::pbbp2::{Frame, Header};
use openlark_client::ws_client::FrameHandler;
use openlark_core::event::dispatcher::EventDispatcherHandler;
use tokio::sync::mpsc;

#[tokio::test]
async fn test_frame_handler_event_smoke() {
    let handler = EventDispatcherHandler::builder().build();
    let (tx, _rx) = mpsc::unbounded_channel();

    // 构造一个最小的 v2 事件 payload。
    // 由于当前 dispatcher 没有注册任何处理器，预期返回 error(code=500)。
    let frame = Frame {
        seq_id: 1,
        log_id: 1,
        service: 1,
        method: 1, // data frame
        headers: vec![
            Header {
                key: "type".to_string(),
                value: "event".to_string(),
            },
            Header {
                key: "message_id".to_string(),
                value: "msg_123".to_string(),
            },
            Header {
                key: "trace_id".to_string(),
                value: "trace_456".to_string(),
            },
        ],
        payload_encoding: None,
        payload_type: None,
        payload: Some(
            br#"{"schema":"2.0","header":{"event_type":"im.message.receive_v1","token":"t"},"event":{}}"#
                .to_vec(),
        ),
        log_id_new: None,
    };

    let resp = FrameHandler::handle_frame(frame, &handler, &tx)
        .await
        .expect("should return response frame for event");

    let payload = resp.payload.expect("response payload should exist");
    let payload_str = String::from_utf8(payload).expect("response payload should be utf8 json");

    assert!(
        payload_str.contains(r#""code":500"#),
        "unexpected response payload: {payload_str}"
    );
}

#[test]
fn test_build_ping_frame_smoke() {
    let frame = FrameHandler::build_ping_frame(42);
    assert_eq!(frame.service, 42);
    assert_eq!(frame.method, 0);
    assert_eq!(frame.headers.len(), 1);
    assert_eq!(frame.headers[0].key, "type");
    assert_eq!(frame.headers[0].value, "ping");
}

