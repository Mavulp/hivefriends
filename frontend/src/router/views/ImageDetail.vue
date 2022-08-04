<script setup lang="ts">
import { computed, onBeforeMount, onBeforeUnmount, onMounted, watch, ref } from "vue"
import { useRoute, useRouter } from "vue-router"
import { imageUrl, useAlbums, Album, Image as ImageStruct } from "../../store/album"
import { isEmpty, isNil } from "lodash"
import { useLoading } from "../../store/loading"
import { onKeyStroke, useClipboard, useCssVar, useMediaQuery, usePreferredDark, whenever } from "@vueuse/core"
import { map_access, map_dark, map_light, getBounds, isValidMarker } from "../../js/map"
import { useUser } from "../../store/user"
import { RGB_TO_HEX, formatDate, formatFileSize, sanitize } from "../../js/utils"
import { useComments } from "../../store/comments"
import { useToast } from "../../store/toast"
import { useBread } from "../../store/bread"
import { url } from "../../js/fetch"
import { Map } from "mapbox-gl"

import { MapboxMap, MapboxMarker } from "vue-mapbox-ts"
import LoadingSpin from "../../components/loading/LoadingSpin.vue"
import CommentsWrap from "../../components/comments/CommentsWrap.vue"
import Modal from "../../components/Modal.vue"

/**
 *  Setup
 */
const wrap = ref(null)
const color = useCssVar("--color-highlight", wrap)
const isPhone = useMediaQuery("(max-width: 512px)")
const scrollWrap = ref<HTMLElement>()

/**
 * Lifecycle
 */

onBeforeMount(async () => {
  if (!albums.getAlbum(albumKey.value)) {
    albums.fetchAlbum(albumKey.value, user.public_token)
  }

  const isDark = usePreferredDark()

  if (isDark && user.public_token) {
    const root = document.querySelector(":root")
    if (root) {
      root.classList.add("dark-normal")
    }
  }

  window.scrollTo({ top: 0 })
})

onBeforeUnmount(() => {
  window.removeEventListener("scroll", () => {})
})

/**
 * Image navigation
 */

const imageDetailUrl = ref<string | null>(null)

const route = useRoute()
const router = useRouter()
const albums = useAlbums()
const user = useUser()
const bread = useBread()
const { getLoading, addLoading, delLoading } = useLoading()

const transDir = ref("imagenext")
const showComments = ref(isPhone.value ? false : localStorage.getItem("show-comments") === "true" ? true : false)

watch(showComments, (val: boolean) => localStorage.setItem("show-comments", val ? "true" : "false"))

// Shut the fuck up typescript
const albumKey = computed(() => route.params?.album?.toString() ?? null)
const imageKey = computed(() => route.params?.image?.toString() ?? null)

// Get album data
const album = computed<Album>(() => albums.getAlbum(albumKey.value) as Album)

const image = computed(() => album.value?.images.find((item) => item.key === imageKey.value))

// Get image's index from the current album's images
const index = computed<number>(() => album.value?.images?.findIndex((item) => item.key === imageKey.value))

const prevIndex = computed(() => album.value.images[index.value - 1])
const nextIndex = computed(() => album.value.images[index.value + 1])

function setIndex(where: string) {
  transDir.value = `image${where}`

  let newIndex

  if (where === "prev" && prevIndex) {
    newIndex = index.value - 1

    // Cancel click
    if (newIndex < 0) return
  } else if (where === "next" && nextIndex) {
    newIndex = index.value + 1

    // Cancel click
    if (newIndex >= album.value.images.length) return
  }

  if (!isNil(newIndex)) {
    router.push({
      name: user.public_token ? "PublicImageDetail" : "ImageDetail",
      params: {
        album: albumKey.value,
        image: album.value.images[newIndex].key,
        ...(user.public_token && { token: user.public_token })
      }
    })
  }
}

watch(
  imageKey,
  (key) => {
    if (key) {
      // Reset
      imageDetailUrl.value = null

      let img = new Image()
      img.src = imageUrl(key)
      img.onload = () => {
        imageDetailUrl.value = img.src
      }

      if (!user.public_token) {
        albums.fetchImageMetadata(key)
      }

      if (image.value) {
        bread.set(`${image.value.fileName} by ${user.getUsername(image.value.uploader)}`)
      }
    }
  },
  { immediate: true }
)

