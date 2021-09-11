<template>
  <article class="prose py-2">
    <h2>
      <span>Epic Games</span>
      <div class="float-right">
        <div class="flex items-center">
          <span v-if="isLoading" class="text-center">
            <svg class="icon-loader animate-spin h-5 w-5 mr-2 text-gray-500 m-auto" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
              <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="2"></circle>
              <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
            </svg>
          </span>
          <span v-if="isConnected" class="text-sm font-normal mr-2 text-green-500">Connected</span>
          <span v-else class="text-sm font-normal mr-2 text-red-500">Not Connected</span>
        </div>
      </div>
    </h2>
    <div v-if="isConnected">
      <div class="grid grid-cols-3 gap-4">
        <div class="text-right font-semibold mr-5">Username: </div>
        <div class="col-span-2">{{ storeSettings.store_user_name }}</div>
        <div class="text-right font-semibold mr-5">Connected On: </div>
        <div class="col-span-2">{{ $filters.iso8601ToRelative(storeSettings.added_on) }}</div>
        <div class="text-right font-semibold mr-5">Last Update On: </div>
        <div class="col-span-2">{{ $filters.iso8601ToRelative(storeSettings.updated_on) }}</div>
        <div class="text-right font-semibold mr-5">Total Games: </div>
        <div class="col-span-2">{{ storeSettings.total_games || 0 }}&nbsp;<a href="#" @click.prevent="fnSyncLibrary">Refresh Library</a></div>
        <div class="col-span-3 text-right">
          <button @click.prevent="fnDisconnect" type="button" class="px-3 py-2 text-sm bg-red-600 rounded-md text-white outline-none focus:ring-4 shadow-lg transform active:scale-x-75 transition-transform flex float-right">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
            <span class="ml-2">Disconnect</span>
          </button>
        </div>
      </div>
    </div>
    <div v-else>
      Please enter the "sid" value from JSON response post login.
      <div>
        <div class="mt-6 bg-transparent border rounded-md dark:border-gray-700 focus-within:ring ring-primary focus-within:border-teal-500 ring-opacity-40">
          <div class="flex flex-wrap justify-between md:flex-row">
            <input v-model="loginSid" id="txtLoginEpicGames" type="text" placeholder="Paste login Token here" required="required" class="flex-1
                      px-4 h-10 lg:h-12 m-1 text-gray-700
                      placeholder-gray-400
                      bg-transparent
                      border-none
                      appearance-none
                      dark:text-gray-200
                      focus:outline-none
                      focus:placeholder-transparent focus:ring-0" />
          </div>
        </div>
        <div class="grid grid-cols-2 gap-4 mt-10 justify-items-center content-center justify-center">
          <div>
            <a href="https://www.epicgames.com/id/login?redirectUrl=https://www.epicgames.com/id/api/redirect" target="_blank"
              class="no-underline px-3 py-2 text-sm bg-green-500 disabled:opacity-50 disabled:cursor-not-allowed rounded-md text-white outline-none focus:ring-4 shadow-lg transform active:scale-x-75 transition-transform flex">
              <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
              <span class="ml-2">Start Login</span>
            </a>
          </div>
          <div>
            <button @click="fnConnectToEpicGames" :disabled="!loginSid || isLoading" type="button" class="px-3 py-2 text-sm bg-blue-600 disabled:opacity-50 disabled:cursor-not-allowed rounded-md text-white outline-none focus:ring-4 shadow-lg transform active:scale-x-75 transition-transform flex">
              <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
              <span class="ml-2">Connect</span>
            </button>
          </div>
        </div>
      </div>
    </div>
    <!-- ... -->
  </article>
</template>

<script>
import { onMounted, ref, reactive } from "vue";
import { Switch, SwitchGroup, SwitchLabel } from "@headlessui/vue";

export default {
  components: { Switch, SwitchGroup, SwitchLabel },

  setup() {
    let isLoading = ref(false);
    let isConnected = ref(false);
    let storeSettings = ref({});

    let loginSid = ref('');

    const fnLoadSettings = async () => {
      isLoading.value = true;
      const response = await fetch("/api/library/epicgames/settings", {
        method: "GET",
        headers: {
          "Content-Type": "application/json;charset=utf-8",
        },
      });
      isLoading.value = false;

      if (response.status !== 200) {
        return alert("Halla bol!");
      }

      storeSettings.value = await response.json();

      if (storeSettings.value && storeSettings.value.id) {
        isConnected.value = true;
      }
      if (storeSettings.value === null) {
        storeSettings.value = {};
      }
    };

    const fnDisconnect = async () => {
      isLoading.value = true;
      const response = await fetch("/api/library/epicgames/settings", {
        method: "DELETE",
        headers: {
          "Content-Type": "application/json;charset=utf-8",
        },
      });
      isLoading.value = false;

      if (response.status !== 204) {
        return alert("Halla bol!");
      }

      storeSettings.value = {};
      isConnected.value = false;
    };

    const fnConnectToEpicGames = async () => {
      isLoading.value = true;

      const response = await fetch("/connect/epicgames/login", {
        method: "POST",
        headers: {
          "Content-Type": "application/json;charset=utf-8",
        },
        body: JSON.stringify({
          sid: loginSid.value
        })
      });

      isLoading.value = false;

      if (response.status !== 200) {
        return alert("Halla bol!");
      }

      loginSid.value = '';
      fnLoadSettings();
    }

    onMounted(() => {
      fnLoadSettings();
    });

    const fnSyncLibrary = async () => {
      isLoading.value = true;

      const response = await fetch("/api/library/epicgames/sync", {
        method: "POST",
        headers: {
          "Content-Type": "application/json;charset=utf-8",
        },
        body: JSON.stringify({
          sid: loginSid.value
        })
      });

      isLoading.value = false;

      if (response.status !== 200) {
        return alert("Halla bol!");
      }
    }

    return {
      isConnected,
      isLoading,
      storeSettings,
      fnDisconnect,
      loginSid,
      fnConnectToEpicGames,
      fnSyncLibrary
    };
  },
};
</script>
