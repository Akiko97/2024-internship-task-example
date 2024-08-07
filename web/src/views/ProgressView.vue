<script setup lang="ts">
import { useWebSocketStore } from '../store/ws'
const webSocketStore = useWebSocketStore()
</script>

<template>
  {{ webSocketStore.url }}
</template>

<style scoped>

</style>
