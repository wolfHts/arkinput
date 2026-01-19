<template>
  <div>
    <h2 class="text-2xl font-bold text-gray-800 mb-6">记录列表</h2>

    <!-- Search and Filter -->
    <div class="bg-white rounded-xl shadow p-4 mb-6">
      <div class="flex flex-wrap gap-4">
        <div class="flex-1 min-w-48">
          <input
            v-model="searchQuery"
            type="text"
            placeholder="搜索内容..."
            class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500 outline-none"
            @input="debouncedSearch"
          />
        </div>
        <div class="w-48">
          <select
            v-model="selectedApp"
            class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500 outline-none"
            @change="loadRecords"
          >
            <option value="">全部应用</option>
            <option v-for="app in appList" :key="app" :value="app">
              {{ app }}
            </option>
          </select>
        </div>
        <div class="w-40">
          <input
            v-model="startDate"
            type="date"
            class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500 outline-none"
            @change="loadRecords"
          />
        </div>
        <div class="w-40">
          <input
            v-model="endDate"
            type="date"
            class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary-500 focus:border-primary-500 outline-none"
            @change="loadRecords"
          />
        </div>
      </div>
    </div>

    <!-- Records List -->
    <div class="bg-white rounded-xl shadow">
      <div v-if="loading" class="p-8 text-center text-gray-400">
        加载中...
      </div>
      <div v-else-if="records.length === 0" class="p-8 text-center text-gray-400">
        暂无记录
      </div>
      <div v-else class="divide-y">
        <RecordItem
          v-for="record in records"
          :key="record.id"
          :record="record"
        />
      </div>

      <!-- Load More -->
      <div v-if="records.length >= pageSize" class="p-4 text-center border-t">
        <button
          @click="loadMore"
          class="px-6 py-2 bg-gray-100 text-gray-600 rounded-lg hover:bg-gray-200 transition-colors"
          :disabled="loading"
        >
          加载更多
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import RecordItem from '../components/RecordItem.vue'

interface InputRecord {
  id: number
  timestamp: string
  app_name: string
  window_title: string | null
  content: string
  key_count: number
}

const records = ref<InputRecord[]>([])
const appList = ref<string[]>([])
const loading = ref(false)
const searchQuery = ref('')
const selectedApp = ref('')
const startDate = ref('')
const endDate = ref('')
const pageSize = 50
const currentOffset = ref(0)

let searchTimeout: number | null = null

function debouncedSearch() {
  if (searchTimeout) {
    clearTimeout(searchTimeout)
  }
  searchTimeout = window.setTimeout(() => {
    loadRecords()
  }, 300)
}

async function loadRecords() {
  loading.value = true
  currentOffset.value = 0

  try {
    const filter = {
      query: searchQuery.value || null,
      app_name: selectedApp.value || null,
      start_date: startDate.value || null,
      end_date: endDate.value ? endDate.value + ' 23:59:59' : null,
      limit: pageSize,
      offset: 0,
    }

    records.value = await invoke<InputRecord[]>('get_records', { filter })
  } catch (e) {
    console.error('Failed to load records:', e)
  } finally {
    loading.value = false
  }
}

async function loadMore() {
  loading.value = true
  currentOffset.value += pageSize

  try {
    const filter = {
      query: searchQuery.value || null,
      app_name: selectedApp.value || null,
      start_date: startDate.value || null,
      end_date: endDate.value ? endDate.value + ' 23:59:59' : null,
      limit: pageSize,
      offset: currentOffset.value,
    }

    const moreRecords = await invoke<InputRecord[]>('get_records', { filter })
    records.value.push(...moreRecords)
  } catch (e) {
    console.error('Failed to load more records:', e)
  } finally {
    loading.value = false
  }
}

async function loadAppList() {
  try {
    appList.value = await invoke<string[]>('get_app_list')
  } catch (e) {
    console.error('Failed to load app list:', e)
  }
}

onMounted(() => {
  loadRecords()
  loadAppList()
})
</script>
