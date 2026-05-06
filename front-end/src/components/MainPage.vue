<script setup>
import { ref } from 'vue'

</script>

<template>
<div class="bg">
    <div>
        <div class="logo"></div>

        <div class="Form">
            <div class="Form-Button"><RouterLink to="/login">Connexion</RouterLink></div>
            <div class="Form-Button"><RouterLink to="/register">Inscription</RouterLink></div>
        </div>
    </div>
</div>
</template>

<style scoped>

.logo{
    width: 100%;
    height: 150px;
    background-position: center;
    background-size: contain;
    background-repeat: no-repeat;
    background-image: url("/public/MINESTONK-06-05-2026.png");
    margin-bottom: 5%;
    margin-top: 10%;
    image-rendering: pixelated;
}

</style>