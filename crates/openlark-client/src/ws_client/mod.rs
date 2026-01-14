// WebSocket 客户端模块
//
// 提供WebSocket连接和事件处理功能

mod client;
mod frame_handler;
mod state_machine;

#[cfg(feature = "websocket")]
pub use client::*;

/// 事件分发处理器（从 core 重新导出，方便使用）
#[cfg(feature = "websocket")]
pub use openlark_core::event::dispatcher::EventDispatcherHandler;

pub use frame_handler::{FrameHandler, FrameType};
pub use state_machine::{ConnectionState, WebSocketStateMachine};

#[cfg(test)]
mod tests;
