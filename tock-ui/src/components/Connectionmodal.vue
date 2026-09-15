<script setup>
import { ref, watch, onUnmounted } from 'vue';

const props = defineProps({
  show: {
    type: Boolean,
    default: false
  }
});

const emit = defineEmits(['close']);


const step = ref('type');

const selectedBoard = ref('');
const selectedConnection = ref('');

const detectionResult = ref(null);

const installProgress = ref(0);
let progressInterval = null;

const knownBoards = [
  'NUCLEO-F401RE',
  'NUCLEO-F446RE',
  'STM32F4 Discovery',
  'Microbit'
];

const connectionTypes = [
  'USB',
  'UART',
  'SWD',
  'JTAG'
];

const resetState = () => {
  step.value = 'type';
  selectedBoard.value = '';
  selectedConnection.value = '';
  detectionResult.value = null;
  installProgress.value = 0;
  clearProgressInterval();
};

watch(() => props.show, (val) => {
  if (val) resetState();
});

const clearProgressInterval = () => {
  if (progressInterval) {
    clearInterval(progressInterval);
    progressInterval = null;
  }
};

onUnmounted(clearProgressInterval);

const selectType = (type) => {
  step.value = type === 'known' ? 'known-board' : 'custom-connection';
};

const goBack = () => {
  if (['detecting', 'result', 'not-found'].includes(step.value)) {
    step.value = selectedBoard.value ? 'known-board' : 'custom-connection';
    return;
  }
  if (step.value === 'installing' || step.value === 'install-failed') {
    step.value = 'result';
    return;
  }
  step.value = 'type';
};

const runDetection = () => {
  step.value = 'detecting';
  detectionResult.value = null;

  // This will be replaced with real Tauri backend call
  setTimeout(() => {
    const result = {
      success: true, // toggled locally
      tockloaderInstalled: false
    };

    detectionResult.value = result;
    step.value = result.success ? 'result' : 'not-found';
  }, 2000);
};

const continueKnownBoard = () => {
  if (!selectedBoard.value) return;
  runDetection();
};

const continueCustomConnection = () => {
  if (!selectedConnection.value) return;
  runDetection();
};

const retryDetection = () => {
  runDetection();
};

const installTock = () => {
  step.value = 'installing';
  installProgress.value = 0;
  clearProgressInterval();

  // This will be replaced with real progress from backend.
  
  progressInterval = setInterval(() => {
    installProgress.value += 10;

    if (installProgress.value >= 100) {
      clearProgressInterval();

      // Simulated outcome
      const installSucceeded = true;
      step.value = installSucceeded ? 'installed' : 'install-failed';
    }
  }, 300);
};

const retryInstall = () => {
  installTock();
};

const addAnotherBoard = () => {
  resetState();
};

const finish = () => {
  emit('close');
};

const activeDot = ref(0);
watch(step, (val) => {
  const map = {
    type: 0,
    'known-board': 1,
    'custom-connection': 1,
    detecting: 2,
    'not-found': 2,
    result: 3,
    installing: 3,
    'install-failed': 3,
    installed: 4
  };
  activeDot.value = map[val] ?? 0;
});
</script>

