<script setup lang="ts">
import { defineAsyncComponent } from "vue"
import { ActivityItem } from "../../store/activity"

const props = defineProps<{ data: ActivityItem }>()
const component = defineAsyncComponent(() => import(`./models/model-${Object.keys(props.data)[0]}.vue`))
</script>

<template>
  <component :is="component" :data="Object.values(props.data)[0]" />
</template>
