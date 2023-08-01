<script setup lang="ts">
import {onMounted, ref} from 'vue'
import {getCurrentInstance} from "vue";

const instance = getCurrentInstance();
const globalProperties = instance && instance.appContext && instance.appContext.config ? instance.appContext.config.globalProperties : undefined;

defineProps<{ msg: string }>()

const count = ref(0)

const name_part = ref("");

const increment = () => {
    count.value = count.value + 1;
}

type Person = {
    age: number,
    name: string
}

const people = ref<Person[]>([]);

onMounted(() => {
    if (globalProperties) {
        globalProperties.$set_name("");
        people.value = globalProperties.$find();
    }
});

const apply_find = () => {
    if (globalProperties) {
        globalProperties.$set_name(name_part.value);
        people.value = globalProperties.$find();
    }
}
</script>

<template>
<h1>{{ msg }}</h1>

<div class="card">
    <label>
        <span>NAME: </span>
        <input type="text" v-model="name_part">
    </label>
    <br>
    <a href="#" @click.prevent="apply_find">絞り込み</a>
    <ul>
        <li v-for="p in people" :key="p.name">{{ p.name }} ({{ p.age }})</li>
    </ul>

    <hr>

    <button type="button" @click="increment">count is {{ count }}</button>
    <p>
        Edit
        <code>components/HelloWorld.vue</code> to test HMR
    </p>
</div>
</template>

<style scoped>
.read-the-docs {
    color: #888;
}
</style>
