import {createApp} from 'vue'
import './style.css'
import App from './App.vue'
import init, {greet} from "/src/dist/jsv.js";

init().then(() => {
    greet("WebAssembly");
    createApp(App).mount('#app')
});
