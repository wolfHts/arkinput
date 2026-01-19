<template>
  <div>
    <h2 class="text-2xl font-bold text-gray-800 mb-6">今日统计</h2>

    <!-- Stats Cards -->
    <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
      <div class="bg-white rounded-xl shadow p-6">
        <div class="flex items-center">
          <div class="p-3 bg-primary-100 rounded-lg">
            <svg class="w-6 h-6 text-primary-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15.232 5.232l3.536 3.536m-2.036-5.036a2.5 2.5 0 113.536 3.536L6.5 21.036H3v-3.572L16.732 3.732z" />
            </svg>
          </div>
          <div class="ml-4">
            <p class="text-sm text-gray-500">总按键数</p>
            <p class="text-2xl font-bold text-gray-800">{{ stats?.total_keys ?? 0 }}</p>
          </div>
        </div>
      </div>

      <div class="bg-white rounded-xl shadow p-6">
        <div class="flex items-center">
          <div class="p-3 bg-green-100 rounded-lg">
            <svg class="w-6 h-6 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
            </svg>
          </div>
          <div class="ml-4">
            <p class="text-sm text-gray-500">记录数</p>
            <p class="text-2xl font-bold text-gray-800">{{ stats?.total_records ?? 0 }}</p>
          </div>
        </div>
      </div>

      <div class="bg-white rounded-xl shadow p-6">
        <div class="flex items-center">
          <div class="p-3 bg-purple-100 rounded-lg">
            <svg class="w-6 h-6 text-purple-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
            </svg>
          </div>
          <div class="ml-4">
            <p class="text-sm text-gray-500">使用应用数</p>
            <p class="text-2xl font-bold text-gray-800">{{ stats?.app_stats?.length ?? 0 }}</p>
          </div>
        </div>
      </div>
    </div>

    <!-- App Distribution Chart -->
    <div class="bg-white rounded-xl shadow p-6 mb-8">
      <h3 class="text-lg font-semibold text-gray-800 mb-4">应用分布</h3>
      <div v-if="stats?.app_stats?.length" class="h-64">
        <Bar :data="chartData" :options="chartOptions" />
      </div>
      <div v-else class="h-64 flex items-center justify-center text-gray-400">
        暂无数据
      </div>
    </div>

    <!-- Recent Apps -->
    <div class="bg-white rounded-xl shadow p-6">
      <h3 class="text-lg font-semibold text-gray-800 mb-4">应用详情</h3>
      <div v-if="stats?.app_stats?.length" class="space-y-3">
        <div
          v-for="app in stats.app_stats"
          :key="app.app_name"
          class="flex items-center justify-between p-3 bg-gray-50 rounded-lg"
        >
          <div class="flex items-center">
            <div class="w-10 h-10 bg-primary-100 rounded-lg flex items-center justify-center text-primary-600 font-bold">
              {{ app.app_name.charAt(0).toUpperCase() }}
            </div>
            <div class="ml-3">
              <p class="font-medium text-gray-800">{{ app.app_name }}</p>
              <p class="text-sm text-gray-500">{{ app.record_count }} 条记录</p>
            </div>
          </div>
          <div class="text-right">
            <p class="font-bold text-gray-800">{{ app.key_count }}</p>
            <p class="text-sm text-gray-500">按键</p>
          </div>
        </div>
      </div>
      <div v-else class="text-center text-gray-400 py-8">
        暂无应用数据
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { Bar } from 'vue-chartjs'
import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  BarElement,
  Title,
  Tooltip,
  Legend,
} from 'chart.js'

ChartJS.register(CategoryScale, LinearScale, BarElement, Title, Tooltip, Legend)

interface AppStats {
  app_name: string
  key_count: number
  record_count: number
}

interface DailyStats {
  date: string
  total_keys: number
  total_records: number
  app_stats: AppStats[]
}

const stats = ref<DailyStats | null>(null)

const chartData = computed(() => ({
  labels: stats.value?.app_stats?.map((a) => a.app_name) ?? [],
  datasets: [
    {
      label: '按键数',
      backgroundColor: '#0ea5e9',
      data: stats.value?.app_stats?.map((a) => a.key_count) ?? [],
    },
  ],
}))

const chartOptions = {
  responsive: true,
  maintainAspectRatio: false,
  plugins: {
    legend: {
      display: false,
    },
  },
  scales: {
    y: {
      beginAtZero: true,
    },
  },
}

async function loadStats() {
  try {
    stats.value = await invoke<DailyStats>('get_today_stats')
  } catch (e) {
    console.error('Failed to load stats:', e)
  }
}

onMounted(() => {
  loadStats()
  // Refresh stats every 5 seconds
  setInterval(loadStats, 5000)
})
</script>
