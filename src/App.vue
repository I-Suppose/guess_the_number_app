<script setup lang="ts">
import { invoke } from '@tauri-apps/api';
import { ref } from 'vue';

var secret_number = Math.floor((Math.random() * 100) + 1);
var return_string = ref('');
var user_guess = ref(0);
function rice() {
  invoke('ev_number', {user69: user_guess.value, secretNumber:secret_number}).then((result) =>{
    console.log(result);
    return_string.value = result as string;
  });
  console.log('working');
}
</script>

<template>
  <div class="container">
    <input v-model="user_guess" type="number" id="guess" placeholder="entre a number">
    <button v-on:click="rice()" class="check">check number</button>
    <p>{{ return_string }}</p>
  </div>
</template>

<style scoped>
.check {
  margin-top: 10px;
}
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>