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
        />
        <button @click="connectWs" class="btn primary-btn">启动监听</button>
      </div>
    </div>

    <div class="card">
      <label class="section-label">显示设置 (字体: {{ settings.fontSize }}px)</label>
      <input 
        type="range" 
        v-model="settings.fontSize" 
        min="16" 
        max="60" 
        @input="syncConfig" 
        class="slider"
      />
      
      <div class="divider"></div>

      <label class="section-label">交互模式</label>
      <button 
        @click="toggleLock" 
        :class="['btn', 'lock-btn', { 'is-locked': isLocked }]"
      >
        <span v-if="isLocked">🔒 已开启鼠标穿透 (无法点击)</span>
        <span v-else>🔓 已关闭鼠标穿透 (可拖动窗口)</span>
      </button>
      <p class="hint">提示：开启穿透后，鼠标将直接点击到下层应用。</p>
    </div>
  </div>
</template>

<script setup>
import { ref, reactive, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core' 
import { emit } from '@tauri-apps/api/event'

const wsUrl = ref('ws://127.0.0.1:8080')
const isLocked = ref(true) // 默认开启穿透，保护用户操作
const settings = reactive({ fontSize: 24 })

// 初始化时同步一次状态
onMounted(() => {
  // 确保启动时窗口就是穿透状态
  invoke('set_overlay_ignore_mouse', { ignore: isLocked.value }).catch(console.error);
})

const connectWs = async () => {
  console.log("正在连接服务器...");
  try {
    await invoke('start_server_connection', { url: wsUrl.value })
  } catch (err) {
    alert("连接失败: " + err)
  }
}

const syncConfig = () => {
  // 将字体大小发送给 Overlay.vue
  emit('update-config', { ...settings })
}

const toggleLock = async () => {
  isLocked.value = !isLocked.value
  try {
    // 调用 Rust 设置窗口穿透属性
    await invoke('set_overlay_ignore_mouse', { ignore: isLocked.value })
  } catch (err) {
    console.error("设置穿透模式失败:", err)
    isLocked.value = !isLocked.value // 失败时回退状态
  }
}
</script>

<style scoped>
.control-container { 
  padding: 24px; 
  font-family: 'Segoe UI', system-ui, sans-serif; 
  color: #333;
  background: #fdfdfd;
  min-height: 100vh;
}

.title { margin-top: 0; margin-bottom: 24px; font-size: 20px; color: #1a1a1a; }

.card { 
  background: #ffffff; 
  padding: 20px; 
  border-radius: 12px; 
  margin-bottom: 20px; 
  border: 1px solid #eaeaea;
  box-shadow: 0 4px 6px rgba(0,0,0,0.02);
}

.section-label { 
  display: block; 
  font-size: 14px; 
  font-weight: 600; 
  margin-bottom: 12px; 
  color: #666;
}

.flex-row { display: flex; gap: 10px; }

.styled-input { 
  flex: 1; 
  padding: 8px 12px; 
  border: 1px solid #ddd; 
  border-radius: 6px;
  outline: none;
  transition: border-color 0.2s;
}
.styled-input:focus { border-color: #00aeec; }

.btn { 
  padding: 8px 16px; 
  border-radius: 6px; 
  border: none; 
  cursor: pointer; 
  font-weight: 500;
  transition: all 0.2s;
}

.primary-btn { background: #00aeec; color: white; }
.primary-btn:hover { background: #0092c8; }

.lock-btn { 
  width: 100%; 
  padding: 12px; 
  background: #f0f0f0; 
  color: #444; 
}
.lock-btn.is-locked { 
  background: #fff1f0; 
  color: #ff4d4f; 
  border: 1px solid #ffa39e;
}

.slider { width: 100%; cursor: pointer; }

.divider { height: 1px; background: #eee; margin: 20px 0; }

.hint { font-size: 12px; color: #999; margin-top: 10px; }
</style>