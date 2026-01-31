<template>
  <div class="control-container">
    <h2 class="title">AnyMaku 控制台</h2>
    
    <div class="card">
      <label class="section-label">WebSocket 服务地址</label>
      <div class="flex-row">
        <input 
          v-model="wsUrl" 
          placeholder="ws://127.0.0.1:8080" 
          class="styled-input"
          :disabled="isConnected"
        />
        <button 
          @click="handleConnection" 
          :class="['btn', isConnected ? 'danger-btn' : 'primary-btn']"
        >
          {{ isConnected ? '断开连接' : '启动监听' }}
        </button>
      </div>
      <p :class="['status-text', { 'online': isConnected }]">
        状态: {{ isConnected ? '● 已连接' : '○ 未连接' }}
      </p>
    </div>

    <div class="card">
      <label class="section-label">显示设置 (字体: {{ settings.fontSize }}px)</label>
      <input type="range" v-model="settings.fontSize" min="16" max="60" @input="syncConfig" class="slider" />
    </div>
  </div>
</template>

<script setup>
import { ref, reactive, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core' 
import { emit, listen } from '@tauri-apps/api/event'

const wsUrl = ref('ws://127.0.0.1:8080')
const isLocked = ref(true)
const isConnected = ref(false)
const settings = reactive({ fontSize: 24 })

onMounted(async () => {
  // 初始窗口状态
  invoke('set_overlay_ignore_mouse', { ignore: isLocked.value });

  // 监听来自 Rust 的断开通知（如服务器意外关闭或手动断开成功）
  await listen('connection-closed', () => {
    isConnected.value = false;
  });

  // 监听连接错误
  await listen('connection-error', (event) => {
    isConnected.value = false;
    alert("连接出错: " + event.payload);
  });
})

const handleConnection = async () => {
  if (isConnected.value) {
    try {
      await invoke('stop_server_connection');
      // 注意：isConnected 在 listen('conn-closed') 中会自动置为 false
    } catch (err) { console.error(err); }
  } else {
    try {
      await invoke('start_server_connection', { url: wsUrl.value });
      isConnected.value = true;
    } catch (err) { alert(err); }
  }
}

const syncConfig = () => emit('update-config', { ...settings });

</script>

<style scoped>
.control-container { padding: 24px; font-family: system-ui; background: #fdfdfd; min-height: 100vh; }
.card { background: white; padding: 20px; border-radius: 12px; margin-bottom: 20px; border: 1px solid #eaeaea; }
.section-label { display: block; font-size: 14px; font-weight: 600; margin-bottom: 12px; color: #666; }
.flex-row { display: flex; gap: 10px; }
.styled-input { flex: 1; padding: 8px; border: 1px solid #ddd; border-radius: 6px; }
.styled-input:disabled { background: #f5f5f5; color: #999; }
.btn { padding: 8px 16px; border-radius: 6px; border: none; cursor: pointer; font-weight: 500; }
.primary-btn { background: #00aeec; color: white; }
.danger-btn { background: #ff4d4f; color: white; }
.lock-btn { width: 100%; padding: 12px; background: #f0f0f0; }
.lock-btn.is-locked { background: #fff1f0; color: #ff4d4f; border: 1px solid #ffa39e; }
.status-text { font-size: 12px; margin-top: 8px; color: #999; }
.status-text.online { color: #52c41a; }
.slider { width: 100%; }
.divider { height: 1px; background: #eee; margin: 20px 0; }
</style>