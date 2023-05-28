<script setup lang="ts">
import { computed, inject, onMounted, ref, watch } from 'vue'
import { onClickOutside, useClipboard, useMagicKeys, whenever } from '@vueuse/core'
import { useRouter } from 'vue-router'
import type { Album, ImageItemInAlbum } from '../../store/album'
import { imageUrl } from '../../store/album'
import { get } from '../../js/fetch'
import { useToast } from '../../store/toast'
import { useUser } from '../../store/user'
import { formatDate } from '../../js/utils'

import LoadingSpin from '../loading/LoadingSpin.vue'
import InputCheckbox from '../form/InputCheckbox.vue'
import Modal from '../../components/Modal.vue'

interface Props {
  image: ImageItemInAlbum
  mode: boolean
  isSelect: boolean
  index: number
}

const props = defineProps<Props>()
const emit = defineEmits<{
  (e: 'select', value: ImageItemInAlbum): void
}>()
const toast = useToast()
const user = useUser()
const router = useRouter()
const open = ref(false)
const hover = ref(false)
const { copy } = useClipboard()
const wrap = ref()

const selectingAlbum = ref(false)
const selectingLoading = ref(false)
const albums = ref<Array<Album>>()

const inAlbum = computed(() => props.image.albumKeys.length > 0)

onMounted(() => {
  onClickOutside(wrap, () => (open.value = false))
})

// @ts-expect-error idk
const { imgIndex, setIndex } = inject<{ imgIndex: number; setIndex: (num: number) => void }>('image-index')
const total = inject<number>('image-total')

function imageClick() {
  if (props.mode) {
    // Select
    emit('select', props.image)
  }
  else {
    open.value = true
    setIndex(props.index)
  }
}

function copyImage() {
  copy(imageUrl(props.image.key))
  toast.add('Image url copied to clipboard')
}

/**
 * Set as avatar / banner
 */
const loading = ref(false)

async function setAs(key: string) {
  loading.value = true
  await user.setSetting(key, props.image.key)
  loading.value = false

  toast.add(`Updated ${key === 'avatarKey' ? 'avatar' : 'banner'} image`)
}

const keys = useMagicKeys()
whenever(keys.Escape, () => {
  open.value = false
  selectingAlbum.value = false
})

/**
 * Go to image album
 */

async function tryToAlbum() {
  if (props.image.albumKeys.length === 1) {
    router.push({ name: 'AlbumDetail', params: { id: props.image.albumKeys[0] } })
  }
  else {
    selectingLoading.value = true
    selectingAlbum.value = true

    Promise.all(props.image.albumKeys.map((key: string) => get(`/api/albums/${key}`))).then((response) => {
      albums.value = response
      selectingLoading.value = false
    })
  }
}

watch(imgIndex, (value) => {
  if (value !== props.index)
    open.value = false
  else
    open.value = true
})
</script>

<template>
  <div
    class="hi-album-image"
    :class="{ 'is-selected': props.isSelect }"
    @mouseenter="hover = true"
    @mouseleave="hover = false"
  >
    <div v-if="mode" class="all-image-checkbox">
      <InputCheckbox :check="props.isSelect" @update:check="emit('select', props.image)" />
    </div>

    <div class="image-wrap" @click="imageClick">
      <img :src="imageUrl(props.image.key, 'tiny')" alt="">
    </div>

    <Teleport v-if="selectingAlbum" to="body">
      <Modal @click="selectingAlbum = false">
        <div class="modal-wrap modal-select-album">
          <button class="btn-close" data-title-left="Close" @click="open = false">
            <span class="material-icons">&#xe5cd;</span>
          </button>

          <h2>Select an album</h2>
          <p>This image is part of multiple albums. Please choose which one you wish to view.</p>

          <LoadingSpin v-if="selectingLoading" />

          <template v-else>
            <router-link
              v-for="album in albums"
              :key="album.key"
              :to="{ name: 'AlbumDetail', params: { id: album.key } }"
              class="select-album-item"
            >
              <div class="album-item-image">
                <img :src="imageUrl(album.coverKey, 'tiny')" alt="">
              </div>

              <div class="album-item-meta">
                <strong>{{ album.title }}</strong>
                <p>
                  Uploaded {{ formatDate(album.publishedAt) }} by
                  {{ album.author }}
                </p>
              </div>
            </router-link>
          </template>
        </div>
      </Modal>
    </Teleport>

    <Teleport v-if="open" to="body">
      <Modal class="modal-images">
        <img :src="imageUrl(props.image.key)" alt="">

        <div class="modal-top">
          <div class="left">
            <button class="btn-close hover-bubble" @click="open = false">
              <span class="material-icons">&#xe5cd;</span>
            </button>
          </div>

          <div class="center">
            <button v-if="inAlbum" class="hover-bubble bubble-small" data-title-top-left="Go to album" @click="tryToAlbum">
              <span class="material-icons"> &#xe89e; </span>
              Album
            </button>
            <button class="hover-bubble bubble-small" data-title-top="Share link" @click="copyImage">
              <span class="material-icons"> &#xe80d; </span>
              Share
            </button>
            <button class="hover-bubble bubble-small" data-title-top="Use as avatar" :class="{ 'btn-disabled': loading }" @click="setAs('avatarKey')">
              <span class="material-icons"> &#xe853; </span>
              Use as Avatar
            </button>
            <button class="hover-bubble bubble-small" data-title-top="Use as banner" :class="{ 'btn-disabled': loading }" @click="setAs('bannerKey')">
              <span class="material-icons"> &#xe40b; </span>
              Use as Banner
            </button>
          </div>
          <div class="all-image-controls">
            <button :disabled="imgIndex <= 0" class="nav-btn btn-prev" @click="setIndex(props.index - 1)">
              <span class="material-icons"> &#xf1e6; </span>
              <!-- <img src="/icons/arrow-left-long.svg" alt=" "> -->
            </button>

            <p class="img-index">
              <b>{{ props.index + 1 }}</b> of {{ total }}
            </p>

            <button :disabled="imgIndex + 1 === total" class="nav-btn btn-next" @click="setIndex(props.index + 1)">
              <span class="material-icons"> &#xf1df; </span>
              <!-- <img src="/icons/arrow-right-long.svg" alt=" "> -->
            </button>
          </div>
        </div>

        <!--
          <div class="all-image-controls">
            <button data-title-bottom="Go to album" @click="tryToAlbum" v-if="inAlbum">
              <span class="material-icons"> &#xe89e; </span>
            </button>
            <button data-title-bottom="Share link" @click="copyImage">
              <span class="material-icons"> &#xe80d; </span>
            </button>
            <button data-title-bottom="Use as avatar" @click="setAs('avatarKey')" :class="{ 'btn-disabled': loading }">
              <span class="material-icons"> &#xe853; </span>
            </button>
            <button data-title-bottom="Use as banner" @click="setAs('bannerKey')" :class="{ 'btn-disabled': loading }">
              <span class="material-icons"> &#xe40b; </span>
            </button>
          </div> -->
      </Modal>
    </Teleport>
  </div>
</template>
