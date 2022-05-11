<script setup lang="ts">
import InputText from "../../components/form/InputText.vue"
import InputTextarea from "../../components/form/InputTextarea.vue"
import ImageUploadItem from "../../components/upload/ImageUploadItem.vue"

import { onMounted, reactive, ref } from "vue"
import Button from "../../components/Button.vue"
import { rootUrl } from "../../js/fetch"

// TODO: type

const draggingOver = ref(false)

interface File {
  values: Array<{
    name: string
    type: string
    size: number
    loading: boolean
    error?: string
  }>
}

// const files = ref<Array<File>>([])

// TODO: move these files into the album.store so that they can be accessed globally
const files = reactive<File>({ values: [] })

const meta = reactive({
  albumName: "",
  albumDescription: "",
  location: "",
  timeframe: {
    from: "",
    to: ""
  }
})

onMounted(() => {
  const el = document.getElementById("drop-area")

  if (el) {
    el.addEventListener("dragenter", uploadHandler, false)
    el.addEventListener("dragleave", uploadHandler, false)
    el.addEventListener("dragover", uploadHandler, false)
    el.addEventListener("drop", uploadHandler, false)
  }
})

function uploadHandler(e: any) {
  e.preventDefault()
  e.stopPropagation()

  let files = e.dataTransfer.files

  if (files.length > 0) {
    fileHandler(files)
  }
}

function fileHandler(_files: any) {
  // let i = 0

  console.log(_files.length)

  for (let i = 0; i <= _files.length; i++) {
    const file = _files[i]

    if (!file) return

    let formData = new FormData()
    formData.append("file", file)

    // let reader = new FileReader()
    // reader.readAsDataURL(file)

    console.log(file)

    files.values.push({
      name: file.name,
      size: file.size,
      type: file.type,
      loading: true
    })

    console.log(files.values)

    fetch(rootUrl + "/api/images", {
      method: "POST",
      body: formData
    }).then((response) => {
      console.log("upload done", response)

      files.values[i].loading = false

      if (response.status !== 200) {
        files.values[i].error = response.statusText
      }
    })
  }
}

function submit() {}
</script>

<template>
  <div class="hi-album-upload">
    <div class="album-upload-layout">
      <div class="album-upload-items">
        <!-- <form action=""> -->
        <div
          class="album-drag-input"
          id="drop-area"
          @dragenter="draggingOver = true"
          @dragleave="draggingOver = false"
          :class="{ hovering: draggingOver }"
        >
          <input id="draginput" name="draginput" type="file" multiple accept="image/*" />
          <label for="draginput">
            <span class="material-icons">&#xe9a3;</span>
            <span>{{ draggingOver ? "Drop the files!" : "Cllick me / Drag files over here" }}</span>
          </label>
        </div>
        <!-- </form> -->

        <div class="album-upload-items-list">
          <template v-if="files.values.length > 0">
            <ImageUploadItem v-for="item in files.values" :data="item" />
          </template>
        </div>
      </div>

      <div class="album-upload-metadata">
        <h3>Create an album</h3>

        <InputText v-model:value="meta.albumName" placeholder="That time in Finland" label="Name" />
        <InputTextarea v-model:value="meta.albumDescription" placeholder="How was it?" label="Description" />

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

        <Button style="width: 100%" @click="submit">Upload</Button>
      </div>
    </div>
  </div>
</template>
