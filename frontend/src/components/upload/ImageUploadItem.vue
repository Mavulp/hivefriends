<script setup lang="ts">
import LoadingBar from "../loading/LoadingBar.vue"

import { onMounted, ref } from "vue"

interface Props {
  data: any
  index: number
}

const { data, index } = defineProps<Props>()

const emit = defineEmits<{
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
      <LoadingBar :class="[{ 'loading-done': !data.loading }, data.error ? 'loading-error' : 'loading-success']" />

      <button data-title-top="Remove Image" @click="emit('remove', index)">
        <span class="material-icons">&#xe5cd;</span>
      </button>
    </div>

    <div class="album-upload-content" v-if="open">
      <pre>
        {{ data }}
      </pre>
    </div>
  </div>
</template>
