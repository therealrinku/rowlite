<script>
import { defineComponent } from "vue";
import { invoke, convertFileSrc } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import SortDownFill from "@primeicons/vue/sort-down-fill";
import Code from "@primeicons/vue/code";
import StopCircle from "@primeicons/vue/stop-circle";
import EllipsisH from "@primeicons/vue/ellipsis-h";
import Plus from "@primeicons/vue/plus";
import Table from "@primeicons/vue/table";
import Database from "@primeicons/vue/database";

export default defineComponent({
  name: "ConnectionActions",
  components: {
    SortDownFill,
    StopCircle,
    Code,
    EllipsisH,
    Plus,
    Table,
    Database,
  },
  data() {
    return {
      showDropdown: false,
    };
  },
  async mounted() {},
  unmounted() {},
  watch: {},
  computed: {},
  methods: {
    async closeConnection() {
      await invoke("sqlite_disconnect");
      window.dispatchEvent(new CustomEvent("db:disconnected"));
      this.showDropdown = false;
    },
  },
});
</script>

<template>
  <div>
    <button
      class="flex items-center gap-2 relative"
      @click="showDropdown = !showDropdown"
    >
      <EllipsisH :size="14" />
    </button>

    <div
      v-if="showDropdown"
      class="absolute top-9 right-2 bg-zinc-600 min-w-[200px] z-50"
    >
      <button class="flex items-center gap-2 hover:bg-zinc-500 w-full p-3">
        <Plus :size="15" /> <span>Add new row</span>
      </button>
      <button class="flex items-center gap-2 hover:bg-zinc-500 w-full p-3">
        <Database :size="15" /> <span>Add new database</span>
      </button>
      <button class="flex items-center gap-2 hover:bg-zinc-500 w-full p-3">
        <Table :size="15" /> <span>Add new table</span>
      </button>
      <button
        @click="closeConnection"
        class="flex items-center gap-2 hover:bg-zinc-500 w-full p-3"
      >
        <StopCircle :size="15" /> <span>Close connection </span>
      </button>
    </div>
  </div>
</template>
