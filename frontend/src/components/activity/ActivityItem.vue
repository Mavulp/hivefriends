<script setup lang="ts">
import { computed, defineAsyncComponent } from "vue"
import { ActivityItem } from "../../store/activity"

const props = defineProps<{ data: ActivityItem }>()

const type = computed(() => Object.keys(props.data)[0])

const component = defineAsyncComponent(() => import(`./models/model-${type.value}.vue`))
</script>

<template>
  <component :is="component" :data="Object.values(props.data)[0]" />
</template>