// Arrow keys
onKeyStroke("ArrowLeft", () => setIndex("prev"))
onKeyStroke("ArrowRight", () => setIndex("next"))
onKeyStroke("Escape", () => {
  router.push({ name: "AlbumDetail", params: { id: albumKey.value } })
})

whenever(album, () => {
  const accent = user.getUser(album.value.author, "accentColor")
  color.value = accent

  album.value.images.map((image) => {
    const img = new Image()
    img.src = imageUrl(image.key, "large")
  })
})

/**
 * Map & metadata
 */

const map = ref<Map>()

function onMapLoaded(mapObject: Map) {
  map.value = mapObject

  setTimeout(() => {
    mapObject.fitBounds(getBounds(sortedMarkers.value), { padding: 128 })
  }, 50)
}

const mapStyle = computed(() => {
  if (user.public_token) {
    return usePreferredDark() ? map_dark : map_light
  }

  return user.settings?.colorTheme?.startsWith("dark") ? map_dark : map_light
})

const sortedMarkers = computed(() => {
  // Make sure the current marker is always the last one to render

  return album.value.images
    .filter((item) => isValidMarker(item))
    .sort((a: ImageStruct, b: ImageStruct) => (a.key === image.value?.key ? 1 : -1))
})

function openImageId(key: string) {
  router.push({
    name: user.public_token ? "PublicImageDetail" : "ImageDetail",
    params: {
      album: albumKey.value,
      image: key,
      ...(user.public_token && { token: user.public_token })
    }
  })

  if (scrollWrap.value) {
    scrollWrap.value.scrollTo({ behavior: "smooth", top: 0 })
  }
}

function scrollDown() {
  if (scrollWrap.value) {
    scrollWrap.value.scrollTo({ top: window.innerHeight, behavior: "smooth" })
  }
}

const comment = useComments()

/**
 * Generate public link
 */
const modal = ref(false)
const publicLink = ref("")
const { copy, isSupported } = useClipboard()
const toast = useToast()

async function getPublicLink() {
  if (user.public_token) {
    publicLink.value = window.location.href
    modal.value = true
    return
  }

  if (!publicLink.value) {
    addLoading("share-link")

    const token = await albums.genPublicAlbumToken(albumKey.value)
    delLoading("share-link")

    if (token) publicLink.value = `${url}/public${route.fullPath}/${token}`
  }

  // Dont error if public_token exists
  if (!publicLink.value) {
    toast.add("Error generating sharing link. No idea why tbh.", "error")
  } else {
    modal.value = true
  }
}

function doCopy(type: string) {
  if (type === "url") {
    copy(imageDetailUrl.value ?? "")
    toast.add("Photo url copied to clipboard")
  } else if (type === "public") {
    copy(publicLink.value)
    toast.add("Photo share link copied to clipboard")
  }
}

/**
 * Swiping
 */

let touchstartX = 0
let touchendX = 0
const threshold = 100

function checkDirection() {
  if (touchstartX === touchendX || window.scrollY > window.innerHeight * 0.25) return

  if (touchendX - threshold > touchstartX) {
    // swipe right, go to previous
    setIndex("prev")
  }
  if (touchendX + threshold < touchstartX) {
    // swipe left, go to next
    setIndex("next")
  }
}

onMounted(() => {
  document.addEventListener("touchstart", (e) => {
    touchstartX = e.changedTouches[0].screenX
  })

  document.addEventListener("touchend", (e) => {
    touchendX = e.changedTouches[0].screenX
    checkDirection()
  })
})

onBeforeUnmount(() => {
  document.removeEventListener("touchstart", () => {})
  document.removeEventListener("touchend", () => {})
})
</script>

