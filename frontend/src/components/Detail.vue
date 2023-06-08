<script setup lang="ts">
import { isNil } from 'lodash'
import { computed, nextTick, ref, useSlots, watch } from 'vue'

interface Props {
  open?: boolean
  header?: string
  unstyle?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  open: undefined,
})
const emit = defineEmits<{
  (e: 'open', state: boolean): void
}>()
const slots = useSlots()

const contentMaxHeight = ref(0)
const content = ref()
// const parent = ref()

const open = ref(false)
const isOpen = computed(() => {
  /* ---------------- TODO ---------------- */
  // Figure out how to control this component form the outside

  if (!isNil(props.open))
    return props.open

  return open.value
})

function toggle() {
  open.value = !open.value
}

watch(isOpen, async (value) => {
  emit('open', value)

  if (value) {
    await nextTick()
    contentMaxHeight.value = content.value.scrollHeight
  }
})

watch(() => slots.content, () => {
  contentMaxHeight.value = content.value.scrollHeight
}, { deep: true })
</script>

<template>
  <div class="detail-wrap" :class="{ 'is-open': isOpen, 'unstyle': props.unstyle }">
    <slot v-if="slots.header" name="header" :open="open" :toggle="toggle" />

    <div ref="content" class="button-detail-content" :style="{ 'max-height': isOpen ? `${contentMaxHeight}px` : 0 }">
      <slot v-if="slots.content" name="content" :toggle="toggle" :open="open" />
      <slot v-else :toggle="toggle" :open="open" />
    </div>
  </div>
</template>
