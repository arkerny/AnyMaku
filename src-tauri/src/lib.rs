pub mod websocket {
    use tauri::{Window, Emitter, Manager};
    use futures_util::StreamExt;
    use tokio_tungstenite::connect_async;
    use serde::{Serialize, Deserialize}; // 引入 Deserialize

    // 定义一个结构体来匹配 Python 发来的原始 JSON 格式
    #[derive(Debug, Deserialize, Serialize, Clone)]
    struct Danmaku {
        user: String,
        text: String,
    }

    // 启动 WebSocket 连接并处理消息
    #[tauri::command]
    pub async fn start_server_connection(window: Window, url: String) -> Result<(), String> {
        tokio::spawn(async move {
            let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
            let (_, mut read) = ws_stream.split();
            
            while let Some(Ok(msg)) = read.next().await {
                if msg.is_text() {
                    let raw_text = msg.to_text().unwrap();
                    if let Ok(parsed) = serde_json::from_str::<Danmaku>(raw_text) {
                        window.emit("new-danmaku", parsed).unwrap();
                    }
                }
            }
        });
        Ok(())
    }

    // 点击穿透
    #[tauri::command]
    pub async fn set_overlay_ignore_mouse(app: tauri::AppHandle, ignore: bool) -> Result<(), String> {
        // 获取真正的 webview 窗口实例
        let window = app.get_webview_window("overlay")
            .ok_or("找不到 overlay 窗口")?;

        // 核心调用
        window.set_ignore_cursor_events(ignore)
            .map_err(|e| e.to_string())?;

        println!("窗口穿透状态已切换为: {}", ignore);
        Ok(())
    }
}