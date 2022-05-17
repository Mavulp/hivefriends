<script setup lang="ts">
import { onClickOutside } from "@vueuse/core"
import { computed, ref } from "vue"

// TODO: implement multiple

type Option = {
  label: string
  value: any
}

interface Props {
  label?: string
  placeholder?: string
  multiple?: boolean
  options: Array<Option | string>
  selected: Array<string> | string
  canclear?: boolean
}

const { label, placeholder, multiple, options, selected, canclear } = defineProps<Props>()
const open = ref(false)
const self = ref(null)

onClickOutside(self, () => {
  open.value = false
})

const emit = defineEmits<{
  (e: "update:selected", value: any): void
}>()

const formattedOptions = computed(() => {
  return options.map((item) => {
    if (typeof item === "string") {
      return {
        label: item,
        value: item
      }
    } else {
      return item
    }
  })
})

const selectedLabels = computed(() => {
  if (!selected || selected.length === 0) return null

  if (typeof selected === "string") {
    const item = formattedOptions.value.find((item) => item.value === selected)
    if (item) return item.label
  } else {
    return selected.map((select) => {
      return formattedOptions.value.find((item) => item.value === select)?.label ?? select
    })
  }
})

function setValue(item: Option) {
  // TODO: multiple

  if (selected === item.value && canclear) {
    emit("update:selected", null)
  } else {
    emit("update:selected", item.value)
  }

  open.value = false
}
</script>

<template>
  <div class="form-select" ref="self" :class="{ 'is-open': open }">
    <label v-if="label">{{ label }}</label>

    <button class="select-button" @click="open = !open">
      <template v-if="selected">
        {{ selectedLabels }}
      </template>
      <span class="select-placeholder" v-else-if="placeholder">{{ placeholder }}</span>
    </button>

    <div class="select-dropdown">
      <button
        v-for="item in formattedOptions"
        :key="item.label"
        :class="{ 'is-selected': selected.includes(item.value) }"
        @click="setValue(item)"
      >
        <div v-html="item.label" />
      </button>
    </div>
  </div>
</template>
