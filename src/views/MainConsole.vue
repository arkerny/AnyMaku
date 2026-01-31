<template>
  <div class="control-container">
    <h2 class="title">AnyMaku 控制台</h2>

    <div class="card">
      <div class="flex-row">
        <button @click="clearOverlay" class="btn secondary-btn">一键清屏</button>
        <button @click="handleExit" class="btn danger-btn">关闭程序</button>
      </div>
    </div>
    

    <div class="card">
      <label class="section-label">WebSocket 服务地址</label>
      <div class="flex-row">
        <input v-model="wsUrl" placeholder="ws://127.0.0.1:8080" class="styled-input" :disabled="isConnected" />
        <button @click="handleConnection" :class="['btn', isConnected ? 'danger-btn' : 'primary-btn']">
          {{ isConnected ? '断开连接' : '启动监听' }}
        </button>
      </div>
      <p :class="['status-text', { 'online': isConnected }]">状态: {{ isConnected ? '● 已连接' : '○ 未连接' }}</p>
    </div>

    <div class="card">
      <label class="section-label">实时显示设置</label>
      
      <div class="setting-group">
        <label>字体大小: {{ settings.fontSize }}px</label>
        <input type="range" v-model.number="settings.fontSize" min="16" max="60" @input="syncConfig" class="slider" />
      </div>

      <div class="setting-group">
        <label>移动速度 {{ settings.speed }}s:</label>
        <input type="range" v-model.number="settings.speed" min="4" max="20" @input="syncConfig" class="slider" />
      </div>

      <div class="divider"></div>

      <div class="setting-item">
        <div class="flex-row">
          <input type="checkbox" v-model="settings.enableShadow" @change="syncConfig" id="shadow-toggle" />
          <label for="shadow-toggle">启用阴影</label>
        </div>
        <input type="range" v-model.number="settings.shadowStrength" min="1" max="15" :disabled="!settings.enableShadow" @input="syncConfig" class="slider-small" />
      </div>

      <div class="setting-item">
        <div class="flex-row">
          <input type="checkbox" v-model="settings.enableStroke" @change="syncConfig" id="stroke-toggle" />
          <label for="stroke-toggle">启用描边</label>
        </div>
        <div class="flex-row" v-if="settings.enableStroke">
        <input type="color" v-model="settings.strokeColor" @input="syncConfig" class="color-mini" title="描边颜色" />
        <div class="slider-container">
          <span class="tiny-label">粗细: {{ settings.strokeWidth }}</span>
          <input 
            type="range" 
            v-model.number="settings.strokeWidth" 
            min="0.5" 
            max="5" 
            step="0.1" 
            @input="syncConfig" 
            class="slider-small" 
          />
        </div>
      </div>
      </div>
      <div class="divider"></div>
      <label class="section-label">显示区域控制 (垂直范围)</label>

      <div class="setting-item">
        <span class="label-text">起始位置: {{ settings.displayTop }}%</span>
        <div class="slider-container">
          <input type="range" v-model.number="settings.displayTop" min="0" :max="settings.displayBottom" @input="syncConfig" class="slider" />
        </div>
      </div>

      <div class="setting-item">
        <span class="label-text">结束位置: {{ settings.displayBottom }}%</span>
        <div class="slider-container">
          <input type="range" v-model.number="settings.displayBottom" :min="settings.displayTop" max="100" @input="syncConfig" class="slider" />
        </div>
      </div>
      <div class="divider"></div>

      <div class="setting-item">
        <div class="flex-row">
          <input type="checkbox" v-model="settings.rainbowUser" @change="syncConfig" id="rb-user" />
          <label for="rb-user">用户彩虹色</label>
        </div>
        <div class="flex-row">
          <input type="checkbox" v-model="settings.rainbowText" @change="syncConfig" id="rb-text" />
          <label for="rb-text">文字彩虹色</label>
        </div>
      </div>

      <div class="divider"></div>

      <div class="flex-row jc-around">
        <div class="color-picker">
          <label>用户名颜色</label>
          <input type="color" v-model="settings.userColor" @input="syncConfig" />
        </div>
        <div class="color-picker">
          <label>弹幕颜色</label>
          <input type="color" v-model="settings.textColor" @input="syncConfig" />
        </div>
      </div>
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

