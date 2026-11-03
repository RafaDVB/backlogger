import "./styles/base.css";
import "./styles/vars.css";
import { createApp } from "vue";
import { createPinia } from "pinia";
import App from "./App.vue";
import { initImages } from "./lib/images";
import { useViewStore } from "./stores/view";
import { useAppearanceStore } from "./stores/appearance.ts";

async function start() {
  await initImages();
  const pinia = createPinia();
  const app = createApp(App).use(pinia);
  await useAppearanceStore(pinia).load();
  await useViewStore(pinia).load();
  app.mount("#app");
}

start();