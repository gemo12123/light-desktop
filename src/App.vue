<template>

  <MyMain/>
</template>

<script>
import MyMain from './components/MyMain.vue';

const {invoke} = window.__TAURI__.tauri;

export default {
  name: 'App',
  components: {
    MyMain
  }
}

import {appWindow} from "@tauri-apps/api/window";

appWindow.onFileDropEvent((event) => {
  if (event.payload.type === 'drop') {
    let path = event.payload.paths;
    invoke('app_add_event', {item: "default", filepath: path[0]})
        .then((rep) => {
          if (rep.length <= 1) {
            console.log("成功")
          } else {
            console.log('错误信息: ' + rep)
          }
        })
  }
});
</script>

<style>
</style>
