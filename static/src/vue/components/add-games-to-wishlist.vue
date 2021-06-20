<template>
  <button @click="showModal" class="bg-gray-900 px-5 py-2 text-sm shadow-sm font-semibold tracking-wider text-white rounded-full hover:bg-gray-800">
    Add Game
  </button>
  <modal-popup :modalOpen="modalOpen">
    <slot>
      <transition enter-active-class="transition ease-out duration-300 transform " enter-from-class="opacity-0 translate-y-10 scale-95" enter-to-class="opacity-100 translate-y-0 scale-100" leave-active-class="ease-in duration-200" leave-from-class="opacity-100 translate-y-0 scale-100" leave-to-class="opacity-0 translate-y-10 translate-y-0 scale-95">
        <div v-show="modalOpen" class="modal-container bg-white w-11/12 md:max-w-5xl mx-auto rounded shadow-lg z-50 overflow-y-auto">
          <div id="searchGamesApp" class="modal-content py-4 text-left px-6">
            <svg style="position: absolute; width: 0; height: 0; overflow: hidden" version="1.1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink">
              <defs>
                <symbol id="icon-loading" viewBox="0 0 32 32" class="text-black">
                  <path d="M16 0c-8.711 0-15.796 6.961-15.995 15.624 0.185-7.558 5.932-13.624 12.995-13.624 7.18 0 13 6.268 13 14 0 1.657 1.343 3 3 3s3-1.343 3-3c0-8.837-7.163-16-16-16zM16 32c8.711 0 15.796-6.961 15.995-15.624-0.185 7.558-5.932 13.624-12.995 13.624-7.18 0-13-6.268-13-14 0-1.657-1.343-3-3-3s-3 1.343-3 3c0 8.837 7.163 16 16 16z"></path>
                </symbol>
              </defs>
            </svg>
            <!--Title-->
            <div class="flex justify-between items-center pb-3">
              <p class="text-2xl font-bold">Add Game!</p>
              <div class="modal-close cursor-pointer z-50" v-on:click="closeModal">
                <svg class="fill-current text-black" xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 18 18">
                  <path d="M14.53 4.53l-1.06-1.06L9 7.94 4.53 3.47 3.47 4.53 7.94 9l-4.47 4.47 1.06 1.06L9 10.06l4.47 4.47 1.06-1.06L10.06 9z"></path>
                </svg>
              </div>
            </div>

            <!--Body-->
            <div class="flex flex-col mb-4">
              <div class="mt-6 bg-transparent border rounded-md dark:border-gray-700 focus-within:ring ring-primary focus-within:border-teal-500 ring-opacity-40">
                <div class="flex flex-wrap justify-between md:flex-row">
                  <input id="txtSearchGames" v-on:keyup.enter="search" v-model="keyword" type="text" name="query" placeholder="Search Components" required="required" class="flex-1
                      px-4 h-10 lg:h-12 m-1 text-gray-700
                      placeholder-gray-400
                      bg-transparent
                      border-none
                      appearance-none
                      dark:text-gray-200
                      focus:outline-none
                      focus:placeholder-transparent focus:ring-0" />
                  <button id="btnSearchGames" v-on:click="search" v-show="keyword.length > 0" v-bind:disabled="isSearching" v-bind:class="{ 'cursor-not-allowed': isSearching }" class="
                      flex
                      justify-center
                      items-center
                      w-full
                      lg:w-12
                      lg:h-12
                      p-2
                      lg:p-0
                      m-1
                      text-white
                      transition-colors
                      duration-200
                      transform
                      rounded-md
                      bg-green-600
                      hover:bg-green-700
                      focus:outline-none
                      focus:bg-green-700
                    ">
                    <svg v-bind:class="{ hidden: isSearching }" class="icon-search w-6 h-6" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"></path>
                    </svg>
                    <svg v-bind:class="{ hidden: !isSearching }" class="icon-loader animate-spin h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                      <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="2"></circle>
                      <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                    </svg>
                  </button>
                </div>
              </div>
            </div>
            <div id="searchResult" class="flex flex-col mb-4 overflow-y-auto h-80">
              <div class="grid mt-8 gap-8 grid-cols-1 md:grid-cols-2 xl:grid-cols-2">
                <game-search-result v-for="game in games" :key="game.id" :game="game" @select="selectGame" />
              </div>
            </div>

            <!--Footer-->
            <div class="flex flex-col text-center">
              <div id="dvSelectedGamesToAdd" class="flex flex-wrap">
                <div v-for="game in selectedGames" :key="game.id" class="inline-flex rounded-md overflow-hidden mb-4 mr-4 selected-game">
                  <span class=" block text-white text-sm shadow-border bg-blue-400 py-2 px-3 font-sans tracking-wide uppercase font-bold">
                    {{ game.name }}
                  </span>
                  <div class="bg-blue-400 hover:bg-blue-500 rounded-r-md cursor-pointer p-2">
                    <div class="w-4 h-4">
                      <button class="remove-selected" v-on:click="nukeSelf(game.id)">
                        <svg class="fill-current text-white" xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 18 18">
                          <path d="M14.53 4.53l-1.06-1.06L9 7.94 4.53 3.47 3.47 4.53 7.94 9l-4.47 4.47 1.06 1.06L9 10.06l4.47 4.47 1.06-1.06L10.06 9z"></path>
                        </svg>
                      </button>
                    </div>
                  </div>
                </div>
              </div>
              <div class="
                  font-medium
                  tracking-wide
                  text-xs text-left
                  float-right
                  mb-1
                  ml-1
                ">
                <span id="spFormError" class="text-red-500"></span>
              </div>
              <button v-show="selectedGames.length > 0" v-on:click="addToWishlist()" v-bind:disabled="isSaving" v-bind:class="{ 'cursor-not-allowed': isSaving }" class="
                  inline-flex
                  items-center
                  justify-center
                  border border-transparent
                  font-medium
                  bg-gray-800
                  text-white
                  hover:bg-gray-900
                  text-sm
                  font-bold
                  uppercase
                  px-6
                  py-3
                  rounded
                  shadow hover:shadow-lg outline-none focus:outline-none mr-1 mb-1 w-full" type="button" style="transition: all 0.15s ease 0s">
                <svg v-bind:class="{ hidden: !isSaving }" class="animate-spin -ml-1 mr-3 h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                  <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                  <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                </svg>
                Save
              </button>
            </div>
          </div>
        </div>
      </transition>
    </slot>
  </modal-popup>
