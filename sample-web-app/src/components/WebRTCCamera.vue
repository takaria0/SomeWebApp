<template>
  <div class="root">
    <!-- *a5 -->
    <video
      v-if="captureStream"
      :srcObject.prop="captureStream"
      id="video"
      autoplay
      playsinline
      muted
    ></video>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref } from "vue";
export default defineComponent({
  // *a1
  name: "Camera",
  setup() {
    // *a2
    const captureStream = ref<MediaStream | undefined>();
    /* global MediaStreamConstraints */
    // 使用するデバイスの制約を定めるオブジェクト
    const constraints: MediaStreamConstraints = {
      audio: true,
      video: true,
    };
    // getUserMedia で MediaStream が取得できた際に実行する関数
    const gotStream = (stream: MediaStream) => {
      captureStream.value = stream;
    };
    // Promise の処理で reject された際に実行する関数
    const handleError = (error: Error) => {
      console.error(error.name, error.message);
    };
    // MediaStream の取得処理を実行
    navigator.mediaDevices
      .getUserMedia(constraints) // *a4
      .then(gotStream)
      .catch(handleError);
    return { captureStream }; // *a3
  },
});
</script>
