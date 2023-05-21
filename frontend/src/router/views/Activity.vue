<script setup lang='ts'>
import { onBeforeMount } from 'vue';
import Activity from '../../components/activity/Activity.vue';

onBeforeMount(() => {
   window.scrollTo({ top: 0, behavior: "smooth" })
})
</script>

<template>
  <div class="hi-home hi-activity-log">
     <div class="container">
        <h3>Activity</h3>
        <Activity class="active activity-home" />
     </div>
  </div>
</template>