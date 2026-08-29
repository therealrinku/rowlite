<script>
import { defineComponent } from "vue";
import { invoke, convertFileSrc } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import Landing from "./components/landing.vue";
import Header from "./components/header.vue";
import Connection from "./components/connection.vue";
import SortDownFill from '@primeicons/vue/sort-down-fill';
import Play from '@primeicons/vue/play';

export default defineComponent({
  name: "rowx",
  components: {
    Header,
    Landing,
    SortDownFill,
    Play,
    Connection
  },
  data() {
    return {
      isConnected: false,
      connectionName: null
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
  },
  methods: {
  },
});
</script>

<template>
  <main
    class="text-xs text-black min-h-screen min-w-screen text-sm tracking-wide flex flex-col items-center gap-5 w-full pb-10"
  >

     <Header/>
     <Connection v-if="isConnected"/>
     <Landing v-else/>
  </main>
</template>
