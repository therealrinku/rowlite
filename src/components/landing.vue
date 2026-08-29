<script>
import { defineComponent } from "vue";
import { invoke, convertFileSrc } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import AddConnectionForm from "./add-connection-form.vue";
import ListTree from '@primeicons/vue/list-tree';
import Database from '@primeicons/vue/database';
import Ban from '@primeicons/vue/ban';
import Bolt from '@primeicons/vue/bolt';

export default defineComponent({
  name: "LandingPage",
  components: {
     AddConnectionForm,
     ListTree,
    Database,
    Ban,
    Bolt
  },
  data() {
    return {
      connections: [],
      showAddForm: false
    };
  },
  async mounted() {
    this.fetchConnections();

    if(this.connections.length === 0) {
      this.showAddForm = true;
    }
  },
  unmounted() {},
  watch: {
  },
  computed: {
  },
  methods: {
    onConnect(connectionName) {
      const connectEvent = new CustomEvent('db:connected', {
      detail: {
        connectionName,
      }
    });

     window.dispatchEvent(connectEvent);
    },
    fetchConnections() {
      const connsJSON = localStorage.getItem("connections");
      const conns = connsJSON && Array.isArray(JSON.parse(connsJSON)) ? JSON.parse(connsJSON) : []
      this.connections = conns;
    },
    onSave() {
      this.fetchConnections();
    },
    onCancelAdd() {
      this.showAddForm = false;
    }
  },
});
</script>

<template>
   <div class="flex flex-col gap-5 w-[400px] mt-20">
     <div v-if="connections.length > 0" class="flex flex-col gap-5">
      <b>Saved connections - {{ connections.length }}</b>
      <button @click="onConnect(connection.name)" v-for="connection in connections" class="border border-gray-100 p-2 flex items-center gap-5 hover:bg-gray-100 cursor-pointer">
        <Database/>
        <p>{{ connection.name }}</p>
      </button>
      <button v-if="!showAddForm" @click="showAddForm=true" class="self-end w-[30%] p-2 bg-green-500 text-white cursor-pointer">New connection</button>
      </div>

      <div v-else class="flex flex-col gap-5 border p-5 border-gray-100">
        <b class="flex items-center gap-2"> <Ban :size="16"/> No saved connections found.</b>
      </div>

     <AddConnectionForm v-if="showAddForm" :showCancel="connections.length>0" :onCancel="onCancelAdd" :onSave="onSave"/>
   </div>
</template>

