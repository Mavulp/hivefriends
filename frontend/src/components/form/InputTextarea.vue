<script setup lang="ts">
import { Error } from "../../js/error"

interface Props {
  label: string | undefined | null
  value: string | undefined | null
  type?: string
  error?: Error
}

const { label = "", value, type = "text", error } = defineProps<Props>()
const emit = defineEmits<{
  (e: "update:value", value: string): void
}>()

function updateValue(e: any) {
  emit("update:value", e.target.value)
}
</script>

<template>
  <div class="form-input" :class="{ 'input-error': error && error.invalid }">
    <label v-if="label">{{ label }}</label>
    <textarea v-bind="$attrs" tabindex="0" class="border-smoke font-14" :type="type" @input="updateValue" />
    <div class="input-error-list" v-if="error && error.invalid">
      <p v-for="item in error.errors">{{ item }}</p>
    </div>
  </div>
</template>
