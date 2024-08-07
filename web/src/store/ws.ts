import { defineStore } from 'pinia'

export const useWebSocketStore = defineStore('webSocket', {
  state: () => ({
    url: 'ws://127.0.0.1:8080/ws',
  }),
  actions: {
    setUrl(newUrl: string) {
      this.url = newUrl;
    },
  },
});
