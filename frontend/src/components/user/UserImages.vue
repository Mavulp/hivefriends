<script setup lang="ts">
import { ref, onBeforeMount, watch, computed } from "vue"
import { getImageChunks } from "../../js/_composables"
import { AllImageItem, Image, useAlbums, imageUrl, Album } from "../../store/album"
import { useBread } from "../../store/bread"
import { useLoading } from "../../store/loading"
import { upload } from "../../js/fetch"
import { useToast } from "../../store/toast"
import { FetchError } from "../../js/global-types"
import { useUser } from "../../store/user"
import { formatDate } from "../../js/utils"
import { isEmpty } from "lodash"

import UserImageItem from "../image/UserImageItem.vue"
import LoadingSpin from "../loading/LoadingSpin.vue"
import Modal from "../Modal.vue"
import { onClickOutside, useMediaQuery } from "@vueuse/core"
import { useRouter } from "vue-router"

const bread = useBread()
const toast = useToast()
const user = useUser()
const album = useAlbums()
const { getLoading, addLoading, delLoading } = useLoading()
const data = ref<Array<AllImageItem>>([])
const isPhone = useMediaQuery("(max-width: 512px)")
const router = useRouter()

const chunks = computed<Array<Array<AllImageItem>>>(() =>
  getImageChunks(
    data.value.sort((a, b) => (a.uploadedAt > b.uploadedAt ? -1 : 1)),
    isPhone.value ? 3 : 5
  )
)

onBeforeMount(async () => {
  bread.set("All your uploaded photos")
  const raw = await album.fetchUserImages()
  data.value = raw.filter(
    (item: AllImageItem) => item.key !== user.settings.avatarKey && item.key !== user.settings.bannerKey
  )
})

const imagesInAlbums = computed(() => data.value?.filter((item: AllImageItem) => item.albumKeys.length > 0).length)

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

// Dropdown options

const open = ref(false)
const wrap = ref(null)

onClickOutside(wrap, () => (open.value = false))

// Delete all selected images
async function deleteSelect() {
  album.deleteImages([...selected.value.keys()]).finally(async () => {
    const raw = await album.fetchUserImages()

    data.value = raw.filter(
      (item: AllImageItem) => item.key !== user.settings.avatarKey && item.key !== user.settings.bannerKey
    )

    clearSelect()
  })
}

// Create new album with selected images
function createSelect() {
  router.push({
    name: "Upload",
    params: {
      images: JSON.stringify([...selected.value.values()])
    }
  })
}

/**
 * Add selected images to an existing album
 */

const modal = ref(false)

const selectingLoading = ref(true)
const albums = ref<Array<Album>>()

// Open modal to select an album to add selected images to
async function tryToAlbum() {
  modal.value = true

  // albums.value = await album.fetchUserAlbums(user.user.username, true)
  albums.value = (await Promise.all([
    album.fetchUserAlbums(user.user.username),
    album.fetchUserAlbums(user.user.username, true)
  ]).then((res: Album[]) => res.flat())) as Album[]

  setTimeout(() => {
    selectingLoading.value = false
  }, 25)
}

// Open album edit page with selected images
</script>

<template>
  <div class="hi-image-list">
    <div class="hi-image-list-upload">
      <h1>All photos</h1>
      <p>
        List of every uploaded photo by you. It is not intended to use the site like that, but you can optionally upload
        a few photos which live outside of any albums. To store them or quickly share with someone.
      </p>

      <input type="file" name="imgfile" id="imgfile" accept="image/*, .heic" @input="uploadImage" />
      <label for="imgfile">
        <span class="material-icons"> &#xe3f4; </span>
        <span>Upload a photo</span>

        <div class="flex-1"></div>

        <LoadingSpin v-if="getLoading('upload')" class="dark small" />
      </label>
    </div>

    <LoadingSpin class="dark center-page" v-if="getLoading('images')" />

    <div v-else class="hi-image-list-info" :class="{ 'is-selecting': selected.size > 0 }">
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
          <div ref="wrap" style="position: relative">
            <button class="hover-bubble" @click="open = !open" :class="{ active: open }">
              <span class="material-icons">&#xe5d2;</span>
              Actions
            </button>

            <div class="dropdown-list" :class="{ active: open }">
              <button class="hover-bubble bubble-info" @click="createSelect()">
                <span class="material-icons">&#xe2cc;</span> Create album
              </button>
              <button class="hover-bubble bubble-info" @click="tryToAlbum()">
                <span class="material-icons">&#xe9a3;</span> Add to album
              </button>
              <button
                class="hover-bubble bubble-red"
                :class="{ 'btn-disabled': getLoading('delete-album') }"
                @click="deleteSelect()"
              >
                <span class="material-icons"> &#xe872; </span>
                Delete
                <span class="material-icons rotate" v-if="getLoading('delete-album')">&#xe863;</span>
              </button>
            </div>
          </div>

          <button class="hover-bubble bubble-highlight" @click="clearSelect">
            <span class="material-icons">&#xe5cd;</span>
            Deselect
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
    <Teleport to="body" v-if="modal">
      <Modal @click="modal = false">
        <div class="modal-wrap modal-select-album">
          <button class="btn-close" data-title-left="Close" @click="open = false">
            <span class="material-icons">&#xe5cd;</span>
          </button>

          <h2>Select an album</h2>
          <p>Please choose an album to add the selected {{ selected.size === 1 ? "image" : "images" }} to.</p>

          <LoadingSpin v-if="selectingLoading" class="dark small" />

          <template v-else-if="albums && !isEmpty(albums)" class="dark small">
            <router-link
              v-for="album in albums"
              class="select-album-item"
              :to="{
                name: 'AlbumEdit',
                params: { id: album.key, images: JSON.stringify([...selected.values()]) }
              }"
            >
              <div class="album-item-image">
                <img :src="imageUrl(album.coverKey, 'tiny')" alt="" />
              </div>

              <div class="album-item-meta">
                <strong>{{ album.title }}</strong>
                <p>Uploaded {{ formatDate(album.createdAt) }}</p>
              </div>
            </router-link>
          </template>

          <p v-else>You have no albums. <router-link :to="{ name: 'Upload' }">Uplaod one.</router-link></p>
        </div>
      </Modal>
    </Teleport>
  </div>
</template>
