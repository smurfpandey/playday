<template>
  <a href="#" @click.prevent="modalOpen = true" class="text-red-500 text-sm tracking-wider hover:text-red-600">
    <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
    </svg>
  </a>
  <modal-popup :modalOpen="modalOpen">
    <slot>
      <div class="bg-white rounded-lg z-50">
        <div class="w-96 border-t-8 border-pink-600 rounded-lg flex">
          <div class="w-1/3 pt-6 flex justify-center">
            <svg xmlns="http://www.w3.org/2000/svg" class="w-16 h-16 bg-pink-600 text-white p-3 rounded-full" fill="none" viewBox="0 0 24 24" height="24" width="24">
              <path xmlns="http://www.w3.org/2000/svg" d="M12 4.52765C9.64418 2.41689 6.02125 2.49347 3.75736 4.75736C1.41421 7.1005 1.41421 10.8995 3.75736 13.2426L10.5858 20.0711C11.3668 20.8521 12.6332 20.8521 13.4142 20.0711L20.2426 13.2426C22.5858 10.8995 22.5858 7.1005 20.2426 4.75736C17.9787 2.49347 14.3558 2.41689 12 4.52765ZM10.8284 6.17157L11.2929 6.63604C11.6834 7.02656 12.3166 7.02656 12.7071 6.63604L13.1716 6.17157C14.7337 4.60948 17.2663 4.60948 18.8284 6.17157C20.3905 7.73367 20.3905 10.2663 18.8284 11.8284L12 18.6569L5.17157 11.8284C3.60948 10.2663 3.60948 7.73367 5.17157 6.17157C6.73367 4.60948 9.26633 4.60948 10.8284 6.17157Z" fill="currentcolor"></path>
            </svg>
          </div>
          <div class="w-full pt-9 pr-4">
            <h3 class="font-bold text-pink-700">Delete {{ title }}?</h3>
            <p class="py-4 text-sm text-gray-400">Are you sure you want to delete this game? If you delete this game, you will have to add the game again.</p>
          </div>
        </div>

        <div class="p-4 flex space-x-4">
          <a href="#" @click.prevent="modalOpen = false" class="w-1/2 px-4 py-3 text-center bg-gray-100 text-gray-400 hover:bg-gray-200 hover:text-black font-bold rounded-lg text-sm">Cancel</a>
          <a href="#" @click.prevent="$emit('remove')" class="w-1/2 px-4 py-3 text-center text-pink-100 bg-pink-600 rounded-lg hover:bg-pink-700 hover:text-white font-bold text-sm inline-flex items-center justify-center">
            <svg v-bind:class="{ hidden: !isLoading }" class="animate-spin -ml-1 mr-3 h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
              <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
              <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
            </svg>
            Delete Game
          </a>
        </div>
      </div>
    </slot>
  </modal-popup>
</template>

<script>
import ModalPopup from "./modal-popup.vue";

export default {
  components: {
    ModalPopup,
  },

  props: {
    title: {
      required: true,
      type: String,
    },
    isLoading: {
      default: false,
      type: Boolean,
    }
  },

  data() {
    return {
      modalOpen: false,
    };
  },
};
</script>
