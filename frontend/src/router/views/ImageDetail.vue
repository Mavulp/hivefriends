<script setup lang="ts">
import { computed, onBeforeMount, onBeforeUnmount, onMounted, watch, ref, watchEffect } from "vue"
import { useRoute, useRouter } from "vue-router"
import { imageUrl, useAlbums, Album, Image as ImageStruct } from "../../store/album"
import { isEmpty, isNil } from "lodash"
import { useLoading } from "../../store/loading"
import { onKeyStroke, useClipboard, useCssVar, whenever } from "@vueuse/core"
import { map_access, map_dark, map_light } from "../../js/map"
import { useUser } from "../../store/user"
import { RGB_TO_HEX, formatDate, formatFileSize } from "../../js/utils"

import { MapboxMap, MapboxMarker } from "vue-mapbox-ts"
import LoadingSpin from "../../components/loading/LoadingSpin.vue"
import CommentsWrap from "../../components/comments/CommentsWrap.vue"
import { useComments } from "../../store/comments"
import { useToast } from "../../store/toast"

/**
 *  Setup
 */
const wrap = ref(null)
const color = useCssVar("--color-highlight", wrap)

/**
 * Lifecycle
 */

onBeforeMount(async () => {
  if (!albums.getAlbum(albumKey.value)) {
    albums.fetchAlbum(albumKey.value)
  }

  window.scrollTo({ top: 0 })
})

onBeforeUnmount(() => {
  window.removeEventListener("scroll", () => {})
})

onMounted(() => {
  window.addEventListener("scroll", () => {})
})

/**
 * Image navigation
 */

const url = ref<string | null>(null)

const route = useRoute()
const router = useRouter()
const albums = useAlbums()
const user = useUser()
const { getLoading } = useLoading()

const transDir = ref("imagenext")
const showComments = ref(false)

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
  transDir.value = `image${where}`

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
    img.src = imageUrl(key, "large")
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

watchEffect(() => {
  if (album.value) {
    const accent = user.getUser(album.value.author, "accentColor")
    color.value = accent
  }
})

/**
 * Map & metadata
 */

const mapStyle = computed(() => (user.settings.colorTheme.startsWith("dark") ? map_dark : map_light))
const sortedMarkers = computed(() => {
  // Make sure the current marker is always the last one to render

  return album.value.images
    .filter((item) => item.location)
    .sort((a: ImageStruct, b: ImageStruct) => (a.key === image.value?.key ? 1 : -1))
})

function openImageId(key: string) {
  router.push({
    name: "ImageDetail",
    params: {
      album: albumKey.value,
      image: key
    }
  })

  window.scrollTo({ top: 0, behavior: "smooth" })
}

function scrollDown() {
  window.scrollTo({ top: window.innerHeight / 1.25, behavior: "smooth" })
}

const comment = useComments()

/**
 * Copy
 */
const { copy } = useClipboard()
const toast = useToast()

function copyClipboard() {
  if (url.value) {
    copy(url.value)
    toast.add("Image link copied to clipboard")
  }
}
</script>

