<script setup lang="ts">
import { onClickOutside } from '@vueuse/core'
import { computed, ref, watch } from 'vue'
import { sanitize } from '../../js/utils'

interface Option {
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

const props = withDefaults(defineProps<Props>(), {
  required: false,
})
const emit = defineEmits<{
  (e: 'update:selected', value: any): void
}>()
const open = ref(false)
const self = ref(null)
const search = ref('')

onClickOutside(self, () => {
  open.value = false
})

watch(open, (val) => {
  if (!val)
    search.value = ''
})

const formattedOptions = computed(() => {
  if (!props.options || props.options.length === 0)
    return null

  return props.options
    .map((item) => {
      if (typeof item === 'string' || typeof item === 'number') {
        return {
          label: item,
          value: item,
        }
      }
      else {
        return item
      }
    })
    .filter(option => option.label.toString().toLowerCase().includes(search.value.toLowerCase()))
})

const selectedLabels = computed(() => {
  if (!props.selected || props.selected.length === 0 || !formattedOptions.value)
    return null

  if (typeof props.selected === 'string') {
    const item = formattedOptions.value.find(item => item.value === props.selected)
    // if (item)
    return item?.label ?? null
  }
  else {
    return props.selected
      .map((select) => {
        const item = formattedOptions.value?.find(item => item.value === select)
        if (item)
          return item.label
        return select
      })
      .join(', ')
  }
})

function setValue(item: Option) {
  // Multiple
  if (props.multiple && Array.isArray(props.selected)) {
    if (props.selected.find(sel => sel === item.value)) {
      // Clearing
      if (props.cantclear && props.selected.length === 1)
        return

      const filtered = props.selected.filter(sel => sel !== item.value)
      emit('update:selected', filtered)
    }
    else {
      // Setting
      emit('update:selected', [...props.selected, item.value])
    }
  }
  else {
    // Single

    if (props.selected && props.selected === item.value) {
      // Clearing
      if (props.cantclear)
        return

      emit('update:selected', null)
    }
    else {
      // Setting
      emit('update:selected', item.value)
    }

    // Only close if you can select just 1 item
    open.value = false
  }
}
</script>

<template>
  <div ref="self" class="form-select" :class="{ 'is-open': open, 'required': required }">
    <label v-if="label">{{ label }}</label>

    <button class="select-button" @click="open = !open">
      <input
        v-model="search"
        type="text"
        :placeholder="selected && selected.length > 0 ? `${selectedLabels}` : `${placeholder}`"
      >

      <div class="select-icon">
        <span class="material-icons">
          &#xe5cf;
        </span>
      </div>
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
            <span v-if="selected && selected.includes(item.value)" class="remove-item material-icons"> &#xe5cd; </span>
            <span v-else class="add-item material-icons">&#xe145;</span>
          </template>
        </button>
      </template>
      <span v-else class="select-no-options">Nothing to select.</span>
    </div>
  </div>
</template>
