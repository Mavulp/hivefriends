<script setup lang="ts">
import { onClickOutside } from "@vueuse/core"
import { sanitize } from "../../js/utils"
import { computed, ref, watch } from "vue"

type Option = {
  label: string
  value: any
}

interface Props {
  label?: string
  placeholder?: string
  multiple?: boolean
  options?: Array<Option | string> | null | undefined
  selected: Array<string> | string | null | undefined
  cantclear?: boolean
  required?: boolean
}

const { label, placeholder, multiple, options, selected, cantclear = false, required = false } = defineProps<Props>()
const open = ref(false)
const self = ref(null)
const search = ref("")

onClickOutside(self, () => {
  open.value = false
})

watch(open, (val) => {
  if (!val) {
    search.value = ""
  }
})

const emit = defineEmits<{
  (e: "update:selected", value: any): void
}>()

const formattedOptions = computed(() => {
  if (!options || options.length === 0) return null

  return options
    .map((item) => {
      if (typeof item === "string" || typeof item === "number") {
        return {
          label: item,
          value: item
        }
      } else {
        return item
      }
    })
    .filter((option) => option.label.toString().toLowerCase().includes(search.value.toLowerCase()))
})

const selectedLabels = computed(() => {
  if (!selected || selected.length === 0 || !formattedOptions.value) return null

  if (typeof selected === "string") {
    const item = formattedOptions.value.find((item) => item.value === selected)
    if (item) return item.label
  } else {
    return selected
      .map((select) => {
        const item = formattedOptions.value?.find((item) => item.value === select)
        if (item) return item.label
        return select
      })
      .join(", ")
  }
})

function setValue(item: Option) {
  // Multiple
  if (multiple && Array.isArray(selected)) {
    if (selected.find((sel) => sel === item.value)) {
      // Clearing
      if (cantclear && selected.length === 1) return

      const filtered = selected.filter((sel) => sel !== item.value)
      emit("update:selected", filtered)
    } else {
      // Setting
      emit("update:selected", [...selected, item.value])
    }
  } else {
    // Single

    if (selected && selected === item.value) {
      //Clearing
      if (cantclear) return

      emit("update:selected", null)
    } else {
      // Setting
      emit("update:selected", item.value)
    }

    // Only close if you can select just 1 item
    open.value = false
  }
}
</script>

<template>
  <div class="form-select" ref="self" :class="{ 'is-open': open, required: required }">
    <label v-if="label">{{ label }}</label>

    <button class="select-button" @click="open = !open">
      <input
        type="text"
        :placeholder="selected && selected.length > 0 ? `${selectedLabels}` : `${placeholder}`"
        v-model="search"
      />
    </button>

    <div class="select-dropdown">
      <template v-if="formattedOptions && formattedOptions.length > 0">
        <button
          v-for="item in formattedOptions"
          :key="item.label"
          :class="{ 'is-selected': selected && selected.includes(item.value) }"
          @click="setValue(item)"
        >
          <div v-html="sanitize(item.label)" />

          <template v-if="!cantclear">
            <span class="remove-item material-icons" v-if="selected && selected.includes(item.value)"> &#xe5cd; </span>
            <span class="add-item material-icons" v-else>&#xe145;</span>
          </template>
        </button>
      </template>
      <span class="select-no-options" v-else>Nothing to select.</span>
    </div>
  </div>
</template>
