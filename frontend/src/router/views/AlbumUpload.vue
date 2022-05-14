<script setup lang="ts">
import InputText from "../../components/form/InputText.vue"
import InputTextarea from "../../components/form/InputTextarea.vue"
import ImageUploadItem from "../../components/upload/ImageUploadItem.vue"
import Button from "../../components/Button.vue"
import LoadingSpin from "../../components/loading/LoadingSpin.vue"

import { onBeforeUnmount, onMounted, reactive, ref, computed } from "vue"
import { post, upload } from "../../js/fetch"
import { useFormValidation, required } from "../../js/validation"
import { useAlbums, NewAlbum } from "../../store/album"
import { clone } from "lodash"
import { delay } from "../../js/utils"

const store = useAlbums()

/**
 * Interface & setup
 */

interface File {
  values: Array<{
    name: string
    type: string
    size: number
    loading: boolean
    error?: string
    key: string | null
  }>
}

const files = reactive<File>({ values: [] })
const album = reactive<NewAlbum>({
  title: "",
  description: "",
  locations: [],
  timeframe: {
    from: 0,
    to: 0
  },
  imageKeys: [],
  userKeys: []
})

// If album was successfuly generated, this will get populated
const albumKey = ref()

// TODO: Compute global loading progress by
// checking loading states of all files and add a loading bar / percentagle somewhere in the upload
// TODO: add option to select 1 day
// TODO: add multiple locations

const draggingOver = ref(false)
const dragShrink = ref(false)

const isLoading = computed(() => files.values.some((file) => file.loading))
const imageKeys = computed<Array<any>>(() => files.values.map((file) => file.key))

/**
 * Lifecycle
 */

onMounted(() => {
  const el = document.getElementById("drop-area")

  if (el) {
    el.addEventListener("dragenter", onSubmitHandler, false)
    el.addEventListener("dragleave", onSubmitHandler, false)
    el.addEventListener("dragover", onSubmitHandler, false)
    el.addEventListener("drop", onSubmitHandler, false)
    el.addEventListener("input", (e) => onSubmitHandler(e, true), false)
  }

  document.addEventListener("scroll", handleScroll, { passive: true })
})

onBeforeUnmount(() => {
  document.removeEventListener("scroll", handleScroll)
})

function handleScroll() {
  if (window.scrollY > 96) {
    dragShrink.value = true
  } else {
    dragShrink.value = false
  }
}

/**
 * File Handling
 */

function onSubmitHandler(e: any, fromField: boolean = false) {
  e.preventDefault()
  e.stopPropagation()

  let files = fromField ? e.target.files : e.dataTransfer.files

  if (files.length > 0) {
    uploadFiles(files)
  }
}

async function uploadFiles(_files: any) {
  for (let i = 0; i <= _files.length; i++) {
    const file = _files[i]

    if (!file) continue

    let formData = new FormData()
    formData.append("file", file)

    await delay(5)

    uploadFile(file, formData, clone(i))
  }
}

async function uploadFile(file: any, formData: any, index: number) {
  files.values.push({
    name: file.name,
    size: file.size,
    type: file.type,
    loading: true,
    key: null
  })

  return upload("/api/images/", formData)
    .then((response: any) => {
      Object.assign(files.values[index], {
        loading: false,
        key: response.key
      })

      console.log("ok upload", response, files.values[index])
    })
    .catch((error) => {
      Object.assign(files.values[index], {
        loading: false,
        error
      })

      console.log("erorr upload", error, files.values[index])
    })
}

// Spent an extended weekend hanging out and discovering Switzerland with my buddy. Did some tortellini tasting, some mountain climbing and some music making. Lot's of good meemories!!

function delImage(index: number) {
  if (files.values[index]) {
    files.values.splice(index, 1)
  }
}

const rules = computed(() => ({
  title: { required }
}))

const { validate, errors } = useFormValidation(album, rules)

async function submit() {
  validate().then(async () => {
    // Iterate over all active images, add them to imageKeys array
    album.imageKeys = imageKeys.value

    const model = { ...album }

    Object.assign(model, {
      locations: album.locations?.join(","),
      timeframe: {
        from: new Date(album.timeframe.from).getTime() / 1000,
        to: new Date(album.timeframe.to).getTime() / 1000
      }
    })

    const { key } = await store.addAlbum(model)

    if (key) {
      // was ok
      albumKey.value = key
    }
  })
}
</script>

<template>
  <div class="hi-album-upload">
    <div class="album-upload-layout">
      <div class="album-upload-items">
        <div
          class="album-drag-input"
          id="drop-area"
          @dragenter="draggingOver = true"
          @mouseleave="draggingOver = false"
          :class="{ hovering: draggingOver, shrink: dragShrink }"
        >
          <input id="draginput" name="draginput" type="file" multiple accept="image/*" />
          <label for="draginput">
            <span class="material-icons">&#xe2cc;</span>
            <span>{{ draggingOver ? "Drop the files!" : "Cllick me / Drag files over here" }}</span>
          </label>
        </div>

        <div class="album-upload-items-list">
          <template v-if="files.values.length > 0">
            <ImageUploadItem
              v-for="(item, index) in files.values"
              :data="item"
              :key="item.name"
              :index="index"
              @remove="delImage"
            />
          </template>
        </div>
      </div>

      <div class="album-upload-metadata">
        <h3>Create an album</h3>

        <InputText v-model:value="album.title" placeholder="That time in Finland" label="Title" :error="errors.title" />
        <InputTextarea v-model:value="album.description" placeholder="How was it?" label="Description" />

        <h6>Event Dates</h6>
        <div class="form-date">
          <div class="form-input">
            <input type="date" v-model="album.timeframe.from" />
            <label>Start</label>
          </div>

          <div class="form-input">
            <input type="date" v-model="album.timeframe.to" />
            <label>End</label>
          </div>
        </div>

        <Button
          class="btn-icon"
          :class="{ 'btn-disabled': files.values.length === 0 || isLoading }"
          style="width: 100%"
          @click="submit"
        >
          Upload
          <LoadingSpin v-if="isLoading" />
        </Button>

        <Button class="btn-blue mt-20 btn-icon" v-if="albumKey" :to="{ name: 'AlbumDetail', params: { id: albumKey } }">
          View Album
          <span class="material-icons"> &#xe941; </span>
        </Button>
      </div>
    </div>
  </div>
</template>
