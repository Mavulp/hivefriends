<script setup lang="ts">
import { ref, computed } from "vue"
import { usePreferredDark } from "@vueuse/core"
import { MapboxMap, MapboxMarker, MapboxPopup } from "vue-mapbox-ts"
import { Map } from "mapbox-gl"
import { map_access, map_dark, map_light, isValidMarker } from "../../js/map"
import { useUser } from "../../store/user"
import { Album, Image, imageUrl } from "../../store/album"
import { TEXT_CONTRAST, RGB_TO_HEX } from "../../js/utils"
import { getBounds } from "../../js/map"
import { isNil } from "lodash"

import CommentsWrap from "../../components/comments/CommentsWrap.vue"

interface Props {
  album: Album
}

interface Emits {
  (e: "close"): void
}

const user = useUser()
const emit = defineEmits<Emits>()
const props = defineProps<Props>()
const zoom = ref(12)
const map = ref<Map>()

const accent = computed(() =>
  user
    .getUser(props.album.author, "accentColor")
    .split(",")
    .map((item: string) => Number(item))
)

const mapStyle = computed(() => {
  if (user.public_token) return usePreferredDark() ? map_dark : map_light
  return user.settings?.colorTheme?.startsWith("dark") ? map_dark : map_light
})

const sortedMarkers = computed(() => props.album.images.filter((item) => isValidMarker(item)))
const mapCenter = ref<Array<string | number>>([
  Number(sortedMarkers.value[0].location?.longitude ?? 0),
  Number(sortedMarkers.value[0].location?.latitude ?? 0)
])

function onMapLoaded(mapObject: Map) {
  map.value = mapObject

  setTimeout(() => {
    mapObject.fitBounds(getBounds(sortedMarkers.value), { padding: Math.round(window.innerWidth / 4) })
  }, 50)
}

/**
 * Active stuff
 */

const activeIndex = ref<number | null>()
const activeImage = ref<Image | null>()

function setActiveImage(item: Image, index: number) {
  // Set active image + the index
  activeImage.value = item
  activeIndex.value = index
}

function resetActiveImage() {
  activeImage.value = null
  activeIndex.value = null
}

function setActiveLocation(index: number) {
  if (isNil(activeIndex.value)) {
    // Either 1 or -1
    activeIndex.value = index === 1 ? 0 : -1
    activeImage.value = sortedMarkers.value.at(activeIndex.value)
  }

  const newIndex = activeIndex.value + index
  const image = sortedMarkers.value[newIndex]

  if (image) {
    //@ts-ignore
    mapCenter.value = [image.location.longitude, image.location.latitude]
    setActiveImage(image, newIndex)
  } else {
    const correctIndex = newIndex < 0 ? sortedMarkers.value.length - 1 : 0
    const image = sortedMarkers.value[correctIndex]

    setActiveImage(image, correctIndex)
  }
}

function setZoom(by: number) {
  if (map.value) {
    const current = map.value.getZoom()
    zoom.value = current + by
  }
}
</script>

<template>
  <div class="hi-album-map">
    <div class="album-map-header">
      <h4>Album Locations</h4>
      <p>
        "{{ props.album.title }}"
        <span> by </span>
        <router-link :to="{ name: 'UserProfile', params: { user: props.album.author } }">
          {{ user.getUsername(props.album.author) }}
        </router-link>
      </p>
    </div>

    <div class="album-map-layout">
      <div class="map-wrapper">
        <div class="active-image-data" :class="{ active: activeImage }">
          <template v-if="activeImage" :key="activeImage.key">
            <img :src="imageUrl(activeImage.key, 'medium')" alt="" />

            <CommentsWrap
              :albumKey="props.album.key"
              :imageKey="activeImage.key"
              :uploader="props.album.author"
              @close="resetActiveImage()"
            />
          </template>
        </div>

        <mapbox-map
          :accessToken="map_access"
          :mapStyle="mapStyle"
          :center="mapCenter"
          @loaded="onMapLoaded"
          :zoom="zoom"
        >
          <template v-for="(marker, index) in sortedMarkers">
            <mapbox-marker
              class="map-marker"
              @click="setActiveImage(marker, index)"
              v-if="marker.location && map"
              :lngLat="[marker.location.longitude, marker.location.latitude]"
              :color="marker.key === activeImage?.key ? RGB_TO_HEX(accent.join(',')) : '#8284ff'"
            />
          </template>
        </mapbox-map>
      </div>

      <div class="map-controls">
        <button data-title-left="Previous location" class="hover-bubble" @click="setActiveLocation(-1)">
          <span class="material-icons">&#xe5cb;</span>
        </button>
        <button data-title-left="Next location" class="hover-bubble" @click="setActiveLocation(1)">
          <span class="material-icons">&#xe409;</span>
        </button>

        <div class="padder" />

        <button data-title-left="Zoom in" class="hover-bubble" @click="setZoom(1)">
          <span class="material-icons">&#xe8ff;</span>
        </button>
        <button data-title-left="Zoom out" class="hover-bubble" @click="setZoom(-1)">
          <span class="material-icons">&#xe900;</span>
        </button>

        <div class="padder" />

        <router-link
          data-title-left="Author's profile"
          :to="{ name: 'UserProfile', params: { user: props.album.author } }"
        >
          <img
            class="user-image"
            :src="imageUrl(user.getUser(props.album.author, 'avatarKey'), 'tiny')"
            alt=""
            @error="(e: any) => e.target.classList.add('image-error')"
          />
        </router-link>

        <div class="flex-1"></div>

        <button @click="emit('close')" data-title-left="Close" class="hover-bubble">
          <span class="material-icons">&#xe5cd;</span>
        </button>
      </div>
    </div>
  </div>
</template>
