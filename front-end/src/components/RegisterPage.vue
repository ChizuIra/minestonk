<script setup>
import { ref } from 'vue'

</script>

<template>
<div class="bg">
    <div>
        <div class="logo"></div>
        <div class="Form">
            <ul>    
                <li>
                    <h1>inscription</h1>
                </li>
                <li>
                    <label for="name">nom :</label>
                    <input type="text" id="name"/>
                </li>
                <li>
                    <label for="password">Mot de passe :</label>
                    <input type="text" id="password"/>
                </li>
                <div class="Form-Button"> Valider </div>
            </ul>
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