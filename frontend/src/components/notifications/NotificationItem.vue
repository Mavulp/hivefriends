<script setup lang="ts">
import { ref } from "vue"
import { Alert } from "../../store/notification"

const props = defineProps<{
  data: Alert
}>()
</script>

<template>
  <div>
    <pre>
      {{ props.data }}
    </pre>
  </div>
</template>
