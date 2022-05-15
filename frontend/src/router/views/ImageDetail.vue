<script setup lang="ts">
import { computed, onBeforeMount, ref, watch } from "vue"
import { useRoute, useRouter } from "vue-router"
import { imageUrl, useAlbums, Album } from "../../store/album"
import { isEmpty, isNil } from "lodash"
import { useLoading } from "../../store/loading"

import LoadingSpin from "../../components/loading/LoadingSpin.vue"

onBeforeMount(() => {
  if (!albums.getAlbum(albumKey.value)) {
    albums.fetchAlbum(albumKey.value)
  }
})

const url = ref<string | null>(null)

const route = useRoute()
const router = useRouter()
const albums = useAlbums()
const { getLoading, addLoading, delLoading } = useLoading()

// Shut the fuck up typescript
const albumKey = computed(() => `${route.params.album}`)
const imageKey = computed(() => `${route.params.image}`)

// Get album data
const album = computed<Album>(() => albums.getAlbum(albumKey.value) as Album)

// Get image's index from the current album's images
const index = computed<number>(() => album.value?.images.findIndex((item) => item.key === imageKey.value))

const prevIndex = computed(() => album.value.images[index.value - 1])
const nextIndex = computed(() => album.value.images[index.value + 1])

function setIndex(where: string) {
  let newIndex

  if (where === "prev" && prevIndex) {
    newIndex = index.value - 1
  } else if (where === "next" && nextIndex) {
    newIndex = index.value + 1
  }

  if (!isNil(newIndex)) {
    router.push({
      name: "ImageDetail",
      params: {
        album: albumKey.value,
        image: album.value.images[newIndex].key
      }
    })
  }
}

watch(
  imageKey,
  (key) => {
    url.value = null
    // addLoading("get-image")
    let img = new Image()
    img.src = imageUrl(key)
    img.onload = () => {
      url.value = img.src

      // delLoading("get-image")
    }
  },
  {
    immediate: true
  }
)
</script>

<template>
  <div class="hi-image-detail">
    <LoadingSpin dark v-if="getLoading('get-album')" />

    <div class="hi-album-detail-error" v-else-if="isEmpty(album)">
      <div class="centered">
        <h3>Error</h3>
        <p>Something happened somewhere o_o</p>
      </div>
    </div>

    <template v-else-if="album">
      <div class="hi-image-wrapper">
        <transition name="fade" mode="out-in" appear>
          <img v-if="url" :src="url" alt="" />
          <div v-else class="image-loading">
            <LoadingSpin dark />
          </div>
        </transition>

        <div class="hi-image-context context-top">
          <div class="flex-1"></div>
          <button class="hover-bubble">
            <span class="material-icons">&#xe88e;</span>
            Details
          </button>
        </div>

        <div class="hi-image-context">
          <router-link :to="{ name: 'AlbumDetail', params: { id: albumKey } }">
            <span class="material-icons"> &#xe2ea; </span>
            View Album
          </router-link>

          <div class="flex-1"></div>

          <p>Image {{ index + 1 }} / {{ album.images.length }}</p>

          <button
            class="nav-prev hover-bubble"
            data-title-top="Previous Image"
            :class="{ disabled: isNil(prevIndex) }"
            @click="setIndex('prev')"
          >
            <img src="/icons/arrow-left-long.svg" alt="" />
          </button>

          <button
            class="nav-left hover-bubble"
            data-title-top="Next Image"
            :class="{ disabled: isNil(nextIndex) }"
            @click="setIndex('next')"
          >
            <img src="/icons/arrow-right-long.svg" alt="" />
          </button>
        </div>
      </div>

      <!-- <div class="hi-image-controls">
      </div> -->
    </template>
  </div>
</template>
