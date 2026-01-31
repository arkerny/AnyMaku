pub mod websocket {
    use tauri::{Window, Emitter, State};
    use futures_util::StreamExt;
    use tokio_tungstenite::connect_async;
    use tokio_util::sync::CancellationToken;
    use std::sync::Mutex;
    use serde::{Serialize, Deserialize};
    use log::{error, info};

    pub struct ConnectionState {
        // 使用 Mutex 保证多线程安全，存储可选的取消令牌
        pub token: Mutex<Option<CancellationToken>>,
    }

    // 定义一个结构体来匹配 Websocket 发来的原始 JSON 格式
    #[derive(Debug, Deserialize, Serialize, Clone)]
    struct Danmaku {
        user: String,
        text: String,
    }

    // 启动 WebSocket 连接并处理消息
    #[tauri::command]
    pub async fn start_server_connection(
        window: Window, 
        url: String,
        state: State<'_, ConnectionState> 
    ) -> Result<(), String> {
        // --- 防止重复连接 ---
        let mut token_lock = state.token.lock().unwrap();
        if let Some(old_token) = token_lock.take() {
            info!(target: "websocket", "已有连接，正在断开旧连接...");
            old_token.cancel(); // 如果已有连接，先关掉它
        }

        // --- 创建并存储新令牌 ---
        let token = CancellationToken::new();
        let cloned_token = token.clone();
        *token_lock = Some(token);
        drop(token_lock); // 释放锁，避免阻塞异步任务

        // --- 启动后台任务 ---
        tokio::spawn(async move {
            let ws_result = connect_async(&url).await;
            let (ws_stream, _) = match ws_result {
                Ok(val) => {
                    info!(target: "websocket", "成功连接到服务器: {}", url);
                    let _ = window.emit("connection-succeed", "已连接到服务器");
                    val
                },
                Err(e) => {
                    error!(target: "websocket", "连接失败: {}", e);
                    let _ = window.emit("connection-failed", e.to_string());
                    return;
                }
            };

            let (_, mut read) = ws_stream.split();

            loop {
                tokio::select! {
                    // 监听手动取消信号
                    _ = cloned_token.cancelled() => {
                        info!(target: "websocket", "连接被手动取消: {}", url);
                        break; 
                    }
                    // 监听 WebSocket 消息
                    msg = read.next() => {
                        match msg {
                            Some(Ok(message)) => {
                                if let Ok(raw_text) = message.into_text() {
                                    if let Ok(parsed) = serde_json::from_str::<Danmaku>(&raw_text) {
                                        let _ = window.emit("new-danmaku", parsed.clone());
                                        info!(target: "websocket", "收到弹幕: {}", parsed.text);
                                    }
                                }
                            }
                            _ => break, // 连接被服务器关闭或报错
                        }
                    }
                }
            }
            let _ = window.emit("connection-closed", "已断开连接");
            info!(target: "websocket", "连接已关闭: {}", url);
        });

        Ok(())
    }

    #[tauri::command]
    pub async fn stop_server_connection(state: State<'_, ConnectionState>) -> Result<(), String> {
        // 触发令牌，通知上面的 loop 停止
        let mut token_lock = state.token.lock().unwrap();
        if let Some(token) = token_lock.take() {
            info!(target: "websocket", "正在断开连接...");
            token.cancel();
            // 发送取消信号后再输出一条日志确认
            info!(target: "websocket", "已成功发送取消信号，连接将关闭");
            Ok(())
        } else {
            Err("当前没有活跃的连接".into())
        }
    }
}

pub mod window {
    use tauri::{Manager, Runtime, AppHandle};
    use log::{error, info};
    // 点击穿透
    #[tauri::command]
    pub async fn set_overlay_ignore_mouse<R: Runtime>(
        app: AppHandle<R>, 
        ignore: bool
    ) -> Result<(), String> {
        let window = app.get_webview_window("overlay")
            .ok_or_else(|| {
                let err_msg = "Window 'overlay' not found".to_string();
                error!("{}", err_msg);
                err_msg
            })?;

        window.set_ignore_cursor_events(ignore)
            .map_err(|e| {
                let err_msg = format!("Failed to set ignore_cursor_events: {}", e);
                error!("{}", err_msg);
                err_msg
            })?;

        info!(target: "window_management", "Overlay ignore mouse: {}", ignore);
        
        Ok(())
    }
}

pub mod application {
    #[tauri::command]
    pub fn exit_app(app_handle: tauri::AppHandle) {
        app_handle.exit(0);
    }
}