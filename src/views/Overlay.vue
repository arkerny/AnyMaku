<template>

  <div class="danmaku-container">

    <div
      v-for="msg in danmuQueue"
      :key="msg.id"
      class="danmaku-item"
      :style="{
        top: msg.top + 'px',
        fontSize: config.fontSize + 'px',
        '--duration': msg.duration + 's'
      }"
    >

      <span class="user-name">{{ msg.user }}:</span>

      <span class="content">{{ msg.text }}</span>

    </div>

  </div>

</template>


<script setup>

import { ref, onMounted } from 'vue'

import { listen } from '@tauri-apps/api/event'


const danmuQueue = ref([])

const config = ref({ fontSize: 24 })


// 计算轨道，防止弹幕在垂直方向重叠

const getTrackTop = () => {

  // 根据当前字号动态计算轨道高度

  const trackHeight = config.value.fontSize + 15;

  const maxTracks = Math.floor((window.innerHeight - 100) / trackHeight);

  const randomTrack = Math.floor(Math.random() * Math.max(maxTracks, 1));

  return randomTrack * trackHeight + 30;

}


onMounted(async () => {

  // 1. 监听来自控制台的配置更新

  await listen('update-config', (event) => {

    if (event.payload.fontSize) {

      config.value.fontSize = event.payload.fontSize;

    }

  });


  // 2. 监听来自 Rust 的新弹幕

  await listen('new-danmaku', (event) => {

    let parsedData = event.payload;


    // 如果 Rust 传过来的是 JSON 字符串则解析，如果是对象则直接用

    if (typeof parsedData === 'string') {

      try { parsedData = JSON.parse(parsedData); } catch (e) { }

    }


    const id = Date.now() + Math.random();

    const newMsg = {

      id,

      user: parsedData.user || "未知用户",

      text: parsedData.text || (typeof parsedData === 'string' ? parsedData : ""),

      top: getTrackTop(),

      duration: 8 + Math.random() * 4 // 8-12秒随机速度

    };


    danmuQueue.value.push(newMsg);


    // 动画结束后的自动清理

    setTimeout(() => {

      danmuQueue.value = danmuQueue.value.filter(m => m.id !== id);

    }, 13000); // 略长于最大动画时间

  });

});

</script>


<style scoped>

.danmaku-container {

  position: fixed;

  top: 0;

  left: 0;

  width: 100vw;

  height: 100vh;

  /* 确保这里没有任何背景颜色，甚至是 rgba(0,0,0,0) 有时也会有问题 */

  background: none !important;

  /* 强制该容器及其子元素不响应任何鼠标事件 */

  pointer-events: none !important;

  user-select: none;

  overflow: hidden;

  z-index: 999999;

}


.danmaku-item {

  position: absolute;

  left: 100vw;

  white-space: nowrap;

  color: white;

  font-weight: bold;

  /* 关键：增加文字外发光或阴影，保证在各种背景下清晰可见 */

  text-shadow:

    1px 1px 2px #000,

    0 0 1em #000,

    0 0 0.2em #000;

  display: flex;

  align-items: center;

  will-change: transform; /* 提升性能，减少抖动 */

  animation: scrollLeft var(--duration) linear forwards;

  pointer-events: none !important;

}


.user-name {

  color: #00aeec;

  margin-right: 8px;

}


.content {

  color: #ffffff;

}


@keyframes scrollLeft {

  from {

    transform: translateX(0);

  }

  to {

    transform: translateX(calc(-100vw - 100%));

  }

}

</style>
