//! WebSocket 事件订阅示例（长连接）
//!
//! 运行前需要设置环境变量：
//! - OPENLARK_APP_ID
//! - OPENLARK_APP_SECRET
//! - OPENLARK_BASE_URL（可选，默认 https://open.feishu.cn；国际站可用 https://open.larksuite.com）
//!
//! 运行：
//! `cargo run --example websocket_events --features websocket`

use open_lark::client::ws_client::LarkWsClient;
use open_lark::event::dispatcher::EventDispatcherHandler;
use open_lark::client::Config;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 从环境变量读取配置
    let config = Config::from_env();
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("debug"),
    )
    .init();

    // 当前仓库的事件分发器支持解析事件结构，但默认没有注册任何具体 handler。
    // 你可以先跑通连接（确认能连上/能收帧），再按需要注册你自己的事件处理器。
    let handler = EventDispatcherHandler::builder()
        // v2 事件的派发 key 规则：`p2.{header.event_type}`
        // 例如：`p2.im.message.receive_v1`
        .register("p2.im.message.receive_v1", |payload| {
            // 这里拿到的是“事件原始 JSON bytes”（未解密/未验签版本的最小示例）
            // 你可以在这里做：
            // - serde_json::from_slice 解析
            // - 打日志
            // - 转发到你自己的业务队列/handler
            let s = String::from_utf8_lossy(payload);
            println!("[event] im.message.receive_v1 payload={s}");
            Ok(())
        })
        .build();

    // 注意：此调用会持续运行（内部循环收事件），不会自己退出
    LarkWsClient::open(Arc::new(config), handler).await?;

    Ok(())
}

