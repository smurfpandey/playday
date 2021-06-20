<template>
  <div class="bg-white shadow-md rounded-3xl p-4">
    <div class="flex-none lg:flex">
      <div class="h-full w-full lg:h-48 lg:w-36 lg:mb-0 mb-3">
        <img v-if="game.cover" v-bind:src="
            'https://images.igdb.com/igdb/image/upload/t_cover_big/' +
            game.cover.image_id +
            '.jpg'
          " alt="Just a flower" class="w-full object-scale-down lg:object-cover lg:h-48 rounded-2xl" />
      </div>
      <div class="flex-auto ml-3 justify-evenly py-2">
        <div class="flex flex-wrap items-center justify-between">
          <h2 class="text-lg font-medium mr-6">{{ game.name }}</h2>
          <div class="flex" v-if="isDBItem">
            <remove-game @remove="$emit('remove')" :title="game.name" :isLoading="isRemoving" />
          </div>
        </div>
        <p class="mt-3"></p>
        <div class="flex py-4 text-sm text-gray-600">
          <div class="flex-1 inline-flex items-center">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16m14 0h2m-2 0h-5m-9 0H3m2 0h5M9 7h1m-1 4h1m4-4h1m-1 4h1m-5 10v-5a1 1 0 011-1h2a1 1 0 011 1v5m-4 0h4" />
            </svg>
            <p class="">{{ game.id }}</p>
          </div>
          <div class="flex-1 inline-flex items-center">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"></path>
            </svg>
            <p class="" v-bind:title="$filters.epochToHuman(game.first_release_date)">{{ $filters.epochToRelative(game.first_release_date) }}</p>
          </div>
        </div>
        <div class="flex p-4 pb-2 border-t border-gray-200"></div>
        <div class="flex space-x-3 text-sm font-medium">
          <div class="flex-auto flex space-x-3">
            <div v-if="game.total_rating" class="mb-2 md:mb-0 bg-white px-5 py-2 shadow-sm tracking-wider border text-gray-600 rounded-full inline-flex items-center space-x-2">
              <span class="text-green-400 rounded-lg">
                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                  <path d="M9.049 2.927c.3-.921 1.603-.921 1.902 0l1.07 3.292a1 1 0 00.95.69h3.462c.969 0 1.371 1.24.588 1.81l-2.8 2.034a1 1 0 00-.364 1.118l1.07 3.292c.3.921-.755 1.688-1.54 1.118l-2.8-2.034a1 1 0 00-1.175 0l-2.8 2.034c-.784.57-1.838-.197-1.539-1.118l1.07-3.292a1 1 0 00-.364-1.118L2.98 8.72c-.783-.57-.38-1.81.588-1.81h3.461a1 1 0 00.951-.69l1.07-3.292z" />
                </svg>
              </span>
              <span>{{ parseInt(game.total_rating, 10) }}</span>
            </div>
          </div>
          <slot></slot>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import RemoveGame from './remove-game.vue';

export default {
  components: {
    RemoveGame,
  },

  props: {
    game: {
      required: true,
      type: Object,
    },
    isDBItem: {
      required: false,
      type: Boolean,
      default: true,
    },
    isRemoving: {
      type: Boolean,
      default: false,
    },
  },
};
</script>
