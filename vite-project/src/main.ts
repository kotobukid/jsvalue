import {createApp} from 'vue'
import './style.css'
import App from './App.vue'
import init, {greet, flatten, get_person} from "/src/dist/jsv.js";

init().then(() => {
    // greet("WebAssembly");
    const aaaa = flatten();
    console.log(aaaa);
    const p = get_person();
    console.log(p);
    createApp(App).mount('#app')
});
