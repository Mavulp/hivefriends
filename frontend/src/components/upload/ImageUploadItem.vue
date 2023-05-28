<script setup lang="ts">
import { MapboxMap, MapboxMarker } from 'vue-mapbox-ts'
import type { Map } from 'mapbox-gl'
import { computed, reactive, ref, watch } from 'vue'
import { useMediaQuery, usePreferredDark } from '@vueuse/core'
import { isEmpty } from 'lodash'
import LoadingBar from '../loading/LoadingBar.vue'
import Button from '../Button.vue'
import InputText from '../form/InputText.vue'
import InputTextarea from '../form/InputTextarea.vue'
import LoadingSpin from '../../components/loading/LoadingSpin.vue'
import InputCheckbox from '../form/InputCheckbox.vue'

import { map_access, map_dark, map_light } from '../../js/map'

import { imageUrl, useAlbums } from '../../store/album'
import { maxLength, minLength, required, useFormValidation } from '../../js/validation'
import { useLoading } from '../../store/loading'
import { useUser } from '../../store/user'

interface Props {
  data: any
  index: number
}

const { data, index } = defineProps<Props>()

const emit = defineEmits<{
  (e: 'remove', index: number): void
  (e: 'setAsCover', key: string): void
}>()

const user = useUser()
const albums = useAlbums()
const { getLoading } = useLoading()
const isPhone = useMediaQuery('(max-width: 512px)')

const originalCoords = reactive({} as { latitude: string; longitude: string })

const open = ref(false)
function size() {
  if (data.size / 1000000 > 1)
    return `${Math.round(data.size / 1000000)}MB`
  return `${Math.round(data.size / 1000)}KB`
}

const _key = computed(() => data.key)
const map = ref<Map>()
const usemap = ref(true)

const form = reactive({
  fileName: '',
  description: '',
  location: {
    latitude: '',
    longitude: '',
  },
})

watch(
  _key,
  async (val) => {
    if (val) {
      const image = await albums.fetchImageMetadata(data.key)

      Object.assign(originalCoords, image.location)

      if (!originalCoords.latitude || !originalCoords.longitude)
        usemap.value = false

      if (image) {
        form.fileName = image.fileName
        form.description = image.description
        form.location = image.location ?? { latitude: '', longitude: '' }
      }
    }
  },
  { immediate: true },
)

/**
 * Custom image metadata
 */

const rules = computed(() => ({
  fileName: {
    required,
    minLength: minLength(3),
    maxLength: maxLength(96),
  },
  description: {
    maxLength: maxLength(256),
  },
}))

const { errors, validate } = useFormValidation(form, rules, { autoclear: true })

async function submit() {
  validate().then(() => {
    albums.saveImageMetadata(data.key, form)
  })
}

/**
 * Location setting
 */

watch(usemap, (val) => {
  if (!val) {
    form.location.latitude = ''
    form.location.longitude = ''
  }
  else if (!isEmpty(originalCoords)) {
    form.location.latitude = originalCoords.latitude
    form.location.longitude = originalCoords.longitude
  }
})

function MapLoaded(mapObject: Map) {
  map.value = mapObject

  map.value.on('click', (event: any) => {
    const { lng, lat } = event.lngLat

    form.location.latitude = String(lat)
    form.location.longitude = String(lng)
  })
}

const mapStyle = computed(() => {
  if (user.public_token)
    return usePreferredDark() ? map_dark : map_light
  return user.settings?.colorTheme?.startsWith('dark') ? map_dark : map_light
})
</script>

<template>
  <div class="album-upload-item" :class="{ 'open': open, 'has-error': data.error }" :draggable="open ? false : true">
    <div class="album-upload-item-header" @click.self="open = !open">
      <button class="hover-bubble bubble-info">
        <span class="material-icons">&#xe945;</span>
      </button>

      <button class="hover-bubble bubble-orange" :class="{ active: open }" @click="open = !open">
        <span class="material-icons">&#xe745;</span>
      </button>

      <strong>{{ form.fileName.length > 0 ? form.fileName : data.name }}</strong>

      <span class="file-size">{{ isPhone ? "" : "Size:" }} {{ size() }}</span>

      <span class="tag tag-blue">{{ isPhone ? "Cover" : "Album Cover" }}</span>

      <div class="flex-1" />
      <p v-if="data.error">
        {{ data.error.message }}
      </p>

      <LoadingBar :class="[{ 'loading-done': !data.loading }, data.error ? 'loading-error' : 'loading-success']" />

      <button data-title-top="Delete" @click="emit('remove', index)">
        <span class="material-icons">&#xe872;</span>
      </button>
    </div>

    <div v-if="open && !data.error" class="album-upload-content">
      <LoadingSpin v-if="data.loading" dark />

      <div v-else class="grid-view">
        <form @submit.prevent="submit">
          <h6>Edit metadata</h6>

          <InputText
            v-model:value="form.fileName"
            label="Name*"
            placeholder="Change image name"
            :error="errors.fileName"
          />
          <InputTextarea v-model:value="form.description" label="Description" placeholder="Add image description" />

          <div class="map-title">
            <InputCheckbox v-model:check="usemap" />
            <h6>Location</h6>
          </div>

          <div v-if="usemap" class="map-wrapper">
            <MapboxMap
              :access-token="map_access"
              :map-style="mapStyle"
              :center="[form.location.longitude, form.location.latitude]"
              @loaded="MapLoaded"
            >
              <MapboxMarker :lng-lat="[form.location.longitude, form.location.latitude]" />
            </MapboxMap>
          </div>

          <div class="buttons">
            <Button
              class="btn-black"
              type="submit"
              :class="{ 'btn-disabled': getLoading(data.key) }"
              @click.prevent="submit"
            >
              Save
              <LoadingSpin v-if="getLoading(data.key)" />
            </Button>

            <Button class="btn-white" @click.prevent="emit('setAsCover', data.key)">
              Set as album cover
            </Button>
          </div>
        </form>

        <div>
          <h6>Preview</h6>
          <img :src="imageUrl(data.key, 'medium')" alt=" ">
          <!-- add preview? -->
        </div>
      </div>
    </div>
  </div>
</template>
