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
      isConnected: false,
      connectionName: null,
      results: [
  {
    id: 1,
    name: 'Alice',
    email: 'alice@example.com',
  },
  {
    id: 2,
    name: 'Bob',
    email: 'bob@example.com',
  },
  {
    id: 3,
    name: 'Charlie',
    email: 'charlie@example.com',
  },
  {
    id: 4,
    name: 'Puth',
    email: 'puth@example.com',
  },
]

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
  },
  unmounted() {
    //window.removeEventListener('db:connected')
    //window.removeEventListener('db:disconnected')
  },
  watch: {
  },
  computed: {
    columns() {
      if (!this.results.length) return []

      return Object.keys(this.results[0])
    }
  },
  methods: {
  },
});
</script>

<template>
   <div class="w-full flex items-start">
    <div class="fixed left-0 top-8 pt-1 bg-gray-200 w-[200px] min-h-screen flex flex-col">
        <b class="px-5 mb-2 mt-1">Tables</b>
        <button class="flex items-center gap-2 w-full cursor-pointer hover:bg-gray-300 h-8 px-5 bg-gray-300">
           <Table :size="14"/>
           <p>users</p>
        </button>        
        <button class="flex items-center gap-2 w-full cursor-pointer hover:bg-gray-300 h-8 px-5">
           <Table :size="14"/>
           <p>contracts</p>
        </button>        
        <button class="flex items-center gap-2 w-full cursor-pointer hover:bg-gray-300 h-8 px-5">
           <Table :size="14"/>
           <p>clients</p>
        </button>        
     </div>

    <div class="ml-[200px] pt-8 w-full">
      <table class="min-w-full border-collapse">
      <!-- Header -->
      <thead class="bg-gray-50 sticky top-8">
        <tr>
          <th
            v-for="column in columns"
            :key="column"
            class="border-b border-gray-200 px-4 py-2 text-left font-bold text-gray-600"
          >
            {{ column }}
          </th>
        </tr>
      </thead>

      <!-- Rows -->
      <tbody>
        <tr
          v-for="(row, rowIndex) in results"
          :key="rowIndex"
          class="hover:bg-gray-50"
        >
          <td
            v-for="column in columns"
            :key="column"
            class="border-b border-gray-100 px-4 py-2 text-gray-800"
          >
            {{ row[column] }}
          </td>
        </tr>
      </tbody>
    </table>
    </div>
    </div>
</template>

