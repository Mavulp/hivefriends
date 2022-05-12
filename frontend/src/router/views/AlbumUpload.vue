<script setup lang="ts">
import InputText from "../../components/form/InputText.vue"
import InputTextarea from "../../components/form/InputTextarea.vue"
import ImageUploadItem from "../../components/upload/ImageUploadItem.vue"

import { onBeforeUnmount, onMounted, reactive, ref, computed } from "vue"
import Button from "../../components/Button.vue"
import { upload } from "../../js/fetch"

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

interface Album {
  title: string
  description?: string
  locations?: Array<string>
  timeframe: {
    from: number
    to: number
  }
  imageKeys: Array<string>
  userKeys: Array<string>
}

const files = reactive<File>({ values: [] })
const meta = reactive<Album>({
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

// TODO: Compute global loading progress by
// checking loading states of all files and add a loading bar / percentagle somewhere in the upload
// TODO: add option to select 1 day
// TODO: add multiple locations

const draggingOver = ref(false)
const dragShrink = ref(false)

const isLoading = computed(() => files.values.some((file) => file.loading))

/**
 * Lifecycle
 */

onMounted(() => {
  const el = document.getElementById("drop-area")

  if (el) {
    el.addEventListener("dragenter", uploadHandler, false)
    el.addEventListener("dragleave", uploadHandler, false)
    el.addEventListener("dragover", uploadHandler, false)
    el.addEventListener("drop", uploadHandler, false)
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

function uploadHandler(e: any) {
  e.preventDefault()
  e.stopPropagation()

  let files = e.dataTransfer.files

  if (files.length > 0) {
    fileHandler(files)
  }
}

function fileHandler(_files: any) {
  for (let i = 0; i <= _files.length; i++) {
    const file = _files[i]

    if (!file) return

    let formData = new FormData()
    formData.append("file", file)

    files.values.push({
      name: file.name,
      size: file.size,
      type: file.type,
      loading: true,
      key: null
    })

    upload("/api/images", formData)
      .then((response: any) => {
        console.log("upload done", response)

        Object.assign(files.values[i], {
          loading: false,
          key: response.key
        })
      })
      .catch((error) => {
        Object.assign(files.values[i], {
          loading: false,
          error
        })
      })
  }
}

function delImage(index: number) {
  if (files.values[index]) {
    files.values.splice(index, 1)
  }
}

async function submit() {
  // Validate form
  // Iterate over all active images, add them to imageKeys array
  //
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
            <span class="material-icons">&#xe9a3;</span>
            <span>{{ draggingOver ? "Drop the files!" : "Cllick me / Drag files over here" }}</span>
          </label>
        </div>

        <div class="album-upload-items-list">
          <template v-if="files.values.length > 0">
            <ImageUploadItem v-for="(item, index) in files.values" :data="item" :index="index" @remove="delImage" />
          </template>
        </div>
      </div>

      <div class="album-upload-metadata">
        <h3>Create an album</h3>

        <InputText v-model:value="meta.title" placeholder="That time in Finland" label="Title" />
        <InputTextarea v-model:value="meta.description" placeholder="How was it?" label="Description" />

        <h6>Event Dates</h6>
        <div class="form-date">
          <div class="form-input">
            <input type="date" v-model="meta.timeframe.from" />
            <label>Start</label>
          </div>

          <div class="form-input">
            <input type="date" v-model="meta.timeframe.to" />
            <label>End</label>
          </div>
        </div>

        <Button :class="{ 'btn-disabled': files.values.length === 0 || isLoading }" style="width: 100%" @click="submit"
          >Upload</Button
        >

        <pre style="padding-top: 32px">
          {{ meta }}
        </pre>
      </div>
    </div>
  </div>
</template>
