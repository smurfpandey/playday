<template>
  <div class="py-16">
    <div class="container mx-auto p-5">
      <section class="text-white">
        <div class="container mx-auto flex px-5 py-6 items-center justify-center flex-col">
          <div class="text-center lg:w-2/3 w-full">
            <h1 class="title-font sm:text-4xl text-3xl mb-4 font-medium text-white font-mono">
              Playday
            </h1>
            <p class="leading-relaxed mb-8 font-normal">
              Welcome {{ user.name }}!.
            </p>
          </div>
        </div>
      </section>
      <section>
        <div class="content">
          <div class="
              flex
              items-center
              justify-between
              w-full
              my-4
              pl-4
              sm:pr-4
              text-white
            ">
            <div class="mr-6">
              <h2 class="text-3xl md:text-4xl font-semibold tracking-tight leading-7 md:leading-10 mb-1 truncate">
                Games Wishlist
              </h2>
              <div class="font-base tracking-tight text-gray-600">
                Games I wish to play, someday!.
              </div>
            </div>
            <div class="flex items-center">
              <add-game-popup />
            </div>
          </div>
          <div v-if="isLoading" class="text-center mt-10">
              <svg class="icon-loader animate-spin h-10 w-10 text-white m-auto" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="2"></circle>
                <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
              </svg>
            </div>
          <div class="grid mt-8 gap-8 grid-cols-1 md:grid-cols-2 xl:grid-cols-2">
            <game-card v-for="game in games" :game="game.igdb_info" :key="game.id" />
          </div>
        </div>
      </section>
    </div>
  </div>
</template>

<script>
import GameCard from "./components/game-card.vue";
import AddGamePopup from "./components/add-games-to-wishlist.vue";
export default {
  components: {
    AddGamePopup,
    GameCard,
  },

  data() {
    return {
      user: window.USER,
      games: [],
      isLoading: false
    };
  },

  async created() {
    this.isLoading = true;
    const response = await fetch(`/api/wishlist`, {
      method: "GET",
      headers: {
        "Content-Type": "application/json;charset=utf-8",
      },
    });
    this.isLoading = false;
    if (response.status !== 200) {
      return alert("Halla bol!");
    }

    this.games = await response.json();
  },

  methods: {},
};
</script>
