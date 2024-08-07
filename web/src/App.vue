<script setup lang="ts">
import { useDark } from '@vueuse/core'
import { Moon, Sunny } from '@element-plus/icons-vue'
import { computed } from 'vue'
import { useWebSocketStore } from './store/ws'

const isDark = useDark()

const webSocketStore = useWebSocketStore()
const webSocketUrl = computed({
  get: () => webSocketStore.url,
  set: (value: string) => webSocketStore.setUrl(value),
});
</script>

<template>
  <div class="app-container">
    <el-container style="height: 100%">
      <el-header class="app-header">
        <el-row :gutter="20" style="width: 100%">
          <el-col :span="16" style="text-align: left">
            <el-text style="font-size: var(--el-font-size-extra-large)">
              Test Website
            </el-text>
          </el-col>
          <el-col :span="8" style="text-align: right">
            <el-switch
                v-model="isDark"
                size="large"
                :active-action-icon="Moon"
                :inactive-action-icon="Sunny"
                style="--el-switch-on-color: #525457; --el-switch-off-color: #a6a9ad"
            />
          </el-col>
        </el-row>
      </el-header>
      <el-container>
        <el-aside class="app-aside">
          <el-card style="width: 95%">
            <template #header>
              <div class="card-header">
                <el-text style="font-size: var(--el-font-size-large)">
                  Settings
                </el-text>
              </div>
            </template>
            <el-row :gutter="20" style="width: 100%">
              <el-col :span="4">
                <el-text style="font-size: var(--el-font-size-base)">
                  URL:
                </el-text>
              </el-col>
              <el-col :span="20">
                <el-input
                    v-model="webSocketUrl"
                    style="width: 100%"
                    placeholder="Websocket URL"
                    clearable
                />
              </el-col>
            </el-row>
          </el-card>
          <br>
          <el-card style="width: 95%">
            <template #header>
              <div class="card-header">
                <el-text style="font-size: var(--el-font-size-large)">
                  Test Items
                </el-text>
              </div>
            </template>
            <p>
              <el-button text style="width: 100%" @click="$router.push('/')">
                <el-text style="font-size: var(--el-font-size-base)">
                  Homepage
                </el-text>
              </el-button>
            </p>
            <p>
              <el-button text style="width: 100%" @click="$router.push('/chart')">
                <el-text style="font-size: var(--el-font-size-base)">
                  Chart
                </el-text>
              </el-button>
            </p>
            <p>
              <el-button text style="width: 100%" @click="$router.push('/progress')">
                <el-text style="font-size: var(--el-font-size-base)">
                  Progress
                </el-text>
              </el-button>
            </p>
          </el-card>
        </el-aside>
        <el-main>
          <RouterView />
        </el-main>
      </el-container>
    </el-container>
  </div>
</template>

<style scoped>
.app-container {
  width: 100%;
  height: 100%;
}
.app-header {
  width: 100%;
  margin: 10px;
  display: flex;
  align-items: center;
  justify-content: center;
}
.app-aside {
  width: 30%;
  height: 98%;
  text-align: center;
}
</style>
