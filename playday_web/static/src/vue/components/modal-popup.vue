<template>
  <teleport to="body">
    <div v-if="modalOpen" class="modal transition-opacity duration-300 fixed w-full h-full top-0 left-0 bg-smoke-light flex items-center justify-center">
      <div class="modal-overlay absolute w-full h-full bg-gray-900 opacity-50"></div>
      <slot></slot>
    </div>
  </teleport>
</template>

<script>
export default {
  props: {
    modalOpen: {
      type: Boolean,
      default: false
    }
  },
};
</script>
