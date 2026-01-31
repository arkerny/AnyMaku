<template>
  <div class="overlay-wrapper">
    <transition-group name="danmaku-list" tag="div" class="container">
      <div 
        v-for="item in messages" 
        :key="item.id" 
        class="danmaku-item"
        :style="{ fontSize: config.fontSize + 'px' }"
      >
        <span class="user-name">{{ item.user }}:</span>
        <span class="content">{{ item.text }}</span>
      </div>
    </transition-group>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue';
import { listen } from '@tauri-apps/api/event';

const messages = ref([]);
const config = ref({ fontSize: 24 });

onMounted(() => {
  // 监听新弹幕事件
  listen('new-danmaku', (event) => {
    const id = Date.now();
    messages.value.push({ ...event.payload, id });
    
    // 10秒后自动移除，释放内存
    setTimeout(() => {
      messages.value = messages.value.filter(m => m.id !== id);
    }, 10000);
  });

  // 监听控制台发来的样式更新
  listen('update-config', (event) => {
    config.value = event.payload;
  });
});
</script>

<style scoped>
.overlay-wrapper {
  background: transparent;
  height: 100vh;
  width: 100vw;
  overflow: hidden;
  display: flex;
  flex-direction: column; /* 关键：垂直排列避重叠 */
  padding: 20px;
}

.danmaku-item {
  color: white;
  font-weight: bold;
  margin-bottom: 4px;
  /* 描边效果：确保在白色背景游戏下也能看清 */
  text-shadow: 2px 2px 2px rgba(0,0,0,0.8), -1px -1px 0 #000, 1px -1px 0 #000;
}

.user-name { color: #00aeec; margin-right: 8px; }

/* 动画效果 */
.danmaku-list-enter-active, .danmaku-list-leave-active {
  transition: all 0.4s ease;
}
.danmaku-list-enter-from {
  opacity: 0;
  transform: translateX(20px);
}
.danmaku-list-leave-to {
  opacity: 0;
  transform: translateY(-20px);
}
</style>