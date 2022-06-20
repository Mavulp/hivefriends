<script setup lang="ts">
import { useMediaQuery } from "@vueuse/core"
import { computed, useAttrs } from "vue"

const attrs = useAttrs()

interface Props {
  size?: string | undefined
  pad?: string | undefined
  to?: string | object | undefined
  color?: string | undefined
  center?: boolean
}

const isPhone = useMediaQuery("(max-width: 512px)")

const { size, pad, to, color, center } = defineProps<Props>()

console.log(size ? size : isPhone.value ? "36px" : "44px")

const style = computed(() => ({
  height: size ? size : isPhone.value ? "36px" : "44px",
  lineHeight: size ? size : isPhone.value ? "36px" : "44px",
  paddingLeft: pad ? pad : isPhone.value ? "16px" : "32px",
  paddingRight: pad ? pad : isPhone.value ? "16px" : "32px",
  // alignText: 'center',
  ...(color && { color }),
  ...(center && { alignText: "center" })
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
