<script setup lang='ts'>
import { onBeforeMount } from 'vue'
import Activity from '../../components/activity/Activity.vue'
import { useActivity } from '../../store/activity'
import { useAlbums } from '../../store/album'

const activity = useActivity()
const albums = useAlbums()

onBeforeMount(() => {
  window.scrollTo({ top: 0, behavior: 'smooth' })
  activity.fetchActivity()
  albums.fetchAlbums()
})
</script>

<template>
  <div class="hi-home hi-activity-log">
    <div class="container">
      <h3>Activity</h3>
      <Activity class="active activity-home activity-page" />
    </div>
  </div>
</template>