<template>
  <div class="hi-image-detail" ref="wrap">
    <LoadingSpin dark v-if="getLoading('get-album')" />

    <div class="hi-album-detail-error" v-else-if="isEmpty(album)">
      <div class="centered">
        <h3>Error</h3>
        <p>Something happened somewhere o_o</p>
      </div>
    </div>

    <div class="content-wrap" v-else-if="album && image">
      <div class="hi-image-container">
        <div class="hi-image-wrapper">
          <transition :name="transDir" mode="out-in">
            <img v-if="url" :src="url" ref="imageel" />
            <div v-else class="image-loading">
              <LoadingSpin dark />
            </div>
          </transition>
        </div>

        <div class="hi-image-context">
          <div class="context-col">
            <router-link class="hover-bubble" :to="{ name: 'AlbumDetail', params: { id: albumKey } }">
              <span class="material-icons"> &#xe2ea; </span>
              Go back
            </router-link>
          </div>

          <div class="context-col">
            <button class="hover-bubble" @click="scrollDown()">
              <span class="material-icons">&#xe5db;</span>
              Details
            </button>

            <button class="hover-bubble" @click="showComments = !showComments">
              <span class="material-icons">&#xe0b9;</span>
              Comments

              <span class="material-icons rotate" v-if="getLoading('comments')">&#xe863;</span>
              <template v-else> ({{ comment.comments.length }}) </template>
            </button>

            <button class="hover-bubble" @click="copyClipboard">
              <span class="material-icons">&#xe157;</span>
              Share
            </button>
          </div>

          <div class="context-col">
            <p>Photo {{ index + 1 }} / {{ album.images.length }}</p>

            <button
              class="nav-prev hover-bubble"
              data-title-top="Previous photo"
              :class="{ disabled: isNil(prevIndex) }"
              @click="setIndex('prev')"
            >
              <img src="/icons/arrow-left-long.svg" alt=" " />
            </button>

            <button
              class="nav-left hover-bubble"
              data-title-top="Next photo"
              :class="{ disabled: isNil(nextIndex) }"
              @click="setIndex('next')"
            >
              <img src="/icons/arrow-right-long.svg" alt=" " />
            </button>
          </div>
        </div>
      </div>
      <div class="divider"></div>
      <div class="hi-image-container">
        <div class="hi-image-meta">
          <h4>
            Location and metadata

            <span
              v-if="image.location"
              class="material-icons tooltip-width-200"
              data-title-top="You can zoom / move within the map. Click other markers to switch pictures."
            >
              &#xe8fd;
            </span>
          </h4>
          <div class="hi-map-wrap" v-if="image.location">
            <mapbox-map
              :accessToken="map_access"
              :mapStyle="mapStyle"
              :zoom="11"
              :center="[image.location.longitude, image.location.latitude]"
            >
              <template v-for="item in sortedMarkers">
                <mapbox-marker
                  v-if="item.location"
                  :color="
                    item.location.longitude === image?.location?.longitude &&
                    item.location.latitude === image.location.latitude
                      ? RGB_TO_HEX(color)
                      : '#a0a0a055'
                  "
                  @click="openImageId(item.key)"
                  :lngLat="[item.location.longitude, item.location.latitude]"
                />
              </template>
            </mapbox-map>
          </div>

          <div class="hi-image-metadata">
            <div class="hi-image-properties">
              <span>Uploader</span>

              <router-link class="hover-bubble" :to="{ name: 'UserProfile', params: { user: image.uploader } }">
                <img
                  class="user-image"
                  :src="imageUrl(user.getUser(image.uploader, 'avatarKey'), 'tiny')"
                  :style="[`backgroundColor: rgb(${user.getUser(image.uploader, 'accentColor')})`]"
                  alt=" "
                  @error="(e: any) => e.target.classList.add('image-error')"
                />

                {{ image.uploader }}
              </router-link>

              <span>Name</span>
              <strong>{{ image.fileName }}</strong>

              <template v-if="image.description">
                <span>description</span>
                <p>{{ image.description }}</p>
              </template>

              <template v-if="image.takenAt">
                <span>Taken</span>
                <p>{{ formatDate(image.takenAt) }}</p>
              </template>
            </div>

            <ul class="hi-image-metadata-list">
              <li class="meta-item" v-if="image.cameraBrand || image.cameraModel">
                <span class="material-icons"> &#xe412; </span>
                <span>Device</span>
                <p>{{ image.cameraBrand }}, {{ image.cameraModel }}</p>
              </li>

              <li class="meta-item" v-if="image.fNumber || image.focalLength || image.exposureTime">
                <span class="material-icons"> &#xe3af; </span>
                <span>Camera settings</span>
                <p>
                  {{ [image.fNumber, image.focalLength, image.exposureTime].join(", ") }}
                </p>
              </li>

              <li v-if="image.sizeBytes">
                <span class="material-icons"> &#xe161; </span>
                <span>Size</span>
                <p>{{ formatFileSize(image.sizeBytes, true) }}</p>
              </li>

              <template v-if="image.location">
                <li>
                  <span class="material-icons"> &#xe87a; </span>
                  <span>Latitude</span>
                  <p>{{ image.location.latitude }}</p>
                </li>

                <li>
                  <span class="material-icons"> &#xe87a; </span>
                  <span>Longitude</span>
                  <p>{{ image.location.longitude }}</p>
                </li>
              </template>
            </ul>
          </div>
        </div>
      </div>
    </div>

    <div class="comment-wrap" v-if="album && image" :class="{ active: showComments }">
      <CommentsWrap
        @close="showComments = false"
        :albumKey="albumKey"
        :imageKey="imageKey"
        :uploader="image.uploader"
      />
    </div>
  </div>
</template>
