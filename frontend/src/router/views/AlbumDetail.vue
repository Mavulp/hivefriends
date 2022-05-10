<script setup lang="ts">
import { ref } from "vue"

const comments = ref(false)

const album = {
  title: "That time in Butjadingen Nothing Happened",
  description: "Cum laude",
  users: ["user-id-0", "user-id-1", "user-id-2", "user-id-3"],
  timestamps: {
    from: "14/07/2021",
    to: "21/07/2021",
    created: "32/08/2021"
  },
  photos: 73
}
</script>

<template>
  <div class="hi-album-detail">
    <div class="hi-album-detail-layout">
      <div class="col-query">
        <div class="hi-album-header">
          <div class="album-timestamp">
            <span>{{ album.timestamps.from }}</span>
            <div class="timestamp-divider"></div>
            <span>{{ album.timestamps.to }}</span>
          </div>

          <h2>{{ album.title }}</h2>
          <p>{{ album.description }}</p>

          <div class="album-description">
            <span> Users: </span>
            <div class="album-description-users">
              <button class="album-description-user"></button>
              <button class="album-description-user"></button>
              <button class="album-description-user"></button>
            </div>

            <span> Photos: {{ album.photos }} </span>

            <span>Created: {{ album.timestamps.created }}</span>
          </div>
        </div>
        <div class="hi-album-images">
          <!-- Generate columns by splitting array of images into 3
          and iterating over those to generate a bit more natural masonry -->

          <div class="col"></div>
          <div class="col"></div>
          <div class="col"></div>
        </div>
      </div>

      <div class="hi-album-comments"></div>
    </div>
  </div>
</template>