<template>
  <div class="hi-image-detail" ref="wrap">
    <LoadingSpin class="center-page" dark v-if="getLoading('get-album')" />

    <div class="hi-album-detail-error" v-else-if="isEmpty(album)">
      <div class="centered">
        <h3>Error</h3>
        <p>Something happened somewhere o_o</p>
      </div>
    </div>

    <div class="content-wrap" v-else-if="imageKey && album && image" ref="scrollWrap">
      <Teleport to="body">
        <Modal @close="modal = false" v-if="modal">
          <div class="modal-wrap modal-copy">
            <div class="modal-title">
              <h4>Get sharing links</h4>
              <button class="modal-close" @click="modal = false">
                <span class="material-icons">&#xe5cd;</span>
              </button>
            </div>

            <p>Anyone with this link will be able to view this image and album it's uploaded in.</p>

            <div class="input-with-copy">
              <span class="material-icons"> &#xe80d; </span>
              <input name="public-link-input" readonly :value="publicLink" />
              <button class="hover-bubble bubble-black" v-if="isSupported" @click="doCopy('public')">
                <span class="material-icons"> &#xe14d; </span>Copy link
              </button>
            </div>

            <hr />
            <p>Link to the image file. (Better for quick sharing)</p>

            <div class="input-with-copy" v-if="imageDetailUrl">
              <span class="material-icons"> &#xe157; </span>
              <input name="url-input" readonly :value="imageDetailUrl" />
              <button class="hover-bubble bubble-black" v-if="isSupported" @click="doCopy('url')">
                <span class="material-icons"> &#xe14d; </span> Copy link
              </button>
            </div>
          </div>
        </Modal>
      </Teleport>

      <div class="hi-image-container">
        <div class="hi-image-wrapper">
          <transition :name="transDir" mode="out-in">
            <img v-if="imageDetailUrl" :src="imageDetailUrl" ref="imageel" />
            <div v-else class="image-loading">
              <LoadingSpin dark />
            </div>
          </transition>
        </div>

        <div class="hi-image-context">
          <div class="context-col">
            <router-link
              :class="[isPhone ? 'phone-go-back' : 'hover-bubble']"
              :to="{
                name: user.public_token ? 'PublicAlbumDetail' : 'AlbumDetail',
                params: { id: albumKey, ...(user.public_token && { token: user.public_token }) }
              }"
            >
              <template v-if="isPhone">
                <span class="material-icons"> &#xe5cd; </span>
              </template>
              <template v-else>
                <span class="material-icons" style="font-size: 2rem"> &#xf1e6; </span>
                Go back
              </template>
            </router-link>
          </div>

          <div class="context-col">
            <button class="hover-bubble" @click="scrollDown()">
              <span class="material-icons">&#xe5cf;</span>
              Details
            </button>

            <button class="hover-bubble" @click="showComments = !showComments">
              <span class="material-icons">&#xe0b9;</span>
              Comments

              <span class="material-icons rotate" v-if="getLoading('comments')">&#xe863;</span>
              <template v-else> ({{ comment.comments.length }}) </template>
            </button>

            <button class="hover-bubble" @click="getPublicLink()">
              <span class="material-icons">&#xe80d;</span>
              Share
              <span class="material-icons rotate" v-if="getLoading('share-link')">&#xe863;</span>
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
              <span class="material-icons" v-if="isPhone"> &#xf1e6;</span>
              <img src="/icons/arrow-left-long.svg" v-else alt=" " />
            </button>

            <button
              class="nav-left hover-bubble"
              data-title-top="Next photo"
              :class="{ disabled: isNil(nextIndex) }"
              @click="setIndex('next')"
            >
              <span class="material-icons" v-if="isPhone"> &#xf1df;</span>
              <img src="/icons/arrow-right-long.svg" v-else alt=" " />
            </button>
          </div>
        </div>
      </div>
      <div class="divider"></div>
      <div class="hi-image-container">
        <div class="hi-image-meta">
          <h4>
            Photo details

            <span
              v-if="image.location"
              class="material-icons tooltip-width-200"
              data-title-top="You can zoom / move within the map. Click other markers to switch pictures."
            >
              &#xe8fd;
            </span>
          </h4>

          <div class="wrapper">
            <div class="hi-map-wrap" v-if="image.location && image.location.latitude && image.location.longitude">
              <mapbox-map
                :accessToken="map_access"
                :mapStyle="mapStyle"
                :zoom="11"
                :center="[image.location.longitude, image.location.latitude]"
                @loaded="onMapLoaded"
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
                  <li v-if="image.location.latitude">
                    <span class="material-icons"> &#xe87a; </span>
                    <span>Latitude</span>
                    <p>{{ image.location.latitude }}</p>
                  </li>

                  <li v-if="image.location.longitude">
                    <span class="material-icons"> &#xe87a; </span>
                    <span>Longitude</span>
                    <p>{{ image.location.longitude }}</p>
                  </li>
                </template>

                <li v-if="image.takenAt">
                  <span class="material-icons"> &#xebcc; </span>
                  <span>Taken</span>
                  <p>{{ formatDate(image.takenAt) }}</p>
                </li>
              </ul>
            </div>
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
