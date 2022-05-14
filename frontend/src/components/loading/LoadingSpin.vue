<script setup lang="ts">
interface Props {
  dark?: boolean
}

const { dark } = defineProps<Props>()
</script>

<template>
  <div class="loading-spinner" :class="{ dark }"></div>
</template>
