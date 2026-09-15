<script setup>
import { ref } from 'vue';
import Connectionmodal from '../components/Connectionmodal.vue';
import Navbar from '../components/Navbar.vue';

const isModalOpen = ref(false);
</script>

<template>
  <div class="flex flex-col min-h-screen">

    <Navbar />

    <div class="flex flex-col items-center justify-center flex-1 w-full gap-8 text-center text-gray-200">

        <h1 class="text-5xl">No board detected.<br/> Click here to set up your board</h1>

        <button 
          @click="isModalOpen = true" 
          class="px-8 py-3 text-lg shadow-lg font-bold text-white transition-colors duration-200 bg-secondary rounded-lg hover:bg-[#605F5F]"
        >
          Choose Board
        </button>
        
    </div>
    
    <Connectionmodal 
      :show="isModalOpen" 
      @close="isModalOpen = false" 
    />
    
  </div>
</template>