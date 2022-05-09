<script setup lang="ts">
import { ref } from "vue"

import Album from "../../components/albums/Album.vue"
import Button from "../../components/Button.vue"

const album = {
  title: "That time in Butjadingen Nothing Happened",
  users: ["user-id-0", "user-id-1", "user-id-2", "user-id-3"],
  timestamps: {
    from: "14/07/2021",
    to: "21/07/2021"
  },
  photos: 73
}
</script>

<template>
  <div class="hi-home">
    <div class="hi-home-layout">
      <div class="album-column large">
        <Album :data="album" large />
      </div>

      <div class="album-column small">
        <Album :data="album" />
        <Album :data="album" />
        <!-- <Album :data="album" /> -->
      </div>
    </div>

    <!-- <div class="hi-home-button">
      <Button>All Albums</Button>
    </div> -->
  </div>
</template>
