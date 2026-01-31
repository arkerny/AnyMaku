<template>
  <div class="danmaku-container">
    <div
      v-for="msg in danmuQueue"
      :key="msg.id"
      class="danmaku-item"
      :style="{
        top: `${msg.top}px`,
        fontSize: `${config.fontSize}px`,
        '--duration': `${msg.duration}s`
      }"
      @animationend="removeDanmu(msg.id)"
    >
      <span class="user-name">{{ msg.user }}:</span>
      <span class="content">{{ msg.text }}</span>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted } from 'vue';
import { listen } from '@tauri-apps/api/event';

/**
 * 状态定义
 */
const danmuQueue = ref([]);
const config = ref({
  fontSize: 24,
  trackHeightOffset: 15,
  baseDuration: 8
});

/**
 * 核心逻辑：移除弹幕
 */
const removeDanmu = (id) => {
  danmuQueue.value = danmuQueue.value.filter(m => m.id !== id);
};

/**
 * 核心逻辑：计算轨道位置
 */
const getTrackTop = () => {
  const rowHeight = config.value.fontSize + config.value.trackHeightOffset;
  const maxTracks = Math.floor((window.innerHeight - 100) / rowHeight);
  const randomTrack = Math.floor(Math.random() * Math.max(maxTracks, 1));
  
  return (randomTrack * rowHeight) + 40;
};

/**
 * 核心逻辑：处理并压入新弹幕
 */
const pushDanmaku = (payload) => {
  let data = payload;
  
  // 类型安全解析
  if (typeof data === 'string') {
    try {
      data = JSON.parse(data);
    } catch (e) {
      data = { text: data };
    }
  }

  const id = Date.now() + Math.random();
  const newMsg = {
    id,
    user: data.user || "未知用户",
    text: data.text || (typeof data === 'string' ? data : ""),
    top: getTrackTop(),
    duration: config.value.baseDuration + (Math.random() * 4)
  };

  danmuQueue.value.push(newMsg);
};

/**
 * 生命周期与事件监听
 */
let unlistenConfig;
let unlistenDanmu;

onMounted(async () => {
  // 监听配置更新
  unlistenConfig = await listen('update-config', (event) => {
    if (event.payload.fontSize) {
      config.value.fontSize = event.payload.fontSize;
    }
  });

  // 监听新弹幕
  unlistenDanmu = await listen('new-danmaku', (event) => {
    pushDanmaku(event.payload);
  });
});

onUnmounted(() => {
  if (unlistenConfig) unlistenConfig();
  if (unlistenDanmu) unlistenDanmu();
});
</script>

<style scoped>
/* 容器：全屏透明，禁止交互 */
.danmaku-container {
  position: fixed;
  inset: 0;
  background: transparent !important;
  pointer-events: none !important;
  user-select: none;
  overflow: hidden;
  z-index: 999999;
}

/* 弹幕单条样式 */
.danmaku-item {
  position: absolute;
  left: 100vw;
  white-space: nowrap;
  display: flex;
  align-items: center;
  will-change: transform;
  animation: scrollLeft var(--duration) linear forwards;
  
  /* 字体视觉增强 */
  color: #ffffff;
  font-weight: bold;
  text-shadow:
    -1px -1px 0 #000,
     1px -1px 0 #000,
    -1px  1px 0 #000,
     1px  1px 0 #000,
     0 2px 4px rgba(0, 0, 0, 0.5);
}

.user-name {
  color: #00aeec;
  margin-right: 8px;
}

.content {
  color: #ffffff;
}

/* 滚动动画 */
@keyframes scrollLeft {
  from {
    transform: translateX(0);
  }
  to {
    transform: translateX(calc(-100vw - 100%));
  }
}
</style>