<script setup lang="ts">
import { computed, useAttrs } from "vue"

const attrs = useAttrs()

interface Props {
  size?: string | undefined
  pad?: string | undefined
  to?: string | object | undefined
  color?: string | undefined
}

const { size = "44px", pad = "32px", to, color } = defineProps<Props>()

const style = computed(() => ({
  height: size,
  lineHeight: size,
  paddingLeft: pad,
  paddingRight: pad,
  ...(color && { color })
}))
</script>

<template>
  <button class="button" :style="style" v-bind="attrs" v-if="!to">
    <slot />
  </button>

  <a class="button" :style="style" v-bind="attrs" :href="to" v-else-if="to && typeof to === 'string'">
    <slot />
  </a>

  <router-link class="button" :style="style" v-bind="attrs" v-else-if="to" :to="to">
    <slot />
  </router-link>
</template>
