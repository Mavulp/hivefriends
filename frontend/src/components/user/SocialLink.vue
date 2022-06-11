<script setup lang="ts">
import { ref, reactive, computed } from "vue"
import { Social } from "../../js/global-types"

interface Props {
  link: Social
}

const props = defineProps<Props>()

const icon = computed<string | undefined>(() => {
  const l: string = props.link.url
  if (l.includes("twitter")) return "/icons/twitter-brands.svg"
  if (l.includes("facebook")) return "/icons/facebook-square-brands.svg"
  if (l.includes("github")) return "/icons/github-square-brands.svg"
  if (l.includes("steam")) return "/icons/steam-brands.svg"
})
</script>

<template>
  <a :href="props.link.url" target="_blank" class="social-link">
    <img v-if="icon" :src="icon" alt=" " />
    <span v-else class="material-icons"> &#xe157; </span>
    <span>{{ props.link.name }}</span>
  </a>
</template>
