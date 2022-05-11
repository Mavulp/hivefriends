<script setup lang="ts">
import LoadingBar from "../loading/LoadingBar.vue"

import { onMounted, ref } from "vue"

interface Props {
  data: any
}

const { data } = defineProps<Props>()

onMounted(() => {
  console.log(data)
})

const emit = defineEmits<{
  (e: "key", id: string): void
  (e: "remove", index: number): void
}>()

const open = ref(false)
// const loading = ref(true)

const size = () => {
  if (data.size / 1000000 > 1) return data.size / 1000000 + "mb"
  return data.size / 1000 + "kb"
}
</script>

<template>
  <div class="album-upload-item" :class="{ open: open }">
    <div class="album-upload-item-header" @click.self="open = !open">
      <strong>{{ data.name }}</strong>

      <span class="file-size">Size: {{ size() }}</span>

      <div class="flex-1"></div>
      <LoadingBar :class="{ 'progress-done': !data.loading }" />

      <button data-title-top="Remove Image">
        <span class="material-icons">&#xe5cd;</span>
      </button>
    </div>

    <div class="album-upload-content" v-if="open">djhfajdsfd;as</div>
  </div>
</template>
