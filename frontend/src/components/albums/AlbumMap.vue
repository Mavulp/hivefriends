<script setup lang="ts">
import { computed, ref } from 'vue'
import { usePreferredDark } from '@vueuse/core'
import { MapboxMap, MapboxMarker } from 'vue-mapbox-ts'
import type { Map } from 'mapbox-gl'
import { isNil } from 'lodash'
import { getBounds, isValidMarker, map_access, map_dark, map_light } from '../../js/map'
import { useUser } from '../../store/user'
import type { Album, Image } from '../../store/album'
import { imageUrl } from '../../store/album'
import { RGB_TO_HEX } from '../../js/utils'

import CommentsWrap from '../../components/comments/CommentsWrap.vue'

interface Props {
  album: Album
}

interface Emits {
  (e: 'close'): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()
const user = useUser()
const zoom = ref(12)
const map = ref<Map>()

const accent = computed(() =>
  user
    .getUser(props.album.author, 'accentColor')
    .split(',')
    .map((item: string) => Number(item)),
)

const mapStyle = computed(() => {
  if (user.public_token)
    return usePreferredDark() ? map_dark : map_light
  return user.settings?.colorTheme?.startsWith('dark') ? map_dark : map_light
})

const sortedMarkers = computed(() => props.album.images.filter(item => isValidMarker(item)))
const mapCenter = ref<Array<string | number>>([
  Number(sortedMarkers.value[0].location?.longitude ?? 0),
  Number(sortedMarkers.value[0].location?.latitude ?? 0),
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
    // @ts-expect-error no idea stfu
    mapCenter.value = [image.location.longitude, image.location.latitude]
    setActiveImage(image, newIndex)
  }
  else {
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
          <template v-if="activeImage">
            <router-link
              :to="{
                name: user.public_token ? 'PublicImageDetail' : 'ImageDetail',
                params: {
                  album: props.album.key,
                  image: activeImage.key,
                  ...(user.public_token && { token: user.public_token }),
                },
              }"
            >
              <span>
                Go to image detail
                <span class="material-icons">&#xe5cc;</span>
              </span>
              <img :src="imageUrl(activeImage.key, 'medium')" alt="">
            </router-link>

            <CommentsWrap
              :album-key="props.album.key"
              :image-key="activeImage.key"
              :uploader="props.album.author"
              @close="resetActiveImage()"
            />
          </template>
        </div>

        <MapboxMap
          :access-token="map_access"
          :map-style="mapStyle"
          :center="mapCenter"
          :zoom="zoom"
          @loaded="onMapLoaded"
        >
          <template v-for="(marker, index) in sortedMarkers">
            <MapboxMarker
              v-if="marker.location && map"
              :key="marker.location.longitude"
              class="map-marker"
              :lng-lat="[marker.location.longitude, marker.location.latitude]"
              :color="marker.key === activeImage?.key ? RGB_TO_HEX(accent.join(',')) : '#8284ff'"
              @click="setActiveImage(marker, index)"
            />
          </template>
        </MapboxMap>
      </div>

      <div class="map-controls">
        <router-link
          data-title-top="Author's profile"
          :to="{ name: 'UserProfile', params: { user: props.album.author } }"
        >
          <img
            class="user-image"
            :src="imageUrl(user.getUser(props.album.author, 'avatarKey'), 'tiny')"
            alt=""
            @error="(e: any) => e.target.classList.add('image-error')"
          >
        </router-link>

        <div class="padder" />

        <button data-title-top="Previous location" class="hover-bubble" @click="setActiveLocation(-1)">
          <span class="material-icons">&#xe5c4;</span>
        </button>
        <button data-title-top="Next location" class="hover-bubble" @click="setActiveLocation(1)">
          <span class="material-icons">&#xf1df;</span>
        </button>

        <div class="padder" />

        <button data-title-top="Zoom in" class="hover-bubble" @click="setZoom(1)">
          <span class="material-icons">&#xe8ff;</span>
        </button>
        <button data-title-top="Zoom out" class="hover-bubble" @click="setZoom(-1)">
          <span class="material-icons">&#xe900;</span>
        </button>

        <div class="padder" />

        <!-- <div class="flex-1" /> -->

        <button data-title-top="Close" class="hover-bubble" @click="emit('close')">
          <span class="material-icons">&#xe5cd;</span>
        </button>
      </div>
    </div>
  </div>
</template>
