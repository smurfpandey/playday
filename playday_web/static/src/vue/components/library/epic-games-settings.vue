<template>
  <article class="prose py-2">
    <h2>
      <span>Epic Games</span>
      <div class="float-right">
        <SwitchGroup>
          <div class="flex items-center">
            <span v-if="isLoading" class="text-center">
              <svg class="icon-loader animate-spin h-5 w-5 mr-2 text-gray-500 m-auto" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="2"></circle>
                <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
              </svg>
            </span>
            <span v-if="isConnected" class="text-sm font-normal mr-2 text-green-500">Connected</span>
            <span v-else class="text-sm font-normal mr-2 text-red-500">Not Connected</span>
            <Switch v-model="isConnected" :class='isConnected ? "bg-blue-600" : "bg-gray-200"' class="relative inline-flex items-center h-6 transition-colors rounded-full w-11 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500">
              <span :class='isConnected ? "translate-x-6" : "translate-x-1"' class="inline-block w-4 h-4 transition-transform transform bg-white rounded-full" />
            </Switch>
          </div>
        </SwitchGroup>
      </div>
    </h2>
    <div v-if="isConnected">
      For years parents have espoused the health benefits of eating garlic bread with cheese to their
      children, with the food earning such an iconic status in our culture that kids will often dress
      up as warm, cheesy loaf for Halloween.
    </div>
    <div v-else>
      But a recent study shows that the celebrated appetizer may be linked to a series of rabies cases
      springing up around the country.
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
    let libraryData = reactive({});

    const loadSettings = async () => {
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

      libraryData.value = await response.json();

      if (libraryData.value && libraryData.value.id) {
        isConnected.value = true;
      }
    };

    // watchEffect((props) => {
    //   loadSettings();
    // });

    onMounted(() => {
      loadSettings();
    });

    return {
      isConnected,
      isLoading,
      libraryData,
    };
  },
};
</script>
