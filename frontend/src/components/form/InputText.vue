<script setup lang="ts">
// import { ref } from "vue"

interface Props {
  label: string | undefined | null
  value: string | undefined | null
  type?: string
}

const { label = "", value, type = "text" } = defineProps<Props>()
const emit = defineEmits<{
  (e: "update:value", value: string): void
}>()

function updateValue(e: any) {
  emit("update:value", e.target.value)
}
</script>

<template>
  <div class="form-input">
    <label v-if="label">{{ label }}</label>
    <input v-bind="$attrs" tabindex="0" class="border-smoke font-14" :type="type" @input="updateValue" />
  </div>
</template>
