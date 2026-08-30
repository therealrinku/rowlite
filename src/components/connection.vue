<script>
import { defineComponent } from "vue";
import { invoke, convertFileSrc } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import SortDownFill from '@primeicons/vue/sort-down-fill';
import Play from '@primeicons/vue/play';
import Table from '@primeicons/vue/table';

export default defineComponent({
  name: "connection",
  components: {
    SortDownFill,
    Play,
    Table
  },
  data() {
    return {
      tables: [],
      isConnected: false,
      connectionName: null,
      selectedTable: null,
      tableData: {}
    };
  },
  async mounted() {
     window.addEventListener('db:connected', (event) => {
        this.connectionName = event.detail.connectionName;
        this.isConnected = true;
     });
     window.addEventListener('db:disconnected', (event) => {
        this.connectionName = null;
        this.isConnected = false;
     });

     const tables = await invoke("sqlite_get_tables");
     this.tables = tables;
  },
  unmounted() {
    //window.removeEventListener('db:connected')
    //window.removeEventListener('db:disconnected')
  },
  watch: {
  },
  computed: {
    tableRows() {
      if (!this.tableData[this.selectedTable]) return []

      return this.tableData[this.selectedTable].rows;
    },
    columns() {
      if (!this.tableData[this.selectedTable]) return []

      return this.tableData[this.selectedTable].columns
    }
  },
  methods: {
    async fetchTable(tableName) {
      this.selectedTable = tableName;

      const resp = await invoke("sqlite_execute_query", { query: `select * from ${tableName} limit 100` })

       this.tableData[this.selectedTable] = {};
       this.tableData[this.selectedTable].rows = resp.rows;
       this.tableData[this.selectedTable].columns = resp.columns;
     }
  },
});
</script>

<template>
   <div class="w-full flex items-start">
    <div class="fixed left-0 top-8 pt-1 bg-gray-100 w-[220px] min-h-screen flex flex-col overflow-y-auto">
        <b class="px-5 mb-2 mt-1">Tables</b>
        <button @click="fetchTable(table.name)" v-for="table in tables" class="flex items-center gap-2 w-full cursor-pointer hover:bg-gray-300 h-8 px-5 text-ellipsis" :class="{ 'bg-gray-200': selectedTable === table.name }" >
           <Table :size="14"/>
           <p class="max-w-[85%] truncate">{{ table.name }}</p>
        </button>
     </div>

    <div class="ml-[220px] w-full overflow-x-auto overflow-y-auto max-h-screen">
      <table class="min-w-full border-collapse">

      <thead class="bg-gray-50 ">
        <tr>
          <th
            v-for="column in columns"
            :key="column"
            class="border-b border-gray-200 px-4 py-2 text-left font-bold text-gray-600 max-w-[200px] truncate sticky top-8 bg-gray-50"
          >
            {{ column }}
          </th>
        </tr>
      </thead>


      <tbody>
        <tr
          v-for="(row, rowIndex) in tableRows"
          :key="rowIndex"
          class="hover:bg-gray-50"
        >
          <td
            v-for="column in columns"
            :key="column"
            class="border-b border-gray-100 px-4 py-2 text-gray-800 max-w-[200px] truncate"
          >
            {{ row[column] }}
          </td>
        </tr>
      </tbody>
    </table>
    </div>
    </div>
</template>