</template>

<script>
import reject from "lodash/reject";
import lodashFind from "lodash/find";
import GameSearchResult from "./game-search-result.vue";
import ModalPopup from "./modal-popup.vue";

export default {
  components: {
    GameSearchResult,
    ModalPopup,
  },

  data() {
    return {
      modalOpen: false,
      keyword: "",
      isSearching: false,
      isSaving: false,
      games: [],
      selectedGames: [],
    };
  },

  methods: {
    showModal() {
      document.querySelector("body").classList.toggle("modal-active");
      this.modalOpen = true;
    },

    closeModal() {
      document.querySelector("body").classList.toggle("modal-active");
      this.modalOpen = false;
      this.keyword = "";
      this.isSearching = false;
      this.isSaving = false;
      this.games = [];
      this.selectedGames = [];
    },
    async search(event) {
      const searchKeyword = this.keyword;
      if (!searchKeyword) {
        return;
      }

      this.isSearching = true;
      const response = await fetch(`/api/search?keyword=${searchKeyword}`, {
        method: "GET",
        headers: {
          "Content-Type": "application/json;charset=utf-8",
        },
      });
      this.isSearching = false;

      if (response.status !== 200) {
        return alert("Halla bol!");
      }

      this.games = await response.json();
    },
    isGameSelected(gameId) {
      const thisGame = lodashFind(this.selectedGames, (game) => {
        return game.id === gameId;
      });

      if (thisGame) {
        return true;
      } else {
        return false;
      }
    },
    selectGame(igdbGame) {
      if (!this.isGameSelected(igdbGame.id)) {
        this.selectedGames.push(igdbGame);
      }
    },
    nukeSelf(gameId) {
      this.selectedGames = reject(this.selectedGames, (game) => {
        return game.id === gameId;
      });
    },
    async addToWishlist() {
      if (this.selectedGames.length === 0) {
        return;
      }
      this.isSaving = true;
      const response = await fetch("/api/wishlist", {
        method: "POST",
        headers: {
          "Content-Type": "application/json;charset=utf-8",
        },
        body: JSON.stringify(this.selectedGames),
      });
      this.isSaving = false;
      if (response.status !== 200) {
        return alert("Udi baba!");
      }

      this.closeModal();
    },
  },
};
</script>
