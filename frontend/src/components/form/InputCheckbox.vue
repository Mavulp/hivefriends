<script setup lang="ts">
import { computed } from "vue"

interface Props {
  label: string
  check: boolean
}

// Unique id for the checkbox
const d = `${new Date().getTime()}`

const emit = defineEmits<{
  (e: "update:check", value: boolean): void
}>()

const { check, label } = defineProps<Props>()

const data = computed<boolean>({
  get() {
    return check
  },
  set(value) {
    emit("update:check", value)
  }
})
</script>

<template>
  <div class="form-checkbox">
    <input type="checkbox" :name="d" :id="d" v-model="data" />
    <label :for="d">
      <div class="icon">
        <span v-if="check" class="material-icons">&#xe834;</span>
        <span v-else class="material-icons">&#xe835;</span>
      </div>

      <p>{{ label }}</p>
    </label>
  </div>
</template>
