<script setup lang="ts">
import {useWebSocketStore} from '../store/ws'
import {onBeforeUnmount, onMounted, Ref, ref} from 'vue'
import {CategoryScale, Chart as ChartJS, Legend, LinearScale, LineElement, PointElement, Title, Tooltip} from 'chart.js'
import {Line} from 'vue-chartjs'
import {ElMessage} from 'element-plus'
import {
  decodePacket,
  encodePacket,
  RANDOM_NUMBER_REQUEST,
  RANDOM_NUMBER_RESPONSE,
  STOP_RANDOM_NUMBER_REQUEST,
  STOP_RANDOM_NUMBER_RESPONSE,
} from '../utils'
import {
  decodeRandomNumberResponse,
  encodeRandomNumberRequest,
  encodeStopRandomNumberRequest,
  RandomNumberRequest,
  RandomNumberResponse,
  StatusCode,
  StopRandomNumberRequest,
} from '../proto/msg_pb'

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

const webSocketStore = useWebSocketStore()
const min = ref(0)
const max = ref(100)
const interval = ref(5)
const id = ref('chart0')

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

const ws: Ref<WebSocket | null> = ref(null)
const initWebSocket = () => {
  ws.value = new WebSocket(webSocketStore.url)

  ws.value.onopen = () => {
    ElMessage({
      message: 'WebSocket connection opened',
      type: 'success',
    })
  }

  ws.value.onmessage = (event: MessageEvent) => {
    if (event.data instanceof Blob) {
      const reader = new FileReader()
      reader.onload = () => {
        const packet = reader.result as ArrayBuffer
        try {
          let [cmdID, msg_buf] = decodePacket(packet)
          switch (cmdID) {
            case RANDOM_NUMBER_RESPONSE: {
              let msg: RandomNumberResponse = decodeRandomNumberResponse(msg_buf)
              if (msg.id && msg.id === id.value) {
                if (msg.status && msg.status.code !== StatusCode.SUCCESS) {
                  ElMessage.error(`Receive error msg: ${msg.status.message}`)
                } else {
                  if (msg.number) {
                    data.value = {
                      labels: [...data.value.labels, `Data${data_no.value++}`],
                      datasets: [
                        {
                          ...data.value.datasets[0],
                          data: [...data.value.datasets[0].data, msg.number]
                        }
                      ]
                    }
                  } else {
                    ElMessage({
                      message: 'Received packet without number',
                      type: 'warning',
                    })
                  }
                }
              } else {
                ElMessage({
                  message: 'Received id not equal to id set by website',
                  type: 'warning',
                })
              }
              break
            }
            case STOP_RANDOM_NUMBER_RESPONSE: {
              //
              break
            }
            default: {
              ElMessage({
                message: 'Received packet include invalid cmd id',
                type: 'warning',
              })
            }
          }
        } catch (error) {
          ElMessage.error(`Invalid packet: ${error}`)
        }
      }
      reader.readAsArrayBuffer(event.data)
    } else {
      ElMessage({
        message: 'Received packet is not a Blob',
        type: 'warning',
      })
    }
  }

  ws.value.onclose = () => {
    ElMessage({
      message: 'WebSocket connection closed',
      type: 'warning',
    })
    ws.value = null
  }

  ws.value.onerror = (error: Event) => {
    ElMessage.error(`WebSocket error: ${error}`)
  }
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

onMounted(() => {
  initWebSocket()
})

const disconnectWebSocket = () => {
  if (ws.value) {
    ws.value.close()
    ws.value = null
  }
}

onBeforeUnmount(() => {
  disconnectWebSocket()
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
            <el-row :gutter="20" style="width: 100%">
              <el-col :span="6">
                <el-button
                    v-if="!(ws)"
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
