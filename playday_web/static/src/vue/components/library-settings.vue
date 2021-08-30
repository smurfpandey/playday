<template>
  <button @click="showModal" class="bg-gray-900 px-5 py-2 text-sm shadow-sm font-semibold tracking-wider text-white rounded-full hover:bg-gray-800">
    Library Settings
  </button>
  <modal-popup :modalOpen="modalOpen">
    <slot>
      <transition enter-active-class="transition ease-out duration-300 transform " enter-from-class="opacity-0 translate-y-10 scale-95" enter-to-class="opacity-100 translate-y-0 scale-100" leave-active-class="ease-in duration-200" leave-from-class="opacity-100 translate-y-0 scale-100" leave-to-class="opacity-0 translate-y-10 translate-y-0 scale-95">
        <div v-show="modalOpen" class="modal-container bg-white w-11/12 md:max-w-5xl mx-auto rounded shadow-lg z-50 overflow-y-auto">
          <div id="librarySettings" class="modal-content py-4 text-left px-6">
            <svg style="position: absolute; width: 0; height: 0; overflow: hidden" version="1.1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink">
              <defs>
                <symbol id="icon-loading" viewBox="0 0 32 32" class="text-black">
                  <path d="M16 0c-8.711 0-15.796 6.961-15.995 15.624 0.185-7.558 5.932-13.624 12.995-13.624 7.18 0 13 6.268 13 14 0 1.657 1.343 3 3 3s3-1.343 3-3c0-8.837-7.163-16-16-16zM16 32c8.711 0 15.796-6.961 15.995-15.624-0.185 7.558-5.932 13.624-12.995 13.624-7.18 0-13-6.268-13-14 0-1.657-1.343-3-3-3s-3 1.343-3 3c0 8.837 7.163 16 16 16z"></path>
                </symbol>
              </defs>
            </svg>
            <!--Title-->
            <div class="flex justify-between items-center pb-3">
              <p class="text-2xl font-bold">Library Settings!</p>
              <div class="modal-close cursor-pointer z-50" v-on:click="closeModal">
                <svg class="fill-current text-black" xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 18 18">
                  <path d="M14.53 4.53l-1.06-1.06L9 7.94 4.53 3.47 3.47 4.53 7.94 9l-4.47 4.47 1.06 1.06L9 10.06l4.47 4.47 1.06-1.06L10.06 9z"></path>
                </svg>
              </div>
            </div>

            <!--Body-->
            <div>
              <TabGroup vertical as="div" class="flex mb-4 h-96">
                <aside class="flex flex-col items-center h-full w-1/4 border-r-2">
                  <TabList class="w-full">
                    <nav class="flex-grow md:block pr-2 overflow-y-auto">
                      <Tab v-for="platform in platforms" :key="platform.code" as="template" v-slot="{ selected }">
                        <a :class="[
                          'block px-4 py-2 mt-2 text-sm font-semibold text-gray-900 rounded-lg  hover:text-gray-900 focus:text-gray-900 hover:bg-gray-200 focus:bg-gray-200 focus:outline-none focus:shadow-outline',
                          selected ? 'bg-gray-200' : 'bg-transparent'
                        ]" href="#">
                          {{ platform.name }}
                        </a>
                      </Tab>
                    </nav>
                  </TabList>
                </aside>
                <TabPanels class="flex px-4 flex-col w-3/4 text-gray-700 bg-white dark-mode:text-gray-200 dark-mode:bg-gray-800 flex-shrink-0">
                  <TabPanel>
                    <EpicLibrarySettings />
                  </TabPanel>
                  <TabPanel>
                    <article class="prose py-2">
                      <h2>Steam</h2>
                      <p>
                        For years parents have espoused the health benefits of eating garlic bread with cheese to their
                        children, with the food earning such an iconic status in our culture that kids will often dress
                        up as warm, cheesy loaf for Halloween.
                      </p>
                      <p>
                        But a recent study shows that the celebrated appetizer may be linked to a series of rabies cases
                        springing up around the country.
                      </p>
                      <!-- ... -->
                    </article>
                  </TabPanel>
                  <TabPanel>
                    <article class="prose lg:prose-xl">
                      <h1>Gog</h1>
                      <p>
                        For years parents have espoused the health benefits of eating garlic bread with cheese to their
                        children, with the food earning such an iconic status in our culture that kids will often dress
                        up as warm, cheesy loaf for Halloween.
                      </p>
                      <p>
                        But a recent study shows that the celebrated appetizer may be linked to a series of rabies cases
                        springing up around the country.
                      </p>
                      <!-- ... -->
                    </article>
                  </TabPanel>
                  <TabPanel>
                    <article class="prose lg:prose-xl">
                      <h1>EA Origin</h1>
                      <p>
                        For years parents have espoused the health benefits of eating garlic bread with cheese to their
                        children, with the food earning such an iconic status in our culture that kids will often dress
                        up as warm, cheesy loaf for Halloween.
                      </p>
                      <p>
                        But a recent study shows that the celebrated appetizer may be linked to a series of rabies cases
                        springing up around the country.
                      </p>
                      <!-- ... -->
                    </article>
                  </TabPanel>

                  <TabPanel>
                    <article class="prose lg:prose-xl">
                      <h1>Uplay</h1>
                      <p>
                        For years parents have espoused the health benefits of eating garlic bread with cheese to their
                        children, with the food earning such an iconic status in our culture that kids will often dress
                        up as warm, cheesy loaf for Halloween.
                      </p>
                      <p>
                        But a recent study shows that the celebrated appetizer may be linked to a series of rabies cases
                        springing up around the country.
                      </p>
                      <!-- ... -->
                    </article>
                  </TabPanel>
                  <TabPanel>
                    <article class="prose lg:prose-xl">
                      <h1>Itch.io</h1>
                      <p>
                        For years parents have espoused the health benefits of eating garlic bread with cheese to their
                        children, with the food earning such an iconic status in our culture that kids will often dress
                        up as warm, cheesy loaf for Halloween.
                      </p>
                      <p>
                        But a recent study shows that the celebrated appetizer may be linked to a series of rabies cases
                        springing up around the country.
                      </p>
                      <!-- ... -->
                    </article>
                  </TabPanel>
                </TabPanels>
              </TabGroup>
            </div>
          </div>
        </div>
      </transition>
    </slot>
  </modal-popup>
</template>
<script>
import { onMounted, ref } from 'vue';
import { TabGroup, TabList, Tab, TabPanels, TabPanel } from "@headlessui/vue";

import ModalPopup from "./modal-popup.vue";
import EpicLibrarySettings from "./library/epic-games-settings.vue";

export default {
  components: {
    ModalPopup,
    TabGroup,
    TabList,
    Tab,
    TabPanels,
    TabPanel,
    EpicLibrarySettings,
  },

  data() {
    return {
      modalOpen: false,
      platforms: [
        {
          name: "Epic Games",
          code: "epic",
        },
        {
          name: "Steam",
          code: "steam",
        },
        {
          name: "Gog Games",
          code: "gog",
        },
        {
          name: "EA Origin",
          code: "origin",
        },
        {
          name: "Uplay",
          code: "uplay",
        },
        {
          name: "Itch.io",
          code: "itch",
        },
      ],
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
    },
  },
};
</script>
