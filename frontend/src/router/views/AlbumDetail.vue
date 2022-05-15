<script setup lang="ts">
import { onBeforeMount, computed, reactive } from "vue"
import { useRoute } from "vue-router"
import { useAlbums, Album, imageUrl, Image } from "../../store/album"
import { useLoading } from "../../store/loading"
import { isEmpty } from "lodash"
import { useAuth } from "../../store/auth"
import { formatDate } from "../../js/utils"

import LoadingSpin from "../../components/loading/LoadingSpin.vue"
import AlbumTimestamp from "../../components/albums/AlbumTimestamp.vue"
import ImageListitem from "../../components/albums/ImageListitem.vue"
import { useBread } from "../../store/bread"

const albums = useAlbums()
const route = useRoute()
const user = useAuth()
const bread = useBread()

const { getLoading } = useLoading()

const album = reactive<Album>({} as Album)

onBeforeMount(async () => {
  const id = route.params.id

  if (id) {
    const data = await albums.fetchAlbum(id)
    Object.assign(album, data)

    bread.set(`${data.title} by ${user.getUsername(data.uploaderKey)}`, `${data.title} // hi!friends`)
  }
})

const chunks = computed(() => {
  if (!album.images) return []

  const images: any = album.images
  const chunks: Array<Array<Image>> = [[], [], []]

  let i: number = 0
  let j: number = 0

  while (i !== images.length) {
    chunks[j].push(images[i])

    if (j >= 2) j = 0
    else j++

    i++
  }

  return chunks
})
</script>

<template>
  <div class="hi-album-detail">
    <LoadingSpin dark v-if="getLoading('get-album')" />

    <div class="hi-album-detail-error" v-else-if="isEmpty(album)">
      <div class="centered">
        <h3>Lmao</h3>
        <p>Error loading album</p>
      </div>
    </div>

    <div class="hi-album-title" v-else>
      <div class="hi-album-title-meta">
        <AlbumTimestamp class="dark" :timeframe="album.timeframe" />

        <h1>{{ album.title }}</h1>
        <p>{{ album.description }}</p>

        <div class="album-meta-cells">
          <span class="material-icons">&#xe3f4;</span>
          <p class="mr-32">{{ album.images.length }} Images</p>

          <span class="material-icons">&#xe851;</span>
          <router-link :to="{ name: 'UserProfile', params: { id: album.uploaderKey } }" class="mr-32">
            by: {{ user.getUsername(album.uploaderKey) }}
          </router-link>

          <span class="material-icons">&#xe8df;</span>
          <p>Uploaded</p>
          <p>{{ formatDate(album.createdAt) }}</p>
        </div>
      </div>

      <div class="hi-album-title-thumbnail">
        <div class="thumbnail-image-wrap">
          <img :src="imageUrl(album.images[0].key)" alt="" />
        </div>
      </div>
    </div>

    <div class="hi-album-images">
      <div class="hi-album-image-col" v-for="chunk in chunks" :key="chunk.length">
        <ImageListitem v-for="image in chunk" :key="image.key" :image="image" :album-key="album.key" />
      </div>
    </div>
  </div>
</template>
