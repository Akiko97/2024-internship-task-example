import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'

import serverConfig from './server.json'

let base = serverConfig.http.base_path
if (!base.endsWith('/')) {
  base = base + '/'
}

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [vue()],
  base: base,
})
