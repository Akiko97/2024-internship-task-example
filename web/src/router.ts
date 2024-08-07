import { createMemoryHistory, createRouter } from 'vue-router'

import HomeView from './views/HomeView.vue'
import ChartView from './views/ChartView.vue'
import ProgressView from './views/ProgressView.vue'

const routes = [
  { path: '/', component: HomeView },
  { path: '/chart', component: ChartView },
  { path: '/progress', component: ProgressView },
]

export const router = createRouter({
  history: createMemoryHistory(),
  routes,
})
