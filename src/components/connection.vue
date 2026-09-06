<script>
import { defineComponent } from "vue";
import { invoke, convertFileSrc } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import SortDownFill from "@primeicons/vue/sort-down-fill";
import Play from "@primeicons/vue/play";
import Table from "@primeicons/vue/table";

export default defineComponent({
  name: "connection",
  components: {
    SortDownFill,
    Play,
    Table,
  },
  props: {
    isConnected: {
      type: Boolean,
      default: false,
    },
    connectionName: {
      type: String,
      default: null,
    },
  },
  data() {
    return {
      tables: [],
      selectedTable: null,
      tableData: {},
    };
  },
  async mounted() {
    const tables = await invoke("sqlite_get_tables");
    this.tables = tables;
  },
  unmounted() {
    //window.removeEventListener('db:connected')
    //window.removeEventListener('db:disconnected')
  },
  watch: {},
  computed: {
    tableRows() {
      if (!this.tableData[this.selectedTable]) return [];

      return this.tableData[this.selectedTable].rows;
    },
    columns() {
      if (!this.tableData[this.selectedTable]) return [];

      return ["#", ...this.tableData[this.selectedTable].columns];
    },
  },
  methods: {
    async fetchTable(tableName) {
      this.selectedTable = tableName;

      const resp = await invoke("sqlite_execute_query", {
        query: `select * from '${tableName}' limit 100`,
      });

      this.tableData[this.selectedTable] = {};
      this.tableData[this.selectedTable].rows = resp.rows;
      this.tableData[this.selectedTable].columns = resp.columns;
    },
  },
});
</script>

<template>
  <div class="w-full flex items-start">
    <div
      class="fixed left-0 top-0 pt-1 bg-zinc-700 w-[220px] min-h-screen flex flex-col overflow-y-auto"
    >
      <div
        class="flex items-center gap-2 px-2 py-1 mx-5 border border-zinc-400"
      >
        <p>{{ connectionName }}</p>
        <p>/</p>
        <p>default</p>
        <button><SortDownFill :size="14" /></button>
      </div>

      <b class="px-5 mb-2 mt-3">Tables</b>
      <button
        @click="fetchTable(table.name)"
        v-for="table in tables"
        class="flex items-center gap-2 w-full cursor-pointer hover:bg-zinc-600 h-8 px-5 text-ellipsis"
        :class="{ 'bg-zinc-600': selectedTable === table.name }"
      >
        <Table :size="14" />
        <p class="max-w-[85%] truncate">{{ table.name }}</p>
      </button>
    </div>

    <div class="ml-[220px] w-full overflow-x-auto overflow-y-auto max-h-screen">
      <table class="min-w-full border-collapse bg-inherit">
        <thead class="bg-inherit bg-zinc-800">
          <tr>
            <th
              v-for="column in columns"
              :key="column"
              class="border-b border-zinc-700 px-4 py-2 text-left font-bold max-w-[200px] sticky top-0 truncate z-0 bg-zinc-800"
            >
              {{ column }}
            </th>
          </tr>
        </thead>

        <tbody>
          <tr
            v-for="(row, rowIndex) in tableRows"
            :key="rowIndex"
            class="hover:bg-zinc-600"
          >
            <td
              v-for="column in columns"
              :title="row[column]"
              :key="column"
              class="border-b border-zinc-600 px-4 py-2 max-w-[200px] truncate"
            >
              <slot v-if="column === '#'">{{ rowIndex + 1 }}</slot>
              <slot else> {{ row[column] }}</slot>
            </td>
          </tr>
          <!-- <tr> -->
          <!-- <td -->
          <!-- v-for="(column, index) in columns" -->
          <!-- :contenteditable="index >=1 ? true : false" -->
          <!-- class="border border-zinc-200 px-4 py-2 max-w-[200px] truncate border outline-none" -->
          <!-- > -->
          <!-- <slot v-if="index===0">{{ tableRows.length + 1 }}</slot> -->
          <!-- </td> -->
          <!-- </tr> -->
        </tbody>
      </table>
    </div>
  </div>
</template>
