<script setup lang="ts">
import { computed, onBeforeMount, ref, watch } from "vue"
import { useRoute, useRouter } from "vue-router"
import { imageUrl, useAlbums, Album } from "../../store/album"
import { isEmpty, isNil } from "lodash"
import { useLoading } from "../../store/loading"
import { onKeyStroke } from "@vueuse/core"

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
const { getLoading } = useLoading()

const showMeta = ref(false)

// const data = album.getImageMetadata()

// Shut the fuck up typescript
const albumKey = computed(() => `${route.params.album}`)
const imageKey = computed(() => `${route.params.image}`)

// Get album data
const album = computed<Album>(() => albums.getAlbum(albumKey.value) as Album)
const image = computed(() => album.value.images.find((item) => item.key === imageKey.value))

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
    // Reset
    url.value = null

    // Create new image and load it
    let img = new Image()
    img.src = imageUrl(key)
    img.onload = () => {
      // Append src to image element, end loading
      url.value = img.src
    }
  },
  {
    immediate: true
  }
)

// Arrow keys
onKeyStroke("ArrowLeft", () => setIndex("prev"))
onKeyStroke("ArrowRight", () => setIndex("next"))
onKeyStroke("Escape", () => {
  router.push({ name: "AlbumDetail", params: { id: albumKey.value } })
})
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
          <button class="hover-bubble" @click="showMeta = !showMeta">
            <span class="material-icons" v-if="showMeta">&#xe5cd;</span>
            <span class="material-icons" v-else>&#xe88e;</span>
            {{ showMeta ? "Close" : "Details" }}
          </button>
        </div>

        <div class="hi-image-context">
          <router-link :to="{ name: 'AlbumDetail', params: { id: albumKey } }">
            <span class="material-icons"> &#xe2ea; </span>
            Go to album
          </router-link>

          <div class="flex-1"></div>

          <p>Photo {{ index + 1 }} / {{ album.images.length }}</p>

          <button
            class="nav-prev hover-bubble"
            data-title-top="Previous photo"
            :class="{ disabled: isNil(prevIndex) }"
            @click="setIndex('prev')"
          >
            <img src="/icons/arrow-left-long.svg" alt="" />
          </button>

          <button
            class="nav-left hover-bubble"
            data-title-top="Next photo"
            :class="{ disabled: isNil(nextIndex) }"
            @click="setIndex('next')"
          >
            <img src="/icons/arrow-right-long.svg" alt="" />
          </button>
        </div>
      </div>

      <div class="hi-image-meta-wrap" :class="{ active: showMeta }">
        <div class="hi-image-meta">
          <pre>
            {{ image }}
          </pre>
        </div>
      </div>
    </template>
  </div>
</template>
