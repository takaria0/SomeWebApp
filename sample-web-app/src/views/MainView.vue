<script setup lang="ts">
import { reactive, ref } from "vue";
import Camera from "@/components/WebRTCCamera.vue";

interface Book {
  title: string;
  year?: number;
}

const book: Book = reactive({ title: "Vue 3 Guide" });
</script>

<template>
  <div class="main">
    <h1>This is an main page</h1>

    <input type="text" v-model="book.title" />
    <p>{{ book.title }}</p>

    <Camera></Camera>
  </div>
</template>

<style>
@media (min-width: 1024px) {
  .about {
    min-height: 100vh;
    display: flex;
    align-items: center;
  }
}
</style>
