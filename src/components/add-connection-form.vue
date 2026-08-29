<script>
import { defineComponent } from "vue";
import { invoke, convertFileSrc } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import Play from '@primeicons/vue/play';
import SortDownFill from '@primeicons/vue/sort-down-fill';

export default defineComponent({
  name: "AddConnectionForm",
  components: {
    Play,
    SortDownFill
  },
  props:{
    onSave: {
      type: Function,
      required: false,
    },
    showCancel: {
      type: Function,
      required: false,
    },
    onCancel: {
      type: Function,
      required: false
    }
  },
  data() {
    return {
      type: "postgres",
      name: "",
      host: '',
      port: 80,
      user: '',
      password: '',
      filePath: ''
    }
  },
  async mounted() {
    const i = document.getElementById("add-connection-form")
    if(i) {
      i.scrollTop = 10;
    }
  },
  unmounted() {},
  watch: {
  },
  computed: {
  },
  methods: {
    handleSave(){
      const connsJSON = localStorage.getItem("connections");
      const conns = connsJSON && Array.isArray(JSON.parse(connsJSON)) ? JSON.parse(connsJSON) : []
      if(this.type==="sqlite"){
        conns.push({type:"sqlite",  name: this.name, filePath: this.filePath})
      } else {
        conns.push({ type:this.type, name: this.name, host: this.host, port: this.port, user: this.user, password: this.password })
      }
      localStorage.setItem("connections", JSON.stringify(conns));

      this.onSave();
      onCancel();
    },
    onFileChange(e) {
      console.log(e.target.files[0].path)
    }
  },
});
</script>

<template>
     <div id="add-connection-form" class="flex flex-col gap-5 w-[400px] border-gray-100 border p-5">
        <b>Add new connection</b>
        <div class="flex flex-col gap-2">
          <p class="opacity-50">Type</p>
          <div class="flex items-center gap-2">
            <button @click="type='postgres'" :class="{'bg-gray-100': type === 'postgres'}" class="border px-5 h-8 border-gray-100 cursor-pointer hover:bg-gray-100">PostgreSQL</button>
            <button @click="type='mysql'" :class="{'bg-gray-100': type === 'mysql'}" class="border px-5 h-8 border-gray-100 cursor-pointer hover:bg-gray-100">MySQL</button>
            <button @click="type='sqlite'" :class="{'bg-gray-100': type === 'sqlite'}" class="border px-5 h-8 border-gray-100 cursor-pointer hover:bg-gray-100">SQLite</button>
          </div>
        </div>

        <slot v-if="type!=='sqlite'">
        <div class="flex flex-col gap-2">
          <p class=" opacity-50">Connection name</p>
          <input v-model="name" placeholder="my-postgres" type="text" class="bg-gray-100 text-black px-2 py-2 focus:outline-[1.2px] outline-green-500"/>
        </div>
        <div class="flex flex-col gap-2">
          <p class=" opacity-50">Host</p>
          <input v-model="host" placeholder="dkdweid.di.com" type="text" class="bg-gray-100 text-black px-2 py-2 focus:outline-[1.2px] outline-green-500"/>
        </div>
        <div class="flex flex-col gap-2">
          <p class="opacity-50">Port</p>
          <input v-model="port" placeholder="40" type="text" class="bg-gray-100 text-black px-2 py-2 focus:outline-[1.2px] outline-green-500"/>
        </div>
        <div class="flex flex-col gap-2">
          <p class="opacity-50">User</p>
          <input v-model="user" placeholder="root" type="text" class="bg-gray-100 text-black px-2 py-2 focus:outline-[1.2px] outline-green-500"/>
        </div>
        <div class="flex flex-col gap-2">
          <p class="opacity-50">Password</p>
          <input v-model="password" placeholder="****" type="password" class="bg-gray-100 text-black px-2 py-2 focus:outline-[1.2px] outline-green-500"/>
        </div>
        </slot>

        <slot v-else>
        <div class="flex flex-col gap-2">
          <p class=" opacity-50">Connection name</p>
          <input v-model="name" placeholder="my-sqlite" type="text" class="bg-gray-100 text-black px-2 py-2 focus:outline-[1.2px] outline-green-500"/>
        </div>
        <div class="flex flex-col gap-2">
          <p class="opacity-50">File Path</p>
          <input type="text" v-model="filePath" class="bg-gray-100 text-black px-2 py-2 focus:outline-[1.2px] outline-green-500"/>
        </div>
        </slot>

        <div class="flex items-center">
          <button v-if="showCancel" @click="onCancel" class="h-8 cursor-pointer bg-gray-200 w-[30%] flex justify-center items-center gap-3">Cancel <span class="text-gray-400">Esc</span> </button>
          <!-- <button class="h-8 cursor-pointer ml-auto bg-gray-200 w-[20%]">Test</button> -->
          <button @click="handleSave" class="ml-auto h-8 cursor-pointer flex justify-center items-center gap-2 text-white bg-green-500 w-[30%] font-bold">Save</button>
          <!-- <button class="h-8 bg-gray-100 px-2 flex items-center"><SortDownFill :size="14"/></button> -->
        </div>
        </div>
</template>
