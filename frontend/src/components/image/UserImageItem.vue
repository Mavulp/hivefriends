<script setup lang="ts">
import { computed, ref } from "vue"
import { Album, AllImageItem, imageUrl } from "../../store/album"
import { useClipboard, useMagicKeys, whenever } from "@vueuse/core"
import { get } from "../../js/fetch"
import { useToast } from "../../store/toast"
import { useUser } from "../../store/user"
import { useRouter } from "vue-router"
import { formatDate } from "../../js/utils"

import LoadingSpin from "../loading/LoadingSpin.vue"
import InputCheckbox from "../form/InputCheckbox.vue"
import Modal from "../../components/Modal.vue"

interface Props {
  image: AllImageItem
  mode: boolean
  isSelect: boolean
}

const toast = useToast()
const user = useUser()
const router = useRouter()
const props = defineProps<Props>()
const emit = defineEmits<{
  (e: "select", value: AllImageItem): void
}>()
const open = ref(false)
const hover = ref(false)
const { copy } = useClipboard()

const inAlbum = computed(() => props.image.albumKeys.length > 0)

function imageClick() {
  if (props.mode) {
    // Select
    emit("select", props.image)
  } else {
    open.value = true
  }
}

function copyImage() {
  copy(imageUrl(props.image.key))
  toast.add("Image url copied to clipboard")
}

/**
 * Set as avatar / banner
 */
const loading = ref(false)

async function setAs(key: string) {
  loading.value = true
  await user.setSetting(key, props.image.key)
  loading.value = false

  toast.add(`Updated ${key === "avatarKey" ? "avatar" : "banner"} image`)
}

const keys = useMagicKeys()
whenever(keys["Escape"], () => {
  open.value = false
})

/**
 * Go to image album
 */

const selectingAlbum = ref(false)
const selectingLoading = ref(false)
const albums = ref<Array<Album>>()

async function tryToAlbum() {
  console.log(props.image)

  if (props.image.albumKeys.length === 1) {
    router.push({ name: "AlbumDetail", params: { id: props.image.albumKeys[0] } })
  } else {
    selectingLoading.value = true
    selectingAlbum.value = true

    Promise.all(props.image.albumKeys.map((key: string) => get(`/api/albums/${key}`))).then((response) => {
      albums.value = response
      selectingLoading.value = false
    })
  }
}

function goto(key: string) {}
</script>

<template>
  <div
    class="hi-album-image"
    @mouseenter="hover = true"
    @mouseleave="hover = false"
    :class="{ 'is-selected': props.isSelect }"
  >
    <div class="all-image-checkbox" v-if="mode">
      <InputCheckbox :check="props.isSelect" @update:check="emit('select', props.image)" />
    </div>

    <div class="all-image-controls" v-show="hover && !mode">
      <button data-title-left="Go to album" @click="tryToAlbum" v-if="inAlbum">
        <span class="material-icons"> &#xe89e; </span>
      </button>
      <button data-title-left="Share link" @click="copyImage">
        <span class="material-icons"> &#xe80d; </span>
      </button>
      <button data-title-left="Use as avatar" @click="setAs('avatarKey')" :class="{ 'btn-disabled': loading }">
        <span class="material-icons"> &#xe853; </span>
      </button>
      <button data-title-left="Use as banner" @click="setAs('bannerKey')" :class="{ 'btn-disabled': loading }">
        <span class="material-icons"> &#xe40b; </span>
      </button>
    </div>

    <div class="image-wrap" @click="imageClick">
      <img :src="imageUrl(props.image.key, 'medium')" alt="" />
    </div>

    <Teleport to="body" v-if="selectingAlbum">
      <Modal @click="selectingAlbum = false">
        <div class="modal-wrap modal-select-album">
          <h2>Select an album</h2>
          <p>This image is part of multiple albums. Please choose which one you wish to view.</p>

          <LoadingSpin v-if="selectingLoading" />

          <template v-else>
            <router-link
              :to="{ name: 'AlbumDetail', params: { id: album.key } }"
              class="select-album-item"
              v-for="album in albums"
            >
              <div class="album-item-image">
                <img :src="imageUrl(album.coverKey, 'tiny')" alt="" />
              </div>

              <div class="album-item-meta">
                <strong>{{ album.title }}</strong>
                <p>
                  Uploaded {{ formatDate(album.createdAt) }} by
                  {{ album.author }}
                </p>
              </div>
            </router-link>
          </template>
        </div>
      </Modal>
    </Teleport>

    <Teleport to="body" v-if="open">
      <Modal @click="open = false">
        <div class="modal-wrap modal-image">
          <div>
            <div class="all-image-controls">
              <button data-title-left="Close" @click="open = false">
                <span class="material-icons">&#xe5cd;</span>
              </button>
              <button data-title-left="Go to album" @click="tryToAlbum" v-if="inAlbum">
                <span class="material-icons"> &#xe89e; </span>
              </button>
              <button data-title-left="Share link" @click="copyImage">
                <span class="material-icons"> &#xe80d; </span>
              </button>
              <button data-title-left="Use as avatar" @click="setAs('avatarKey')" :class="{ 'btn-disabled': loading }">
                <span class="material-icons"> &#xe853; </span>
              </button>
              <button data-title-left="Use as banner" @click="setAs('bannerKey')" :class="{ 'btn-disabled': loading }">
                <span class="material-icons"> &#xe40b; </span>
              </button>
            </div>
            <img :src="imageUrl(props.image.key)" alt="" />
          </div>
        </div>
      </Modal>
    </Teleport>
  </div>
</template>
