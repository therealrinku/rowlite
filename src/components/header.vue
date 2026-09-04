<script>
import { defineComponent } from "vue";
import { invoke, convertFileSrc } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import SortDownFill from "@primeicons/vue/sort-down-fill";
import Code from "@primeicons/vue/code";
import StopCircle from "@primeicons/vue/stop-circle";

export default defineComponent({
  name: "Header",
  components: {
    SortDownFill,
    StopCircle,
    Code,
  },
  data() {
    return {
      isConnected: false,
      connectionName: null,
    };
  },
  async mounted() {
    window.addEventListener("db:connected", (event) => {
      this.connectionName = event.detail.connectionName;
      this.isConnected = true;
    });
    window.addEventListener("db:disconnected", (event) => {
      this.connectionName = null;
      this.isConnected = false;
    });
  },
  unmounted() {
    //window.removeEventListener('db:connected')
    //window.removeEventListener('db:disconnected')
  },
  watch: {},
  computed: {},
  methods: {
    async closeConnection() {
      await invoke("sqlite_disconnect");
      window.dispatchEvent(new CustomEvent("db:disconnected"));
    },
  },
});
</script>

<template>
  <div
    class="fixed bottom-[-2px] left-0 font-bold border-b border-gray-200 w-full pt-2 pb-2 bg-zinc-600 text-white flex items-center justify-between px-5 z-100"
  >
    <p
      class="bg-clip-text text-transparent bg-linear-to-r from-green-500 to-indigo-500"
    >
      rowlite
    </p>

    <div v-if="isConnected" class="flex items-center gap-5">
      <div class="flex items-center gap-2">
        <p>{{ connectionName }}</p>
        <p>/</p>
        <p>default</p>
        <SortDownFill :size="14" />
      </div>
    </div>

    <div v-else class="flex items-center gap-5">
      <div class="flex items-center gap-2">
        <p></p>
        <SortDownFill :size="14" />
      </div>
    </div>
  </div>
</template>
