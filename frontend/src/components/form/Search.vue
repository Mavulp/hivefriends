<script setup lang="ts">
import { ref } from "vue"

const input = ref(null)

interface Props {
  value: string | undefined | null
}

const { value } = defineProps<Props>()
const emit = defineEmits<{
  (e: "update:value", value: string): void
}>()

function updateValue(e: any) {
  emit("update:value", e.target.value)
}

function clear() {
  emit("update:value", "")

  if (input.value) {
    //@ts-ignore
    input.value.value = ""
  }
}
</script>

<template>
  <div class="form-search" :class="{ 'has-input': value }">
    <span class="material-icons">&#xe8b6;</span>
    <input v-bind="$attrs" tabindex="0" type="text" @input="updateValue" ref="input" />
    <button v-if="value" @click="clear">
      <span class="material-icons">&#xe5cd;</span>
    </button>
  </div>
</template>
