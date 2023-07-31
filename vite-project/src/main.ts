import {createApp} from 'vue'
import './style.css'
import App from './App.vue'
import init, {greet, flatten} from "/src/dist/jsv.js";

init().then(() => {
    // greet("WebAssembly");
    const aaaa = flatten();
    console.log(aaaa);
    createApp(App).mount('#app')
});
