<script setup lang="ts">
import { onMounted, reactive, ref } from "vue"
import { useToast } from "../../../store/toast"
import { upload } from "../../../js/fetch"
import { FetchError } from "../../../js/global-types"
// import { imageUrl } from "../../../store/album";
import { useUser } from "../../../store/user"

import LoadingSpin from "../../loading/LoadingSpin.vue"
import Button from "../../Button.vue"

/**
 *  Setup
 */

const toast = useToast()
const user = useUser()

const { field } = defineProps<{
  field: string
}>()

interface DataHandling {
  loading: boolean
  key: string | null
  file: File
}

const data = reactive<DataHandling>({} as DataHandling)

/**
 * Events
 */

const droparea = ref<HTMLElement>()
const dragging = ref(false)
const preview = ref<HTMLImageElement>()

onMounted(() => {
  if (droparea.value) {
    droparea.value.addEventListener("dragenter", previewFile, false)
    droparea.value.addEventListener("dragleave", previewFile, false)
    droparea.value.addEventListener("dragover", previewFile, false)
    droparea.value.addEventListener("drop", previewFile, false)
    droparea.value.addEventListener("input", previewFile, false)
  }
})

/**
 * File Handling
 */

function previewFile(e: any) {
  e.preventDefault()
  e.stopPropagation()

  // Get the first file (shouldn't even have more)
  const [file] = e.dataTransfer?.files ?? e.target?.files

  if (file) {
    // Save file in case it should get uploaded
    data.file = file

    console.log(data.file, preview.value)

    if (preview.value) preview.value.src = URL.createObjectURL(file)
  }
}

function clearFile() {
  Object.assign(data, { loading: false, key: null, file: null })
  if (preview.value) preview.value.src = ""
}

async function uploadImage() {
  data.loading = true

  let formData = new FormData()
  formData.append("file", data.file)

  upload("/api/images/", formData)
    .then((response) => {
      if (response.key) {
        user.setSetting(field, response.key)
        toast.add(`Successfuly uploaded a new ${field === "avatarKey" ? "avatar" : "banner"}`, "success")

        setTimeout(() => {
          clearFile()
        }, 500)
      }
    })
    .catch((error: FetchError) => {
      toast.add(`${field === "avatarKey" ? "Avatar" : "Banner"} upload error: ${error.message}`, "error")
    })
    .finally(() => {
      data.loading = false
    })
}
</script>

<template>
  <div class="settings-upload-image">
    <div
      class="drop-section"
      :class="{ dragging }"
      ref="droparea"
      @dragenter="dragging = true"
      @dragleave="dragging = false"
      @mouseleave="dragging = false"
    >
      <input :id="field" :name="field" type="file" accept="image/*" />
      <label :for="field">
        <span class="material-icons">&#xe439;</span>
        <span>{{
          data.file ? "Upload a different one" : `Upload a new ${field === "avatarKey" ? "avatar" : "banner"}`
        }}</span>
      </label>
    </div>
    <transition name="fade" appear>
      <div class="preview-section" v-show="data.file">
        <button class="remove-image" @click="clearFile" data-title-left="Remove">
          <span class="material-icons">&#xe5cd;</span>
        </button>

        <img ref="preview" src="" alt="" />

        <Button
          :class="{ 'btn-disabled': data.loading }"
          class="btn-icon btn-black"
          style="width: 100%"
          @click="uploadImage"
        >
          Save
          <LoadingSpin v-if="data.loading" />
        </Button>
      </div>
    </transition>
  </div>
</template>
