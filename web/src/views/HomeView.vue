<script setup lang="ts">
import { useWebSocketStore } from '../store/ws'
const webSocketStore = useWebSocketStore()
</script>

<template>
  <div class="home-container">
    <el-text style="font-size: var(--el-font-size-large)">
      Instructions
    </el-text>
    <br>
    <br>
    <br>
    <el-text style="font-size: var(--el-font-size-base)">
      Please enter the WebSocket URL in the "Settings" section on the left.
      <br>
      <br>
      The URL you are about to connect to is: <span style="color: #66b1ff">{{ webSocketStore.url }}</span>.
      <br>
      <br>
      Please select the item to be tested in the "Test Items" section on the left.
    </el-text>
    <div class="footer">
      <el-text style="font-size: var(--el-font-size-base)">
        <el-link href="https://github.com/Akiko97" target="_blank">@Akiko97</el-link>
      </el-text>
    </div>
  </div>
</template>

<style scoped>
.home-container {
  display: flex;
  flex-direction: column;
  height: 100%;
}
.footer {
  margin-top: auto;
  text-align: center;
}
</style>
