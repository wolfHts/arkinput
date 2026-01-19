<template>
  <div>
    <h2 class="text-2xl font-bold text-gray-800 mb-6">设置</h2>

    <div class="space-y-6">
      <!-- Excluded Apps -->
      <div class="bg-white rounded-xl shadow p-6">
        <h3 class="text-lg font-semibold text-gray-800 mb-4">排除应用</h3>
        <p class="text-sm text-gray-500 mb-4">
          这些应用中的键盘输入将不会被记录
        </p>

        <div class="flex gap-2 mb-4">
          <input
            v-model="newExcludedApp"
            type="text"
            placeholder="输入应用名称"
            class="flex-1 px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500 outline-none"
            @keyup.enter="addExcludedApp"
          />
          <button
            @click="addExcludedApp"
            class="px-4 py-2 bg-primary-500 text-white rounded-lg hover:bg-primary-600 transition-colors"
          >
            添加
          </button>
        </div>

        <div class="flex flex-wrap gap-2">
          <span
            v-for="app in settings.excluded_apps"
            :key="app"
            class="inline-flex items-center px-3 py-1 bg-gray-100 rounded-full text-sm"
          >
            {{ app }}
            <button
              @click="removeExcludedApp(app)"
              class="ml-2 text-gray-400 hover:text-gray-600"
            >
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
              </svg>
            </button>
          </span>
          <span v-if="settings.excluded_apps.length === 0" class="text-gray-400 text-sm">
            暂无排除应用
          </span>
        </div>
      </div>

      <!-- Data Management -->
      <div class="bg-white rounded-xl shadow p-6">
        <h3 class="text-lg font-semibold text-gray-800 mb-4">数据管理</h3>

        <div class="space-y-4">
          <!-- Export -->
          <div class="flex items-center justify-between p-4 bg-gray-50 rounded-lg">
            <div>
              <p class="font-medium text-gray-800">导出数据</p>
              <p class="text-sm text-gray-500">将所有记录导出为 JSON 文件</p>
            </div>
            <button
              @click="exportData"
              class="px-4 py-2 bg-primary-500 text-white rounded-lg hover:bg-primary-600 transition-colors"
              :disabled="exporting"
            >
              {{ exporting ? '导出中...' : '导出' }}
            </button>
          </div>

          <!-- Delete Old Records -->
          <div class="flex items-center justify-between p-4 bg-gray-50 rounded-lg">
            <div>
              <p class="font-medium text-gray-800">清理旧数据</p>
              <p class="text-sm text-gray-500">删除指定日期之前的记录</p>
            </div>
            <div class="flex items-center gap-2">
              <input
                v-model="deleteBeforeDate"
                type="date"
                class="px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500 outline-none"
              />
              <button
                @click="deleteOldRecords"
                class="px-4 py-2 bg-red-500 text-white rounded-lg hover:bg-red-600 transition-colors"
                :disabled="!deleteBeforeDate"
              >
                删除
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- Save Button -->
      <div class="flex justify-end">
        <button
          @click="saveSettings"
          class="px-6 py-2 bg-primary-500 text-white rounded-lg hover:bg-primary-600 transition-colors"
          :disabled="saving"
        >
          {{ saving ? '保存中...' : '保存设置' }}
        </button>
      </div>

      <!-- Status Message -->
      <div
        v-if="statusMessage"
        class="p-4 rounded-lg"
        :class="statusType === 'success' ? 'bg-green-100 text-green-800' : 'bg-red-100 text-red-800'"
      >
        {{ statusMessage }}
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

interface Settings {
  excluded_apps: string[]
  merge_interval_ms: number
  auto_start: boolean
}

const settings = ref<Settings>({
  excluded_apps: [],
  merge_interval_ms: 500,
  auto_start: false,
})

const newExcludedApp = ref('')
const deleteBeforeDate = ref('')
const saving = ref(false)
const exporting = ref(false)
const statusMessage = ref('')
const statusType = ref<'success' | 'error'>('success')

function showStatus(message: string, type: 'success' | 'error') {
  statusMessage.value = message
  statusType.value = type
  setTimeout(() => {
    statusMessage.value = ''
  }, 3000)
}

function addExcludedApp() {
  const app = newExcludedApp.value.trim()
  if (app && !settings.value.excluded_apps.includes(app)) {
    settings.value.excluded_apps.push(app)
    newExcludedApp.value = ''
  }
}

function removeExcludedApp(app: string) {
  const index = settings.value.excluded_apps.indexOf(app)
  if (index > -1) {
    settings.value.excluded_apps.splice(index, 1)
  }
}

async function loadSettings() {
  try {
    settings.value = await invoke<Settings>('get_settings')
  } catch (e) {
    console.error('Failed to load settings:', e)
  }
}

async function saveSettings() {
  saving.value = true
  try {
    await invoke('save_settings', { settings: settings.value })
    showStatus('设置已保存', 'success')
  } catch (e) {
    console.error('Failed to save settings:', e)
    showStatus('保存失败: ' + e, 'error')
  } finally {
    saving.value = false
  }
}

async function exportData() {
  exporting.value = true
  try {
    const json = await invoke<string>('export_records', {
      filter: { limit: null, offset: null },
    })

    // Create a download link
    const blob = new Blob([json], { type: 'application/json' })
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = `arkinput-export-${new Date().toISOString().split('T')[0]}.json`
    document.body.appendChild(a)
    a.click()
    document.body.removeChild(a)
    URL.revokeObjectURL(url)

    showStatus('数据已导出', 'success')
  } catch (e) {
    console.error('Failed to export data:', e)
    showStatus('导出失败: ' + e, 'error')
  } finally {
    exporting.value = false
  }
}

async function deleteOldRecords() {
  if (!deleteBeforeDate.value) return

  if (!confirm(`确定要删除 ${deleteBeforeDate.value} 之前的所有记录吗？此操作不可撤销。`)) {
    return
  }

  try {
    const count = await invoke<number>('delete_old_records', {
      beforeDate: deleteBeforeDate.value,
    })
    showStatus(`已删除 ${count} 条记录`, 'success')
    deleteBeforeDate.value = ''
  } catch (e) {
    console.error('Failed to delete records:', e)
    showStatus('删除失败: ' + e, 'error')
  }
}

onMounted(() => {
  loadSettings()
})
</script>
