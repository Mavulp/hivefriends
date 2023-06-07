<script setup lang="ts">
import { useMediaQuery } from '@vueuse/core'
import { computed, useAttrs } from 'vue'

const { size, pad, to, color, center } = defineProps<Props>()

const attrs = useAttrs()

interface Props {
  size?: string | undefined
  pad?: string | undefined
  to?: string | object | undefined
  color?: string | undefined
  center?: boolean
}

const isPhone = useMediaQuery('(max-width: 512px)')

const style = computed(() => ({
  height: size || (isPhone.value ? '36px' : '44px'),
  lineHeight: size || (isPhone.value ? '36px' : '44px'),
  paddingLeft: pad || (isPhone.value ? '16px' : '24px'),
  paddingRight: pad || (isPhone.value ? '16px' : '24px'),
  // alignText: 'center',
  ...(color && { color }),
  ...(center && { alignText: 'center' }),
}))
</script>

<template>
  <button v-if="!to" class="button" :style="style" v-bind="attrs">
    <slot />
  </button>

  <a v-else-if="to && typeof to === 'string'" class="button" :style="style" v-bind="attrs" :href="to">
    <slot />
  </a>

  <router-link v-else-if="to" class="button" :style="style" v-bind="attrs" :to="to">
    <slot />
  </router-link>
</template>
