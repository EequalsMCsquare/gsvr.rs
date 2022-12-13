import { createApp } from "vue";
import "./style.css";
import App from "./App.vue";
import route from "./route";

const app = createApp(App);
app.use(route);
app.mount("#app");
