<template>
  <div class="danmaku-container">
    <div
      v-for="msg in danmuQueue"
      :key="msg.id"
      class="danmaku-item"
      :style="{
        top: (msg.trackIndex * (config.fontSize + 15) + 30) + 'px',
        fontSize: config.fontSize + 'px',
        '--duration': msg.duration + 's',
        // 动态合成阴影和描边
        textShadow: generateTextStyle()
      }"
    >
      <span 
        class="user-name" 
        :class="{ 'rainbow-text': config.rainbowUser }"
        :style="{ color: config.rainbowUser ? 'transparent' : config.userColor }"
      >
        {{ msg.user }}:
      </span>

      <span 
        class="content" 
        :class="{ 'rainbow-text': config.rainbowText }"
        :style="{ color: config.rainbowText ? 'transparent' : config.textColor }"
      >
        {{ msg.text }}
      </span>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { listen } from '@tauri-apps/api/event'

const danmuQueue = ref([])
const config = ref({ 
  fontSize: 24, speed: 10, userColor: '#00aeec', textColor: '#ffffff',
  enableShadow: true, shadowStrength: 2,
  enableStroke: false, strokeColor: '#000000', strokeWidth: 1.5
})

// 核心函数：生成复杂的 CSS text-shadow
const generateTextStyle = () => {
  let shadows = [];
  
  // 处理描边 (采用 8 方向弥散算法，使描边更圆滑)
  if (config.value.enableStroke) {
    const w = config.value.strokeWidth;
    const c = config.value.strokeColor;
    shadows.push(
      `${w}px 0 0 ${c}`, `-${w}px 0 0 ${c}`, 
      `0 ${w}px 0 ${c}`, `0 -${w}px 0 ${c}`,
      `${w * 0.7}px ${w * 0.7}px 0 ${c}`, `-${w * 0.7}px ${w * 0.7}px 0 ${c}`,
      `${w * 0.7}px -${w * 0.7}px 0 ${c}`, `-${w * 0.7}px -${w * 0.7}px 0 ${c}`
    );
  }

  // 处理阴影 (放在描边之后，确保阴影在最底层)
  if (config.value.enableShadow) {
    const s = config.value.shadowStrength;
    // 增加一个偏置阴影和一个发光阴影，增加立体感
    shadows.push(`${s}px ${s}px ${s}px rgba(0,0,0,0.8)`, `0 0 ${s * 1.5}px #000`);
  }

  return shadows.length > 0 ? shadows.join(', ') : 'none';
}

const getTrackIndex = () => {
  const trackHeight = config.value.fontSize + 15;
  const maxTracks = Math.floor((window.innerHeight - 100) / trackHeight);
  return Math.floor(Math.random() * Math.max(maxTracks, 1));
}

onMounted(async () => {
  await listen('update-config', (event) => {
    config.value = { ...config.value, ...event.payload };
  });

  await listen('new-danmaku', (event) => {
    let parsedData = event.payload;
    if (typeof parsedData === 'string') {
      try { parsedData = JSON.parse(parsedData); } catch (e) { }
    }

    const id = Date.now() + Math.random();
    const newMsg = {
      id,
      user: parsedData.user || "未知用户",
      text: parsedData.text || (typeof parsedData === 'string' ? parsedData : ""),
      trackIndex: getTrackIndex(),
      duration: config.value.speed + (Math.random() * 2) 
    };

    danmuQueue.value.push(newMsg);
    setTimeout(() => {
      danmuQueue.value = danmuQueue.value.filter(m => m.id !== id);
    }, (config.value.speed + 5) * 1000);
  });
});
</script>

<style scoped>
.danmaku-container {
  position: fixed; top: 0; left: 0; width: 100vw; height: 100vh;
  background: none !important; pointer-events: none !important;
  user-select: none; overflow: hidden; z-index: 999999;
}
.danmaku-item {
  position: absolute; left: 100vw; white-space: nowrap;
  color: white; font-weight: bold; display: flex; align-items: center;
  will-change: transform; animation: scrollLeft var(--duration) linear forwards;
  pointer-events: none !important;
}
.user-name { margin-right: 8px; }
@keyframes scrollLeft {
  from { transform: translateX(0); }
  to { transform: translateX(calc(-100vw - 100%)); }
}
.rainbow-text {
  /* 1. 首尾颜色必须一致：这里开头和结尾都是 #ff0000 (红色) */
  background: linear-gradient(
    135deg, 
    #ff0000, #ffff00, #00ff00, #00ffff, #0000ff, #ff00ff, #ff0000
  );
  
  /* 2. 背景宽度设为 200%，这样一半是完整的彩虹，另一半用于循环过渡 */
  background-size: 200% 100%;
  
  -webkit-background-clip: text !important;
  background-clip: text !important;
  color: transparent !important;
  display: inline-block;
  
  /* 3. 动画：使用 linear 确保匀速，通过移动 background-position 实现无缝滚动 */
  animation: rainbow-scroll 3s linear infinite !important;
}

@keyframes rainbow-scroll {
  0% {
    background-position: 0% 0%;
  }
  100% {
    /* 移动到 200% 或 -200% 取决于你想要的滚动方向 */
    background-position: 200% 0%;
  }
}
</style>