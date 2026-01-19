<template>
  <aside class="w-64 bg-white shadow-lg flex flex-col">
    <div class="p-6 border-b">
      <h1 class="text-xl font-bold text-primary-600">Arkinput</h1>
      <p class="text-sm text-gray-500 mt-1">输入记录工具</p>
    </div>

    <nav class="flex-1 p-4">
      <ul class="space-y-2">
        <li>
          <router-link
            to="/dashboard"
            class="flex items-center px-4 py-3 rounded-lg transition-colors"
            :class="[
              $route.path === '/dashboard'
                ? 'bg-primary-50 text-primary-600'
                : 'text-gray-600 hover:bg-gray-100'
            ]"
          >
            <svg class="w-5 h-5 mr-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2V6zM14 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V6zM4 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2v-2zM14 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2z" />
            </svg>
            仪表板
          </router-link>
        </li>
        <li>
          <router-link
            to="/records"
            class="flex items-center px-4 py-3 rounded-lg transition-colors"
            :class="[
              $route.path === '/records'
                ? 'bg-primary-50 text-primary-600'
                : 'text-gray-600 hover:bg-gray-100'
            ]"
          >
            <svg class="w-5 h-5 mr-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
            </svg>
            记录列表
          </router-link>
        </li>
        <li>
          <router-link
            to="/settings"
            class="flex items-center px-4 py-3 rounded-lg transition-colors"
            :class="[
              $route.path === '/settings'
                ? 'bg-primary-50 text-primary-600'
                : 'text-gray-600 hover:bg-gray-100'
            ]"
          >
            <svg class="w-5 h-5 mr-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
            </svg>
            设置
          </router-link>
        </li>
      </ul>
    </nav>

    <div class="p-4 border-t text-center text-xs text-gray-400">
      v0.1.0
    </div>
  </aside>
</template>
