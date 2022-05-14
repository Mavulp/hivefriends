<script setup lang="ts">
import { onBeforeMount, onBeforeUnmount, reactive } from "vue"
import { useRoute } from "vue-router"
import { useAlbums, Album } from "../../store/album"
import { useLoading } from "../../store/loading"
import { rootUrl } from "../../js/fetch"
// import { useAuth } from "../../store/auth"

const albums = useAlbums()
const { getLoading } = useLoading()
const route = useRoute()
// const user = useAuth()

const album = reactive<Album | {}>({})

onBeforeMount(async () => {
  const id = route.params.id

  if (id) {
    const data = await albums.fetchAlbum(id)

    Object.assign(album, data)
  }
})

// Generate columns by splitting array of images into 3 and iterating over those to generate a bit more natural masonry
</script>

<template>
  <div class="hi-album-detail">
    <span v-if="getLoading('get-album')">Loading</span>

    <div v-else>
      <pre>
        {{ album }}
      </pre>

      <!-- <div v-for="image in album.images">
        <img :src="rootUrl + `/data/image/${image.key}/medium.png`" alt="" />
      </div> -->
    </div>
    <!-- <div class="hi-album-detail-layout">
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
          <div class="col"></div>
          <div class="col"></div>
          <div class="col"></div>
        </div>
      </div>

      <div class="hi-album-comments"></div>
    </div> -->
  </div>
</template>
