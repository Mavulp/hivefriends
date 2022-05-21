<script setup lang="ts">
import { onMounted } from "vue"
import { Toast, useToast } from "../../store/toast"

const { del } = useToast()

const { data, index } = defineProps<{
  data: Toast
  index: number
}>()

onMounted(() => {
  // Auto clear
  setTimeout(() => {
    del(index)
  }, data.timeout)
})
</script>

<template>
  <div class="hi-toast-item" :class="[data.type]">
    <p>{{ data.text }}</p>
    <button @click="del(index)">
      <span class="material-icons">&#xe5cd;</span>
    </button>
  </div>
</template>
