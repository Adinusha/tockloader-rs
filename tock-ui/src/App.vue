<script setup>
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

const devices = ref(null)
const errorMsg = ref('')
const loading = ref(false)

async function fetchDevices() {
  errorMsg.value = ''
  loading.value = true
  try {
    // Call the matching Rust command name
    devices.value = await invoke('list_all_devices')
  } catch (err) {
    errorMsg.value = String(err)
    console.error('Failed to list devices:', err)
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <div class="flex flex-col items-center justify-start min-h-screen p-8 bg-slate-900 text-white gap-6">
    <button 
      @click="fetchDevices" 
      :disabled="loading"
      class="px-5 py-2.5 bg-emerald-500 hover:bg-emerald-600 active:scale-95 disabled:opacity-50 font-medium rounded-lg transition"
    >
      {{ loading ? 'Scanning...' : 'List Connected Devices' }}
    </button>

    <!-- Error Banner -->
    <div v-if="errorMsg" class="p-4 bg-rose-950/70 border border-rose-700 text-rose-300 rounded-lg max-w-xl text-center">
      {{ errorMsg }}
    </div>

    <!-- Results Display -->
    <div v-if="devices" class="w-full max-w-2xl flex flex-col gap-6">
      <!-- Debug Probes -->
      <div class="bg-slate-800 p-5 rounded-xl border border-slate-700">
        <h2 class="text-xl font-bold text-emerald-400 mb-3">Debug Probes ({{ devices.debug_probes.length }})</h2>
        <ul v-if="devices.debug_probes.length" class="space-y-2">
          <li v-for="probe in devices.debug_probes" :key="probe.identifier" class="p-3 bg-slate-900 rounded-lg text-sm font-mono">
            <span class="text-slate-300">{{ probe.identifier }}</span>
            <span class="text-xs text-slate-500 block">VID: {{ probe.vendor_id }} | PID: {{ probe.product_id }}</span>
          </li>
        </ul>
        <p v-else class="text-slate-400 text-sm">No debug probes detected.</p>
      </div>

      <!-- Serial Ports -->
      <div class="bg-slate-800 p-5 rounded-xl border border-slate-700">
        <h2 class="text-xl font-bold text-sky-400 mb-3">Serial Ports ({{ devices.serial_ports.length }})</h2>
        <ul v-if="devices.serial_ports.length" class="space-y-2">
          <li v-for="port in devices.serial_ports" :key="port.port_name" class="p-3 bg-slate-900 rounded-lg text-sm font-mono">
            <span class="text-slate-300">{{ port.port_name }}</span>
            <span class="text-xs text-slate-500 block">Product: {{ port.product || 'N/A' }} | Manufacturer: {{ port.manufacturer || 'N/A' }}</span>
          </li>
        </ul>
        <p v-else class="text-slate-400 text-sm">No serial ports detected.</p>
      </div>
    </div>
  </div>
</template>