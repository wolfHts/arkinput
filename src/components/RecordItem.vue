<template>
  <div class="p-4 hover:bg-gray-50 transition-colors">
    <div class="flex items-start justify-between">
      <div class="flex items-start space-x-3">
        <div class="w-10 h-10 bg-primary-100 rounded-lg flex items-center justify-center text-primary-600 font-bold flex-shrink-0">
          {{ record.app_name.charAt(0).toUpperCase() }}
        </div>
        <div class="flex-1 min-w-0">
          <div class="flex items-center gap-2 mb-1">
            <span class="font-medium text-gray-800">{{ record.app_name }}</span>
            <span v-if="record.window_title" class="text-sm text-gray-500 truncate max-w-xs">
              - {{ record.window_title }}
            </span>
          </div>
          <p class="text-gray-600 break-all font-mono text-sm bg-gray-100 p-2 rounded">
            {{ formatContent(record.content) }}
          </p>
        </div>
      </div>
      <div class="text-right flex-shrink-0 ml-4">
        <p class="text-sm text-gray-500">{{ formatTime(record.timestamp) }}</p>
        <p class="text-xs text-gray-400">{{ record.key_count }} 按键</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
interface InputRecord {
  id: number
  timestamp: string
  app_name: string
  window_title: string | null
  content: string
  key_count: number
}

defineProps<{
  record: InputRecord
}>()

function formatTime(timestamp: string): string {
  const date = new Date(timestamp)
  const now = new Date()
  const isToday = date.toDateString() === now.toDateString()

  if (isToday) {
    return date.toLocaleTimeString('zh-CN', {
      hour: '2-digit',
      minute: '2-digit',
      second: '2-digit',
    })
  }

  return date.toLocaleString('zh-CN', {
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit',
  })
}

function formatContent(content: string): string {
  // Highlight special keys
  return content
    .replace(/\[Enter\]/g, '\u23CE')
    .replace(/\[Tab\]/g, '\u21B9')
    .replace(/\[Backspace\]/g, '\u232B')
    .replace(/\[Delete\]/g, '\u2326')
    .replace(/\[Esc\]/g, '\u238B')
    .replace(/\[Up\]/g, '\u2191')
    .replace(/\[Down\]/g, '\u2193')
    .replace(/\[Left\]/g, '\u2190')
    .replace(/\[Right\]/g, '\u2192')
}
</script>
