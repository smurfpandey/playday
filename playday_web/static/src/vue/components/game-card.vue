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
          <template v-for="release_date in game.release_dates" :key="release_date.id">
          <div v-if="getPlatform(release_date.platform)" class="flex-1 inline-flex items-center mr-2">
            <game-platform-icon :platform="getPlatform(release_date.platform)" />
            <p class="" v-bind:title="$filters.epochToHuman(release_date.date)">{{ $filters.epochToRelative(release_date.date) }}</p>
          </div>
          </template>
        </div>
        <div class="flex p-4 pb-2 border-t border-gray-200"></div>
        <div class="flex space-x-3 text-sm font-medium">
          <div class="flex-auto flex space-x-3">
            <template v-for="website in game.websites" :key="website.id">
            <div v-if="getStore(website)" class="inline-flex items-center mr-2">
              <a v-bind="{ href: website.url }" target="_blank">
                <game-store-icon :store="getStore(website)" />
              </a>
            </div>
            </template>
          </div>
          <slot></slot>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import RemoveGame from './remove-game.vue';
import GamePlatformIcon from './game-platform-icon.vue';
import GameStoreIcon from './game-store-icon.vue';

export default {
  components: {
    GamePlatformIcon,
    GameStoreIcon,
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

  methods: {
    getPlatform: (platform) => {
      if(platform.id === 6) {
        return { code: 'PC', name: platform.name };
      }
      if(platform.platform_family === 2) {
        return { code: 'XBOX', name: platform.name };
      }
      if(platform.platform_family === 1) {
        return { code: 'PS', name: platform.name };
      }
    },

    getStore: (website) => {
      switch(website.category) {
        case 13: {
          return { code: 'steam', name: 'Steam' };
        }
        case 16: {
          return { code: 'epic', name: 'Epic Games' };
        }
      }
    },
  }
};
</script>
