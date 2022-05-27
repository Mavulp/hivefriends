<script setup lang="ts">
import { useMagicKeys, whenever } from "@vueuse/core"
import { onBeforeMount, onBeforeUnmount } from "vue"
// import { ref } from "vue"

const emit = defineEmits<{
  (e: "close"): void
}>()

const keys = useMagicKeys()
whenever(keys["Escape"], () => emit("close"))

const body = document.querySelector("body")

onBeforeMount(() => {
  if (body) body.style.overflowY = "hidden"
})

onBeforeUnmount(() => {
  if (body) body.style.overflowY = "unset"
})
</script>

<template>
  <div class="modal" @click.self="emit('close')">
    <slot />
  </div>
</template>
