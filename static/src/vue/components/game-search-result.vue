<template>
  <game-card :game="game" :isDBItem="false">
    <button
      v-if="!isSelected"
      @click.prevent="$emit('select', game)"
      class="
        btnSelectGame
        mb-2
        md:mb-0
        bg-gray-900
        px-5
        py-2
        shadow-sm
        tracking-wider
        text-white
        rounded-full
        hover:bg-gray-800
      "
      type="button"
      aria-label="like"
    >
      Select
    </button>
  </game-card>
</template>

<script>
import GameCard from "./game-card.vue";
export default {
  components: {
    GameCard,
  },
  props: {
    game: {
      required: true,
      type: Object,
    },
    isSelected: {
      type: Boolean,
      default: false,
    },
  },
};
</script>
