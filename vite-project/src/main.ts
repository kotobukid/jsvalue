import {createApp} from 'vue'
import './style.css'
import App from './App.vue'
import init, {
    // greet,
    flatten, get_person, get_people} from "/src/pkg/jsv";

init().then(() => {
    // greet("WebAssembly");

    const aaaa = flatten();
    console.log(aaaa);

    const p = get_person();
    console.log(p);

    const {people} = get_people();
    console.log(people)

    // @ts-ignore
    window.get_people = get_people;

    createApp(App).mount('#app')
});
