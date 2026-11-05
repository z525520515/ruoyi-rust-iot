<template>
  <router-view />
  <div v-if="isDemo" class="demo-badge">演示模式 · Mock 数据</div>
</template>

<script setup>
import useSettingsStore from '@/store/modules/settings'
import { handleThemeStyle } from '@/utils/theme'

// GitHub Pages 纯前端演示模式标识
const isDemo = import.meta.env.VITE_APP_DEMO === 'true'

onMounted(() => {
  nextTick(() => {
    // 初始化主题样式
    handleThemeStyle(useSettingsStore().theme)
  })
})
</script>

<style>
.demo-badge {
  position: fixed;
  right: 18px;
  bottom: 18px;
  z-index: 3000;
  padding: 6px 12px;
  border-radius: 14px;
  background: rgba(64, 158, 255, 0.92);
  color: #fff;
  font-size: 12px;
  line-height: 1;
  pointer-events: none;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
}
</style>
