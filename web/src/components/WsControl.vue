<script setup lang="ts">
defineProps<{
  ifConnect: boolean,
  initWebSocket: () => void,
  disconnectWebSocket: () => void,
  sendStartMsg: () => void,
  sendStopMsg: () => void,
  clear: () => void,
}>()
</script>

<template>
  <el-row :gutter="20" style="width: 100%">
    <el-col :span="6">
      <el-button
          v-if="!ifConnect"
          text
          bg
          style="width: 100%"
          @click="initWebSocket"
      >
        Connect
      </el-button>
      <el-button
          v-else
          text
          bg
          style="width: 100%"
          @click="disconnectWebSocket"
      >
        Disconnect
      </el-button>
    </el-col>
    <el-col :span="6">
      <el-button
          type="primary"
          style="width: 100%"
          @click="sendStartMsg"
      >
        Start
      </el-button>
    </el-col>
    <el-col :span="6">
      <el-button
          type="warning"
          style="width: 100%"
          @click="sendStopMsg"
      >
        Stop
      </el-button>
    </el-col>
    <el-col :span="6">
      <el-button
          type="danger"
          style="width: 100%"
          @click="clear"
      >
        Clear
      </el-button>
    </el-col>
  </el-row>
</template>

<style scoped>
</style>
