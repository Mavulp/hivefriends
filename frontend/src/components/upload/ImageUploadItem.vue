<script setup lang="ts">
import LoadingBar from "../loading/LoadingBar.vue"
import Button from "../Button.vue"
import InputText from "../form/InputText.vue"
import InputTextarea from "../form/InputTextarea.vue"
import LoadingSpin from "../../components/loading/LoadingSpin.vue"
import InputCheckbox from "../form/InputCheckbox.vue"

import { MapboxMap, MapboxMarker } from "vue-mapbox-ts"
import { map_access, map_dark, map_light } from "../../js/map"
import { Map, MapboxEvent, MapDataEvent } from "mapbox-gl"

import { computed, reactive, ref, watch } from "vue"
import { useAlbums, imageUrl, Image } from "../../store/album"
import { minLength, useFormValidation, required, maxLength } from "../../js/validation"
import { useLoading } from "../../store/loading"
import { useMediaQuery, whenever } from "@vueuse/core"
import { useUser } from "../../store/user"
import { usePreferredDark } from "@vueuse/core"
import { isEmpty } from "lodash"

interface Props {
  data: any
  index: number
}

const { data, index } = defineProps<Props>()

const emit = defineEmits<{
  (e: "remove", index: number): void
  (e: "setAsCover", key: string): void
}>()

const user = useUser()
const albums = useAlbums()
const { getLoading } = useLoading()
const isPhone = useMediaQuery("(max-width: 512px)")

const originalCoords = reactive({} as { latitude: string; longitude: string })

const open = ref(false)
const size = () => {
  if (data.size / 1000000 > 1) return Math.round(data.size / 1000000) + "MB"
  return Math.round(data.size / 1000) + "KB"
}

const _key = computed(() => data.key)

watch(
  _key,
  async (val) => {
    if (val) {
      const image = await albums.fetchImageMetadata(data.key)

      Object.assign(originalCoords, image.location)

      if (isEmpty(originalCoords)) {
        usemap.value = false
      }

      if (image) {
        form.fileName = image.fileName
        form.description = image.description
        form.location = image.location ?? { latitude: "", longitude: "" }
      }
    }
  },
  { immediate: true }
)

/**
 * Custom image metadata
 */

const rules = computed(() => ({
  fileName: {
    required,
    minLength: minLength(3),
    maxLength: maxLength(96)
  },
  description: {
    maxLength: maxLength(256)
  }
}))

const form = reactive({
  fileName: "",
  description: "",
  location: {
    latitude: "",
    longitude: ""
  }
})

const { errors, validate } = useFormValidation(form, rules, { autoclear: true })

async function submit() {
  validate().then(() => {
    albums.saveImageMetadata(data.key, form)
  })
}

/**
 * LOcation setting
 */
const map = ref<Map>()
const usemap = ref(true)

watch(usemap, (val) => {
  if (!val) {
    form.location.latitude = ""
    form.location.longitude = ""
  } else if (!isEmpty(originalCoords)) {
    form.location.latitude = originalCoords.latitude
    form.location.longitude = originalCoords.longitude
  }
})

const MapLoaded = (mapObject: Map) => {
  map.value = mapObject

  map.value.on("click", (event: any) => {
    const { lng, lat } = event.lngLat

    form.location.latitude = String(lat)
    form.location.longitude = String(lng)
  })
}

const mapStyle = computed(() => {
  if (user.public_token) return usePreferredDark() ? map_dark : map_light
  return user.settings?.colorTheme?.startsWith("dark") ? map_dark : map_light
})
</script>

<template>
  <div class="album-upload-item" :class="{ open: open, 'has-error': data.error }" :draggable="open ? false : true">
    <div class="album-upload-item-header" @click.self="open = !open">
      <button class="hover-bubble bubble-info">
        <span class="material-icons">&#xe945;</span>
      </button>

      <button class="hover-bubble bubble-orange" @click="open = !open" :class="{ active: open }">
        <span class="material-icons">&#xe745;</span>
      </button>

      <strong>{{ form.fileName.length > 0 ? form.fileName : data.name }}</strong>

      <span class="file-size">{{ isPhone ? "" : "Size:" }} {{ size() }}</span>

      <span class="tag tag-blue">{{ isPhone ? "Cover" : "Album Cover" }}</span>

      <div class="flex-1"></div>
      <p v-if="data.error">{{ data.error.message }}</p>

      <LoadingBar :class="[{ 'loading-done': !data.loading }, data.error ? 'loading-error' : 'loading-success']" />

      <button data-title-top="Delete" @click="emit('remove', index)">
        <span class="material-icons">&#xe5cd;</span>
      </button>
    </div>

    <div class="album-upload-content" v-if="open && !data.error">
      <LoadingSpin dark v-if="data.loading" />

      <div class="grid-view" v-else>
        <form @submit.prevent="submit">
          <h6>Edit metadata</h6>

          <InputText
            label="Name*"
            placeholder="Change image name"
            v-model:value="form.fileName"
            :error="errors.fileName"
          />
          <InputTextarea label="Description" placeholder="Add image description" v-model:value="form.description" />

          <div class="map-title">
            <InputCheckbox v-model:check="usemap" />
            <h6>Location</h6>
          </div>

          <div class="map-wrapper" v-if="usemap">
            <mapbox-map
              :accessToken="map_access"
              :mapStyle="mapStyle"
              @loaded="MapLoaded"
              :center="[form.location.longitude, form.location.latitude]"
            >
              <mapbox-marker :lngLat="[form.location.longitude, form.location.latitude]" />
            </mapbox-map>
          </div>

          <div class="buttons">
            <Button
              class="btn-black"
              type="submit"
              @click.prevent="submit"
              :class="{ 'btn-disabled': getLoading(data.key) }"
            >
              Save
              <LoadingSpin v-if="getLoading(data.key)" />
            </Button>

            <Button @click.prevent="emit('setAsCover', data.key)" class="btn-white">Set as album cover</Button>
          </div>
        </form>

        <div>
          <h6>Preview</h6>
          <img :src="imageUrl(data.key, 'medium')" alt=" " />
          <!-- add preview? -->
        </div>
      </div>
    </div>
  </div>
</template>
