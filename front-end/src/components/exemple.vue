<script setup>
    import {ref,computed} from "vue";

    const compteur = ref(0);
    const txt = ref("");
    const valeur = computed(()=> {
        return compteur.value*2;
    })
</script>

<template>
    <div>{{ valeur }}</div>
    <button @click = "()=>{compteur++}">{{compteur}}</button> 

    <div>{{txt}}</div>
    <input type="text" v-model="txt"/>
</template>

<style scoped>

</style>