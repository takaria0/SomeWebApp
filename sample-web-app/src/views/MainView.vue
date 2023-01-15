<script setup lang="ts">
import { reactive, ref, Ref, onMounted } from "vue";
// Cannot use in insecure environment
// import Camera from "@/components/WebRTCCamera.vue";

interface Response {
  title: string;
  content: string;
}

let result: Ref<string> = ref("");
const exampleResponse: Response = reactive({
  title: "",
  content: "",
});

const getData = () => {
  fetch("http://157.7.195.232/api/v1/index", {})
    .then((res) => res.json())
    .then((response) => {
      console.log({ response });
      exampleResponse.content = response.name;
    })
    .catch((error) => {
      console.log("Looks like there was a problem: \n", error);
    });
};
const postData = () => {
  console.log("calling...", result);
  const url = "http://157.7.195.232/api/v1/index";
  const response = fetch(url, {
    method: "POST", // *GET, POST, PUT, DELETE, etc.
    mode: "cors", // no-cors, *cors, same-origin
    cache: "no-cache", // *default, no-cache, reload, force-cache, only-if-cached
    credentials: "same-origin", // include, *same-origin, omit
    headers: {
      "Content-Type": "application/json",
      // 'Content-Type': 'application/x-www-form-urlencoded',
    },
    redirect: "follow", // manual, *follow, error
    referrerPolicy: "no-referrer", // no-referrer, *no-referrer-when-downgrade, origin, origin-when-cross-origin, same-origin, strict-origin, strict-origin-when-cross-origin, unsafe-url
    body: JSON.stringify({ username: exampleResponse.title }), // body data type must match "Content-Type" header
  })
    .then((res) => res.json())
    .then((res) => {
      console.log("finished...", res);
      result.value = res.name;
      console.log("finished...", result);
    });
  return; // parses JSON response into native JavaScript objects
};

onMounted(() => {
  console.log(`the component is now mounted.`);
  getData();
});
</script>

<template>
  <div class="main">
    <h1>Vue 3 & Rust on VPS with Nginx</h1>

    <p>
      Type something here and press Enter, you would see the response from a
      Rust server!
    </p>
    <input
      type="text"
      size="20"
      v-model="exampleResponse.title"
      @change="postData"
    />
    <p>
      <small style="color: gray">{{ exampleResponse.title }}</small>
    </p>
    <p>{{ exampleResponse.content }}</p>
    <h2>{{ result }}</h2>

    <!-- <Camera></Camera> -->
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
