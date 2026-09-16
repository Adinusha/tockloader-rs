<script setup>
import { ref, onMounted, onUnmounted } from 'vue';
import { listen } from '@tauri-apps/api/event';
import { useRouter, useRoute } from 'vue-router';

const router = useRouter();
const route = useRoute();

const tabs = [
  { label: 'CONNECTED DEVICES', path: '/connected-devices' },
  { label: 'APP MANAGEMENT', path: '/app-management' },
  { label: 'TERMINAL', path: '/terminal' },
];

const connectedBoards = ref([]);
const activeBoard = ref(null);
const isBoardMenuOpen = ref(false);

let unlistenConnected = null;
let unlistenDisconnected = null;

onMounted(async () => {
  unlistenConnected = await listen('board-connected', (event) => {
    const board = event.payload;
    const exists = connectedBoards.value.some(b => b.port_name === board.port_name);
    if (!exists) {
      connectedBoards.value.push(board);
      if (!activeBoard.value) activeBoard.value = board;
    }
  });

  unlistenDisconnected = await listen('board-disconnected', (event) => {
    const portName = event.payload;
    connectedBoards.value = connectedBoards.value.filter(b => b.port_name !== portName);
    if (activeBoard.value?.port_name === portName) {
      activeBoard.value = connectedBoards.value[0] || null;
    }
  });
});

onUnmounted(() => {
  unlistenConnected?.();
  unlistenDisconnected?.();
});

const selectBoard = (board) => {
  activeBoard.value = board;
  isBoardMenuOpen.value = false;
};

const toggleBoardMenu = () => {
  isBoardMenuOpen.value = !isBoardMenuOpen.value;
};

const closeBoardMenu = () => {
  isBoardMenuOpen.value = false;
};
</script>

<template>
  <nav class="grid items-center w-full grid-cols-[1fr_auto_1fr] gap-4 px-6 py-3 shadow-md bg-[#3b3b3b] font-sans">

    <div></div>

    <div class="flex justify-center space-x-2 text-[11px] sm:text-xs font-bold tracking-wider">
      <button
        v-for="tab in tabs"
        :key="tab.path"
        type="button"
        @click="router.push(tab.path)"
        :class="[
          'px-5 py-2 transition-all duration-200 ease-in-out rounded-full whitespace-nowrap',
          route.path === tab.path
            ? 'bg-[#2b2b2b] text-gray-100 shadow-inner'
            : 'bg-transparent text-gray-400 hover:text-gray-200 hover:bg-[#454545]'
        ]"
      >
        {{ tab.label }}
      </button>
    </div>

    <div class="relative justify-self-end">
      <button
        type="button"
        @click="toggleBoardMenu"
        class="flex items-center justify-between w-48 gap-2 px-5 py-2 text-[11px] sm:text-xs font-bold tracking-wider text-gray-900 transition-colors duration-200 bg-gray-100 rounded-full hover:bg-white"
      >
        <span class="truncate">{{ activeBoard?.product || activeBoard?.port_name || 'No board' }}</span>
        <svg
          xmlns="http://www.w3.org/2000/svg"
          class="w-3 h-3 shrink-0 transition-transform duration-200"
          :class="{ 'rotate-180': isBoardMenuOpen }"
          fill="none"
          viewBox="0 0 24 24"
          stroke="currentColor"
          stroke-width="3"
        >
          <path stroke-linecap="round" stroke-linejoin="round" d="M19 9l-7 7-7-7" />
        </svg>
      </button>

      <div
        v-if="isBoardMenuOpen"
        class="absolute right-0 z-50 w-48 mt-2 overflow-hidden bg-gray-100 rounded-lg shadow-xl top-full"
      >
        <button
          v-if="connectedBoards.length === 0"
          type="button"
          disabled
          class="w-full px-4 py-2.5 text-left text-xs font-bold tracking-wider text-gray-400 truncate"
        >
          No boards connected
        </button>
        <button
          v-for="board in connectedBoards"
          :key="board.port_name"
          type="button"
          @click="selectBoard(board)"
          :class="[
            'w-full px-4 py-2.5 text-left text-xs font-bold tracking-wider transition-colors duration-200 truncate',
            board.port_name === activeBoard?.port_name
              ? 'bg-gray-300 text-gray-900'
              : 'text-gray-700 hover:bg-gray-200'
          ]"
        >
          {{ board.product || board.port_name }}
        </button>
      </div>

      <div
        v-if="isBoardMenuOpen"
        class="fixed inset-0 z-40"
        @click="closeBoardMenu"
      ></div>
    </div>

  </nav>
</template>