import {createApp} from 'vue'
import './style.css'
import App from './App.vue'
import init, {
    // greet,
    flatten, get_person, get_people,
    // call_js_callback,
    MyStruct,
    double_people
} from "/src/pkg/jsv";

type Person = {
    age: number,
    name: string
};
type People = {
    people: Person[]
};

declare module "/src/pkg/jsv" {
  export function double_people(js_people: People): People;
}

init().then(() => {
    // greet("WebAssembly");

    const callback1 = (people: People): void => {
        console.log(double_people(people));
    };
    const _people: People = {
        people: [{age: 30, name: "Jane1"},
            {age: 31, name: "Jane2"},
            {age: 32, name: "Jane3"}]
    };
    const ms = new MyStruct();
    ms.init({callback1});
    console.log(double_people.toString());
    console.log(ms.call_callback(_people))
    const dp = double_people(_people);
    console.log(dp);

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
