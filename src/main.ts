import { createApp } from "vue";
import { createPinia } from "pinia";
import App from "./App.vue";
import { initImages } from "./lib/images";

async function start() {
  await initImages();
  createApp(App).use(createPinia()).mount("#app");
}

start();