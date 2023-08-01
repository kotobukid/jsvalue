import {App as _app, createApp} from 'vue'
import './style.css'
import App from './App.vue'
import init, {
    flatten,
    get_person,
    MyStruct,
    double_people,
    Point2D,
    PeopleFinder
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

import {ComponentCustomProperties} from 'vue';

interface PeopleFinderProperties {
    $peopleFinder: typeof PeopleFinder; // PeopleFinderの型
    $set_name_part: (name: string) => void;
    $set_age_gte: (age: number) => void;
    $set_age_lte: (age: number) => void;
    $find: () => { age: number, name: string }[];
}

declare module '@vue/runtime-core' {
    interface ComponentCustomProperties extends PeopleFinderProperties {
    }
}

init().then(() => {

    const point: Point2D = Point2D.new(100, 200);
    console.log(point.get_x());
    console.log(point.get_y());

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

    // const pf: PeopleFinder = PeopleFinder.new();
    // pf.set_name_part("o")
    // console.log(pf.apply());
    // pf.set_age_gte(22);
    // console.log(pf.apply());

    const peopleFinderPlugin = {
        install: (app: any, options: any) => {
            console.log({options})
            // PeopleFinderのインスタンスを作成
            const pf: PeopleFinder = PeopleFinder.new();

            // Vueアプリケーションにプロパティとして追加
            app.config.globalProperties.$peopleFinder = pf;

            // オプションで追加の設定があれば適用
            // if (options && options.namePart) {
            //     pf.set_name_part(options.namePart);
            // }

            app.config.globalProperties.$set_name = (name: string): void => {
                app.config.globalProperties.$peopleFinder.set_name_part(name);
            };
            app.config.globalProperties.$set_name = (name: string): void => {
                app.config.globalProperties.$peopleFinder.set_name_part(name);
            };
            app.config.globalProperties.$set_age_gte = (age: number): void => {
                app.config.globalProperties.$peopleFinder.set_age_gte(age);
            };
            app.config.globalProperties.$set_age_lte = (age: number): void => {
                app.config.globalProperties.$peopleFinder.set_age_lte(age);
            };
            app.config.globalProperties.$find = () => pf.apply();
        }
    };

    const app: _app<Element> = createApp(App);
    app.use(peopleFinderPlugin, {})
    app.mount('#app')
});
