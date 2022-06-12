<script setup lang="ts">
import { computed } from "vue"

interface Props {
  label?: string
  check: boolean
}

const emit = defineEmits<{
  (e: "update:check", value: boolean): void
}>()

const props = defineProps<Props>()

const data = computed<boolean>({
  get() {
    return props.check
  },
  set(value) {
    emit("update:check", value)
  }
})

const d = computed(() => "id" + Math.random().toString(16).slice(2))
</script>

<template>
  <div class="form-checkbox">
    <input type="checkbox" :name="d" :id="d" v-model="data" />
    <label :for="d">
      <div class="icon">
        <span v-if="check" class="material-icons">&#xe834;</span>
        <span v-else class="material-icons">&#xe835;</span>
      </div>

      <p v-if="props.label">{{ props.label }}</p>
    </label>
  </div>
</template>
