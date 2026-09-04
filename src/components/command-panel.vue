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
  name: "CommandPanel",
  components: {
    SortDownFill,
    StopCircle,
    Code,
    EllipsisH,
    Plus,
    Table,
    Database,
  },
  props: {
    onClose: {
      type: Function,
      default: () => {}
    },
    isConnected: {
      type: Boolean,
      default: false,
    },
    connectionName: {
      type: String,
      default: null
    }
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
      this.onClose();
    },
  },
});
</script>

<template>
  <div class="fixed">
    <div class="w-screen h-screen overflow-hidden bg-zinc-900 fixed top-0 left-0 opacity-75"/>
    
  <div class="flex flex-col mt-12 h-screen">
    <div
      class="bg-zinc-500 min-w-[400px] z-50 text-white"
    >
      <input autofocus class="flex items-center gap-2 w-full p-3 border-b border-zinc-300 outline-none" placeholder="Search commands..."  type="text"/>

      <button class="flex items-center gap-2 hover:bg-zinc-400 w-full p-3">
        <Plus :size="15" /> <span>Add new row</span>
      </button>
      <button class="flex items-center gap-2 hover:bg-zinc-400 w-full p-3">
        <Database :size="15" /> <span>Add new database</span>
      </button>
      <button class="flex items-center gap-2 hover:bg-zinc-400 w-full p-3">
        <Table :size="15" /> <span>Add new table</span>
      </button>
      <button
        @click="closeConnection"
        class="flex items-center gap-2 hover:bg-zinc-400 w-full p-3"
      >
        <StopCircle :size="15" /> <span>Close connection </span>
      </button>
    </div>
  </div>
  </div>
</template>
