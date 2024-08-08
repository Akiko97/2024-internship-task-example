<script setup lang="ts">
import { useWebSocketStore } from '../store/ws'
import { onBeforeUnmount, onMounted, ref, computed, watch } from 'vue'
import { ElMessage } from 'element-plus'
import {
  encodePacket,
  INCREMENTAL_SEQUENCE_REQUEST,
  STOP_INCREMENTAL_SEQUENCE_REQUEST,
} from '../utils'
import {
  encodeIncrementalSequenceRequest,
  encodeStopIncrementalSequenceRequest,
  IncrementalSequenceRequest,
  IncrementalSequenceResponse,
  StopIncrementalSequenceRequest,
} from '../proto/msg_pb'
import {
  ws,
  initWebSocket,
  disconnectWebSocket,
  setID,
  unsetID,
  setUpdateIncrementalSequenceResponseData,
  setStopIncrementalSequenceResponseCallback,
} from '../websocket.ts'

import WsControl from '../components/WsControl.vue'

const webSocketStore = useWebSocketStore()
const start = ref(0)
const end = ref(100)
const interval = ref(2)
const id = ref('progress0')
watch(id, (newVal) => {
  setID(newVal)
})
const percentage = ref(0)
const isWarning = ref(false)
const status = computed(() => {
  if (percentage.value === 100) {
    return 'success'
  } else if (!(ws.value)) {
    return 'exception'
  } else if (isWarning.value) {
    return 'warning'
  } else {
    return ''
  }
})
const duration = computed(() => {
  if (status.value === 'success') {
    return 0
  } else if (status.value === '') {
    return 20
  } else if (status.value === 'warning') {
    return 10
  } else {
    return 5
  }
})

const format = (percentage: number) => (percentage > 0 && percentage < 100 ? 'Progressing' : '')

const customColor = (percentage: number) => {
  if (percentage < 25) {
    return '#213d5b'
  }
  if (percentage < 50) {
    return '#2a598a'
  }
  if (percentage < 75) {
    return '#3375b9'
  }
  return '#66b1ff'
}

const clear = () => {
  percentage.value = 0
  isWarning.value = false
}

const sendStartMsg = () => {
  if (ws.value && ws.value.readyState === WebSocket.OPEN) {
    let msg: IncrementalSequenceRequest = {
      id: id.value,
      start: start.value,
      end: end.value,
      interval: interval.value,
    }
    let msg_buf = encodeIncrementalSequenceRequest(msg)
    let packet = encodePacket(INCREMENTAL_SEQUENCE_REQUEST, msg_buf)
    ws.value.send(packet)
  } else {
    ElMessage.error(`WebSocket is not open`)
  }
}

const sendStopMsg = () => {
  if (ws.value && ws.value.readyState === WebSocket.OPEN) {
    let msg: StopIncrementalSequenceRequest = {
      id: id.value,
    }
    let msg_buf = encodeStopIncrementalSequenceRequest(msg)
    let packet = encodePacket(STOP_INCREMENTAL_SEQUENCE_REQUEST, msg_buf)
    ws.value.send(packet)
  } else {
    ElMessage.error(`WebSocket is not open`)
  }
}

const connectWebsocket = () => {
  initWebSocket(webSocketStore.url)
}

const updateData = (msg: IncrementalSequenceResponse) => {
  isWarning.value = false
  percentage.value = msg.number ? msg.number : 0 // undefined guard
}

const stopCallback = () => {
  setTimeout(() => {
    ElMessage({
      message: 'Task stopped',
      type: 'success',
    })
    if (percentage.value < end.value) {
      isWarning.value = true
    }
  }, interval.value * 2 * 1000)
}

onMounted(() => {
  setUpdateIncrementalSequenceResponseData(updateData)
  setStopIncrementalSequenceResponseCallback(stopCallback)
  setID(id.value)
})

onBeforeUnmount(() => {
  unsetID()
})
</script>

<template>
  <div class="progress-container">
    <el-container style="height: 100%">
      <el-header>
        <el-text style="font-size: var(--el-font-size-large)">
          Progress
        </el-text>
      </el-header>
      <el-main>
        <el-container style="height: 100%">
          <el-header style="margin: 10px">
            <el-row :gutter="20" style="width: 100%">
              <el-col :span="6">
                <el-input
                    v-model="start"
                    style="width: 100%"
                >
                  <template #prepend>Start:</template>
                </el-input>
              </el-col>
              <el-col :span="6">
                <el-input
                    v-model="end"
                    style="width: 100%"
                >
                  <template #prepend>End:</template>
                </el-input>
              </el-col>
              <el-col :span="6">
                <el-input
                    v-model="interval"
                    style="width: 100%"
                >
                  <template #prepend>Interval:</template>
                  <template #append>s</template>
                </el-input>
              </el-col>
              <el-col :span="6">
                <el-input
                    v-model="id"
                    style="width: 100%"
                >
                  <template #prepend>ID:</template>
                </el-input>
              </el-col>
            </el-row>
            <br>
            <WsControl
                :ifConnect="ws !== null"
                :connectWebsocket="connectWebsocket"
                :disconnectWebSocket="disconnectWebSocket"
                :sendStartMsg="sendStartMsg"
                :sendStopMsg="sendStopMsg"
                :clear="clear"
            />
          </el-header>
          <el-main>
            <div class="progress-main">
              <div class="progress">
                <el-progress
                    :percentage="percentage"
                    :format="format"
                    :status="status"
                    :color="customColor"
                />
              </div>
              <div class="progress">
                <el-progress
                    :text-inside="true"
                    :stroke-width="24"
                    :percentage="percentage"
                    :status="status"
                    :color="customColor"
                />
              </div>
              <div class="progress">
                <el-progress
                    :percentage="percentage"
                    :status="status"
                    :color="customColor"
                    :indeterminate="true"
                    :duration="5"
                />
              </div>
              <div class="progress">
                <el-progress
                    :text-inside="true"
                    :stroke-width="24"
                    :percentage="percentage"
                    :status="status"
                    :color="customColor"
                    striped
                    striped-flow
                    :duration="duration"
                />
              </div>
              <div class="progress">
                <el-progress
                    type="circle"
                    :percentage="percentage"
                    :status="status"
                    :color="customColor"
                />
              </div>
              <div class="progress">
                <el-progress
                    type="dashboard"
                    :percentage="percentage"
                    :status="status"
                    :color="customColor"
                />
              </div>
            </div>
          </el-main>
        </el-container>
      </el-main>
    </el-container>
  </div>
</template>

<style scoped>
.progress-container {
  width: 100%;
  height: 100%;
}
.progress-main {
  padding: 1.5rem;
}
.progress {
  padding: 1rem;
}
</style>
