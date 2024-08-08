<script setup lang="ts">
import { onBeforeUnmount, onMounted, ref, watch } from 'vue'
import { ElMessage } from 'element-plus'
import { useWebSocketStore } from '../store/ws'
import {
  CategoryScale,
  Chart as ChartJS,
  Legend,
  LinearScale,
  LineElement,
  PointElement,
  Title,
  Tooltip
} from 'chart.js'
import { Line } from 'vue-chartjs'
import {
  encodePacket,
  RANDOM_NUMBER_REQUEST,
  STOP_RANDOM_NUMBER_REQUEST,
} from '../utils'
import {
  encodeRandomNumberRequest,
  encodeStopRandomNumberRequest,
  RandomNumberRequest,
  RandomNumberResponse,
  StopRandomNumberRequest,
} from '../proto/msg_pb'
import {
  ws,
  initWebSocket,
  disconnectWebSocket,
  setUpdateRandomNumberResponseData,
  setStopRandomNumberResponseCallback,
  setID,
  unsetID,
} from '../websocket.ts'
import WsControl from '../components/WsControl.vue'

const webSocketStore = useWebSocketStore()

ChartJS.register(
    CategoryScale,
    LinearScale,
    PointElement,
    LineElement,
    Title,
    Tooltip,
    Legend
)

const data = ref<{
  labels: string[],
  datasets: {
    label: string,
    backgroundColor: string,
    data: number[],
  }[]
}>({
  labels: [],
  datasets: [
    {
      label: 'Data Received',
      backgroundColor: '#3375b9',
      data: []
    }
  ]
})

const options = {
  responsive: true,
  maintainAspectRatio: false
}

const data_no = ref(0)

const min = ref(0)
const max = ref(100)
const interval = ref(2)
const id = ref('chart0')

watch(id, (newVal) => {
  setID(newVal)
})

const clear = () => {
  data.value = {
    labels: [],
    datasets: [
      {
        label: 'Data Received',
        backgroundColor: '#3375b9',
        data: []
      }
    ]
  }
  data_no.value = 0
}

const sendStartMsg = () => {
  if (ws.value && ws.value.readyState === WebSocket.OPEN) {
    let msg: RandomNumberRequest = {
      id: id.value,
      min: min.value,
      max: max.value,
      interval: interval.value,
    }
    let msg_buf = encodeRandomNumberRequest(msg)
    let packet = encodePacket(RANDOM_NUMBER_REQUEST, msg_buf)
    ws.value.send(packet)
  } else {
    ElMessage.error(`WebSocket is not open`)
  }
}

const sendStopMsg = () => {
  if (ws.value && ws.value.readyState === WebSocket.OPEN) {
    let msg: StopRandomNumberRequest = {
      id: id.value,
    }
    let msg_buf = encodeStopRandomNumberRequest(msg)
    let packet = encodePacket(STOP_RANDOM_NUMBER_REQUEST, msg_buf)
    ws.value.send(packet)
  } else {
    ElMessage.error(`WebSocket is not open`)
  }
}

const connectWebsocket = () => {
  initWebSocket(webSocketStore.url)
}

const updateData = (msg: RandomNumberResponse) => {
  data.value = {
    labels: [...data.value.labels, `Data${data_no.value++}`],
    datasets: [
      {
        ...data.value.datasets[0],
        data: [...data.value.datasets[0].data, msg.number]
      }
    ]
  }
}

const stopCallback = () => {
  setTimeout(() => {
    ElMessage({
      message: 'Task stopped',
      type: 'success',
    })
  }, interval.value * 2 * 1000)
}

onMounted(() => {
  setUpdateRandomNumberResponseData(updateData)
  setStopRandomNumberResponseCallback(stopCallback)
  setID(id.value)
})

onBeforeUnmount(() => {
  unsetID()
})
</script>

<template>
  <div class="chart-container">
    <el-container style="height: 100%">
      <el-header>
        <el-text style="font-size: var(--el-font-size-large)">
          Chart
        </el-text>
      </el-header>
      <el-main>
        <el-container style="height: 100%">
          <el-header style="margin: 10px">
            <el-row :gutter="20" style="width: 100%">
              <el-col :span="6">
                <el-input
                    v-model="min"
                    style="width: 100%"
                >
                  <template #prepend>Min Value:</template>
                </el-input>
              </el-col>
              <el-col :span="6">
                <el-input
                    v-model="max"
                    style="width: 100%"
                >
                  <template #prepend>Max Value:</template>
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
            <Line :data="data" :options="options" />
          </el-main>
        </el-container>
      </el-main>
    </el-container>
  </div>
</template>

<style scoped>
.chart-container {
  height: 100%;
  width: 100%;
}
</style>
