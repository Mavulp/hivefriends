<script setup lang="ts">
import LoadingBar from "../loading/LoadingBar.vue"
import Button from "../Button.vue"
import InputText from "../form/InputText.vue"
import InputTextarea from "../form/InputTextarea.vue"
import LoadingSpin from "../../components/loading/LoadingSpin.vue"

import { computed, reactive, ref, watch } from "vue"
import { useAlbums, imageUrl } from "../../store/album"
import { minLength, useFormValidation, required } from "../../js/validation"
import { useLoading } from "../../store/loading"

interface Props {
  data: any
  index: number
}

const { data, index } = defineProps<Props>()

const emit = defineEmits<{
  (e: "remove", index: number): void
  (e: "setAsCover", key: string): void
}>()

const albums = useAlbums()
const { getLoading } = useLoading()

const open = ref(false)
const size = () => {
  if (data.size / 1000000 > 1) return data.size / 1000000 + "MB"
  return data.size / 1000 + "KB"
}

const _key = computed(() => data.key)

watch(
  _key,
  async (val) => {
    if (val) {
      const image = await albums.fetchImageMetadata(data.key)

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
    minLength: minLength(3)
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
</script>

<template>
  <div class="album-upload-item" :class="{ open: open, 'has-error': data.error }" draggable="true">
    <div class="album-upload-item-header" @click.self="open = !open">
      <button class="hover-bubble bubble-info">
        <span class="material-icons">&#xe945;</span>
      </button>

      <strong>{{ form.fileName.length > 0 ? form.fileName : data.name }}</strong>

      <span class="file-size">Size: {{ size() }}</span>

      <span class="tag tag-blue">Album Cover</span>

      <div class="flex-1"></div>
      <p v-if="data.error">{{ data.error.message }}</p>

      <LoadingBar :class="[{ 'loading-done': !data.loading }, data.error ? 'loading-error' : 'loading-success']" />

      <button data-title-top="Remove Image" @click="emit('remove', index)">
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

          <h6>Location</h6>
          <div class="double-input">
            <InputText label="Latitude" v-model:value="form.location.latitude" placeholder="Set photo latitude" />
            <InputText label="Latitude" v-model:value="form.location.longitude" placeholder="Set photo longitude" />
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
