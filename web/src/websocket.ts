import { ref, Ref } from 'vue'
import { ElMessage } from 'element-plus'
import {
  decodePacket, INCREMENTAL_SEQUENCE_RESPONSE,
  RANDOM_NUMBER_RESPONSE, STOP_INCREMENTAL_SEQUENCE_RESPONSE,
  STOP_RANDOM_NUMBER_RESPONSE,
} from './utils'
import {
  decodeIncrementalSequenceResponse,
  decodeRandomNumberResponse, decodeStopIncrementalSequenceResponse,
  decodeStopRandomNumberResponse, IncrementalSequenceResponse,
  RandomNumberResponse,
  StatusCode, StopIncrementalSequenceResponse,
  StopRandomNumberResponse,
} from './proto/msg_pb'

export const ws: Ref<WebSocket | null> = ref(null)

export const initWebSocket = (url: string) => {
  ws.value = new WebSocket(url)

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
                if (msg.status && msg.status.code && msg.status.code !== StatusCode.SUCCESS) {
                  ElMessage.error(`Receive error msg: ${msg.status.message}`)
                } else {
                  if (msg.number) {
                    if (updateRandomNumberResponseData) {
                      updateRandomNumberResponseData(msg)
                    } else {
                      ElMessage({
                        message: 'Callback function not set',
                        type: 'warning',
                      })
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
              let msg: StopRandomNumberResponse = decodeStopRandomNumberResponse(msg_buf)
              if (msg.id && msg.id === id.value) {
                if (msg.status && msg.status.code && msg.status.code !== StatusCode.SUCCESS) {
                  ElMessage.error(`Receive error msg: ${msg.status.message}`)
                } else {
                  if (stopRandomNumberResponseCallback) {
                    stopRandomNumberResponseCallback()
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
            case INCREMENTAL_SEQUENCE_RESPONSE: {
              let msg: IncrementalSequenceResponse = decodeIncrementalSequenceResponse(msg_buf)
              if (msg.id && msg.id === id.value) {
                if (msg.status && msg.status.code && msg.status.code !== StatusCode.SUCCESS) {
                  ElMessage.error(`Receive error msg: ${msg.status.message}`)
                } else {
                  if (msg.number) {
                    if (updateIncrementalSequenceResponseData) {
                      updateIncrementalSequenceResponseData(msg)
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
            case STOP_INCREMENTAL_SEQUENCE_RESPONSE: {
              let msg: StopIncrementalSequenceResponse = decodeStopIncrementalSequenceResponse(msg_buf)
              if (msg.id && msg.id === id.value) {
                if (msg.status && msg.status.code && msg.status.code !== StatusCode.SUCCESS) {
                  ElMessage.error(`Receive error msg: ${msg.status.message}`)
                } else {
                  if (stopIncrementalSequenceResponseCallback) {
                    stopIncrementalSequenceResponseCallback()
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

export const disconnectWebSocket = () => {
  if (ws.value) {
    ws.value.close()
    ws.value = null
  }
}

const id: Ref<null | string> = ref(null)
export const setID = (new_id: string) => {
  id.value = new_id
}
export const unsetID = () => {
  id.value = null
}

let updateRandomNumberResponseData: null | ((RandomNumberResponse) => void) = null
export const setUpdateRandomNumberResponseData = (callback: (RandomNumberResponse) => void) => {
  updateRandomNumberResponseData = callback
}

let stopRandomNumberResponseCallback: null | (() => void) = null
export const setStopRandomNumberResponseCallback = (callback: () => void) => {
  stopRandomNumberResponseCallback = callback
}

let updateIncrementalSequenceResponseData: null | ((IncrementalSequenceResponse) => void) = null
export const setUpdateIncrementalSequenceResponseData = (callback: (IncrementalSequenceResponse) => void) => {
  updateIncrementalSequenceResponseData = callback
}

let stopIncrementalSequenceResponseCallback: null | (() => void) = null
export const setStopIncrementalSequenceResponseCallback = (callback: () => void) => {
  stopIncrementalSequenceResponseCallback = callback
}