const settings = reactive({ 
  fontSize: 24,
  speed: 10,
  userColor: '#00aeec',
  textColor: '#ffffff',
  // 阴影
  enableShadow: true,
  shadowStrength: 2,
  // 描边
  enableStroke: false,
  strokeColor: '#000000',
  strokeWidth: 1.5,
  // 彩虹色
  rainbowUser: false,
  rainbowText: false,
  displayTop: 10,
  displayBottom: 50
})

onMounted(async () => {
  invoke('set_overlay_ignore_mouse', { ignore: isLocked.value });
  await listen('connection-closed', () => { isConnected.value = false; });
  await listen('connection-error', (event) => {
    isConnected.value = false;
    alert("连接出错: " + event.payload);
  });
})

const handleConnection = async () => {
  if (isConnected.value) {
    try { await invoke('stop_server_connection'); } catch (err) { console.error(err); }
  } else {
    try {
      await invoke('start_server_connection', { url: wsUrl.value });
      isConnected.value = true;
    } catch (err) { alert(err); }
  }
}

const syncConfig = () => emit('update-config', { ...settings });

const handleExit = async () => {
  if (confirm('确定要退出程序并关闭所有弹幕吗？')) {
    await invoke('stop_server_connection'); // 断开 WebSocket 连接
    await emit('clear-all'); // 触发 Overlay.vue 中的监听
    setTimeout(async () => {
      await invoke('exit_app'); // 需要你在 Rust 后端实现此指令
    }, 100);
  }
}

const clearOverlay = async () => {
  await emit('clear-all'); // 触发 Overlay.vue 中的监听
}

</script>

<style>
  html, body {
    margin: 0;
    padding: 0;
    width: 100%;
    height: 100%;
    overflow: hidden; 
    background-color: transparent;
  }
  #app {
    height: 100%;
  }
</style>

<style scoped>
.title { font-size: 32px; font-weight: 700; margin-bottom: 24px; color: #222; } 
.control-container { padding: 24px; font-family: system-ui; background: #fdfdfd; min-height: 100vh; }
.card { background: white; padding: 20px; border-radius: 12px; margin-bottom: 20px; border: 1px solid #eaeaea; }
.section-label { display: block; font-size: 24px; font-weight: 600; margin-bottom: 16px; color: #444; border-left: 3px solid #00aeec; padding-left: 8px; }.flex-row { display: flex; gap: 10px; align-items: center; }
.jc-around { justify-content: space-around; }
.setting-group { margin-bottom: 15px; }
.setting-item { display: flex; justify-content: space-between; align-items: center; margin-bottom: 12px; }
.divider { height: 1px; background: #eee; margin: 15px 0; }
.color-picker { text-align: center; font-size: 12px; }
.color-mini { width: 24px; height: 24px; border: none; padding: 0; background: none; cursor: pointer; }
.slider { width: 100%; }
.slider-small { width: 100px; }
.styled-input { flex: 1; padding: 8px; border: 1px solid #ddd; border-radius: 6px; }
.btn { padding: 8px 16px; border-radius: 6px; border: none; cursor: pointer; font-weight: 500; }
.primary-btn { background: #00aeec; color: white; }
.primary-btn:hover { background: #33bfff; }
.secondary-btn { 
  background: #f0f0f0; 
  color: #666; 
  border: 1px solid #ddd;
}
.secondary-btn:hover { background: #e5e5e5; }
.danger-btn { background: #ff4d4f; color: white; }
.danger-btn:hover { background: #ff7875; }
.status-text { font-size: 12px; margin-top: 8px; color: #999; }
.status-text.online { color: #52c41a; }
</style>