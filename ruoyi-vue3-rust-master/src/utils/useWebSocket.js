// useWebSocket.js
import { ref, onBeforeUnmount } from 'vue'

export default function useWebSocket(url) {

  const messages = ref([])
  const isConnected = ref(false)
  let ws = null

  const connect = (options) => {
    // console.log('connect')

    ws = new WebSocket(url)
    // console.log('WebSocket ',ws)
    ws.onopen = () => {
      isConnected.value = true
      console.log('WebSocket connected')
    }

    ws.onmessage = (event) => {
      let data = JSON.parse(event.data)
      options.onMessage(data)
      messages.value.push(data)
    }

    ws.onerror = (error) => {
      console.error('WebSocket error:', error)
      options.onError(error)
      isConnected.value = false
    }

    ws.onclose = () => {
      isConnected.value = false
      options.onClose('WebSocket closed')
      console.log('WebSocket closed')
    }
  }

  const send = (data) => {
    if (ws?.readyState === WebSocket.OPEN) {
      ws.send(JSON.stringify(data))
    }
  }

  const close = () => {
    ws?.close()
  }

  // 自动重连逻辑
  const reconnect = () => {
    if (!isConnected.value) {
      setTimeout(() => {
        console.log('Reconnecting...')
        connect()
      }, 3000)
    }
  }

  onBeforeUnmount(close)

  return {
    messages,
    isConnected,
    connect,
    send,
    close,
    reconnect
  }
}
