<script setup lang="ts">
import { ref, onBeforeMount, watch, computed } from "vue"
import { getImageChunks } from "../../js/_composables"
import { Image, useAlbums } from "../../store/album"
import { useBread } from "../../store/bread"
import { useLoading } from "../../store/loading"
import { upload } from "../../js/fetch"
import { useToast } from "../../store/toast"
import { FetchError } from "../../js/global-types"
import { useUser } from "../../store/user"

import UserImageItem from "../image/UserImageItem.vue"
import LoadingSpin from "../loading/LoadingSpin.vue"
import router from "../../router"

const bread = useBread()
const toast = useToast()
const user = useUser()
const album = useAlbums()
const { getLoading, addLoading, delLoading } = useLoading()
const data = ref<Array<Image>>([])

const chunks = computed<Array<Array<Image>>>(() =>
  getImageChunks(
    data.value.sort((a, b) => (a.uploadedAt > b.uploadedAt ? -1 : 1)),
    5
  )
)

onBeforeMount(async () => {
  bread.set("All your uploaded images")
  const raw = await album.fetchUserImages()
  data.value = raw.filter((item: Image) => item.key !== user.settings.avatarKey && item.key !== user.settings.bannerKey)
})

// const imagesInAlbums = computed(() => data.value?.filter(item => item.albumKey).length)
const imagesInAlbums = 74

async function uploadImage(e: any) {
  if (!e) return

  addLoading("upload")

  e.preventDefault()
  e.stopPropagation()

  const file = new FormData()
  file.append("file", e.target.files[0])

  return upload("/api/images/", file)
    .then((response: any) => data.value.unshift(response))
    .catch((error: FetchError) => toast.add(error.message, "error"))
    .finally(() => delLoading("upload"))
}

/**
 * Image selecting
 */

const selected = ref(new Map())
const selectMode = ref(false)

watch(selected, (value) => {
  console.log(value)
})

function selectItem(item: Image) {
  if (selected.value.has(item.key)) {
    selected.value.delete(item.key)
  } else {
    selected.value.set(item.key, item)
  }
}

function clearSelect() {
  selected.value = new Map()
  selectMode.value = false
}
function createSelect() {
  router.push({
    name: "Upload",
    params: {
      images: JSON.stringify([...selected.value.values()])
    }
  })
}
function deleteSelect() {}
</script>

<template>
  <div class="hi-image-list">
    <div class="hi-image-list-upload">
      <h1>All photos</h1>
      <p>
        List of every uploaded photo by you. It is not intended to use the site like that, but you can optionally upload
        a few images which live outside of any albums. To store them or quickly share with someone.
      </p>

      <input type="file" name="imgfile" id="imgfile" accept="image/*" @input="uploadImage" />
      <label for="imgfile">
        <span class="material-icons"> &#xe3f4; </span>
        <span>Upload an image</span>

        <div class="flex-1"></div>

        <LoadingSpin v-if="getLoading('upload')" class="dark small" />
      </label>
    </div>

    <div class="hi-image-list-info">
      <div>
        <p>Sorting by upload date</p>
      </div>

      <div>
        <p>{{ data?.length }} photos</p>
        <p>{{ imagesInAlbums }} in albums</p>
        <p v-if="selected.size > 0">
          <b>{{ selected.size }} {{ selected.size === 1 ? "photo" : "photos" }} selected</b>
        </p>
      </div>

      <div>
        <template v-if="selected.size > 0 && selectMode">
          <button class="hover-bubble bubble-red" @click="deleteSelect">
            <span class="material-icons">&#xe872;</span>
            Delete selected
          </button>

          <button class="hover-bubble bubble-orange" @click="createSelect">
            <span class="material-icons">&#xe2cc;</span>
            Create album
          </button>

          <button class="hover-bubble bubble-highlight" @click="clearSelect">
            <span class="material-icons">&#xe5cd;</span>
            Clear selection
          </button>
        </template>

        <button v-else class="hover-bubble bubble-highlight" @click="selectMode = !selectMode">
          <span class="material-icons" v-if="selectMode"> &#xe5cd; </span>
          <span class="material-icons" v-else> &#xe03c;</span>
          {{ selectMode ? "Cancel" : "Select" }}
        </button>
      </div>
    </div>

    <div class="hi-album-images">
      <div class="hi-album-image-col" v-for="(chunk, index) in chunks" :key="index">
        <UserImageItem
          v-for="image in chunk"
          :image="image"
          :key="image.key"
          @select="selectItem"
          :isSelect="selected.has(image.key)"
          :mode="selectMode"
        />
      </div>
    </div>
  </div>
</template>