<template>
  <Teleport to="body">
    <div
      v-if="show"
      class="fixed inset-0 z-[9999] flex items-center justify-center bg-black/60"
      style="position: fixed; top: 0; left: 0; right: 0; bottom: 0;"
      @click.self="emit('close')"
    >
      <div class="relative w-full max-w-2xl mx-4 p-12 bg-neutral-content rounded-xl shadow-2xl">

        <button
          v-if="step !== 'type'"
          @click="goBack"
          class="absolute flex items-center justify-center w-10 h-10 text-gray-300 transition-colors duration-200 top-6 left-6 hover:text-white"
        >
          <svg xmlns="http://www.w3.org/2000/svg" class="w-7 h-7" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
            <path stroke-linecap="round" stroke-linejoin="round" d="M10 19l-7-7m0 0l7-7m-7 7h18" />
          </svg>
        </button>

        <button
          @click="emit('close')"
          class="absolute flex items-center justify-center w-10 h-10 text-gray-300 bg-secondary rounded-full hover:bg-[#605F5F] transition-colors duration-200 top-6 right-6"
        >
          ✕
        </button>

        <template v-if="step === 'type'">
          <h2 class="mb-10 text-3xl font-bold tracking-wide text-center text-gray-100">
            SELECT CONNECTION TYPE
          </h2>

          <div class="flex justify-center gap-6">
            <button
              @click="selectType('known')"
              class="px-8 py-4 text-lg font-bold text-gray-900 bg-accent rounded-lg hover:bg-white transition-colors duration-200"
            >
              KNOWN BOARD
            </button>
            <button
              @click="selectType('custom')"
              class="px-16 py-4 text-lg font-bold text-gray-900 bg-accent rounded-lg hover:bg-white transition-colors duration-200"
            >
              CUSTOM
            </button>
          </div>
        </template>

        <template v-else-if="step === 'known-board'">
          <h2 class="mb-10 text-3xl font-bold tracking-wide text-center text-gray-100">
            SELECT BOARD
          </h2>

          <div class="flex flex-col items-center gap-4">
            <select
              v-model="selectedBoard"
              class="w-full max-w-md px-4 py-3 text-gray-800 bg-accent rounded-lg outline-none appearance-none cursor-pointer"
            >
              <option value="" disabled>Select your board</option>
              <option v-for="board in knownBoards" :key="board" :value="board">
                {{ board }}
              </option>
            </select>

            <button
              @click="continueKnownBoard"
              :disabled="!selectedBoard"
              class="w-full max-w-md px-4 py-3 font-bold text-gray-900 bg-accent rounded-lg hover:bg-white disabled:opacity-50 disabled:cursor-not-allowed transition-colors duration-200"
            >
              Continue
            </button>
          </div>
        </template>

        <template v-else-if="step === 'custom-connection'">
          <h2 class="mb-10 text-3xl font-bold tracking-wide text-center text-gray-100">
            SELECT YOUR CONNECTION
          </h2>

          <div class="flex flex-col items-center gap-4">
            <select
              v-model="selectedConnection"
              class="w-full max-w-md px-4 py-3 text-gray-800 bg-accent rounded-lg outline-none appearance-none cursor-pointer"
            >
              <option value="" disabled>Select your connection type</option>
              <option v-for="conn in connectionTypes" :key="conn" :value="conn">
                {{ conn }}
              </option>
            </select>

            <button
              @click="continueCustomConnection"
              :disabled="!selectedConnection"
              class="w-full max-w-md px-4 py-3 font-bold text-gray-900 bg-accent rounded-lg hover:bg-white disabled:opacity-50 disabled:cursor-not-allowed transition-colors duration-200"
            >
              Continue
            </button>
          </div>
        </template>

        <template v-else-if="step === 'detecting'">
          <h2 class="mb-10 text-3xl font-bold tracking-wide text-center text-gray-100">
            DETECTING BOARD
          </h2>

          <div class="flex items-center justify-center py-6">
            <div class="w-12 h-12 border-4 border-gray-500 rounded-full border-t-gray-200 animate-spin"></div>
          </div>
        </template>

        <template v-else-if="step === 'not-found'">
          <h2 class="mb-6 text-3xl font-bold tracking-wide text-center text-gray-100">
            BOARD NOT DETECTED
          </h2>

          <p class="mb-8 text-center text-gray-400">
            We couldn't find "{{ selectedBoard || selectedConnection }}" on the connection. Check the wiring and try again.
          </p>

          <div class="flex justify-center gap-4">
            <button
              @click="retryDetection"
              class="px-8 py-4 text-lg font-bold text-gray-900 bg-accent rounded-lg hover:bg-white transition-colors duration-200"
            >
              RETRY
            </button>
          </div>
        </template>

        <template v-else-if="step === 'result'">
          <h2 class="mb-10 text-3xl font-bold tracking-wide text-center text-gray-100">
            TOCKLOADER-RS
          </h2>

          <div class="flex justify-center">
            <button
              @click="installTock"
              class="px-8 py-4 text-lg font-bold text-gray-900 bg-accent rounded-lg hover:bg-white transition-colors duration-200"
            >
              INSTALL TOCK
            </button>
          </div>
        </template>

        <!-- Progress -->
        <template v-else-if="step === 'installing'">
          <h2 class="mb-6 text-3xl font-bold tracking-wide text-center text-gray-100">
            INSTALLATION IN PROGRESS
          </h2>

          <div class="flex flex-col items-center gap-3">
            <div class="w-full max-w-md h-3 overflow-hidden bg-secondary rounded-full">
              <div
                class="h-full transition-all duration-300 ease-out rounded-full bg-accent"
                :style="{ width: installProgress + '%' }"
              ></div>
            </div>
            <span class="text-sm font-bold text-gray-300">{{ installProgress }}%</span>
          </div>
        </template>

        <!-- If the installation failed -->
        <template v-else-if="step === 'install-failed'">
          <h2 class="mb-6 text-3xl font-bold tracking-wide text-center text-gray-100">
            INSTALLATION FAILED
          </h2>

          <p class="mb-8 text-center text-gray-400">
            Something went wrong while installing Tock. Please try again.
          </p>

          <div class="flex justify-center">
            <button
              @click="retryInstall"
              class="px-8 py-4 text-lg font-bold text-gray-900 bg-accent rounded-lg hover:bg-white transition-colors duration-200"
            >
              TRY AGAIN
            </button>
          </div>
        </template>

        <!-- If it is installed successfully -->

        <template v-else-if="step === 'installed'">
          <h2 class="mb-2 text-3xl font-bold tracking-wide text-center text-gray-100">
            TOCK INSTALLED SUCCESSFULLY
          </h2>
          <p class="mb-8 text-lg font-bold tracking-wide text-center text-gray-300">
            DO YOU WISH TO ADD ANOTHER BOARD?
          </p>

          <div class="flex justify-center gap-4 ">
            
            <button
              @click="addAnotherBoard"
              class="px-10 py-2 text-lg font-bold text-gray-200 border-2 border-gray-500 rounded-lg hover:border-gray-300 hover:text-white transition-colors duration-200"
            >
              ADD BOARD
            </button>
          
            <button
              @click="finish"
              class="px-10 py-2 text-lg font-bold text-gray-900 bg-accent rounded-lg hover:bg-white transition-colors duration-200"
            >
              FINISH
            </button>

          </div>
        </template>

        
        <div class="flex justify-center gap-3 mt-10">
          <span
            v-for="n in 5"
            :key="n"
            class="w-3 h-3 rounded-full"
            :class="n - 1 === activeDot ? 'bg-accent' : 'bg-secondary'"
          ></span>
        </div>

      </div>
    </div>
  </Teleport>
</template>