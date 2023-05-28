<script setup lang="ts">
import { computed, onBeforeMount, provide, ref } from 'vue'
import { isEmpty } from 'lodash'
import { onClickOutside, useWindowScroll } from '@vueuse/core'
import { useRouter } from 'vue-router'
import type { Album, Image, ImageItemInAlbum } from '../../store/album'
import { imageUrl, useAlbums } from '../../store/album'
import { useBread } from '../../store/bread'
import { useLoading } from '../../store/loading'
import { upload } from '../../js/fetch'
import { useToast } from '../../store/toast'
import type { FetchError } from '../../js/global-types'
import { useUser } from '../../store/user'
import { formatDate } from '../../js/utils'

import UserImageItem from '../image/UserImageItem.vue'
import LoadingSpin from '../loading/LoadingSpin.vue'
import Modal from '../Modal.vue'

const bread = useBread()
const toast = useToast()
const user = useUser()
const album = useAlbums()
const { getLoading, addLoading, delLoading } = useLoading()
const data = ref<Array<ImageItemInAlbum>>([])
const router = useRouter()

const dataByMonths = computed(() => {
  return [...data.value]
    .sort((a, b) => Number(a.uploadedAt < b.uploadedAt))
    .reduce((group, item) => {
      const date = new Date(item.uploadedAt * 1000)
      const key = new Date(date.getUTCFullYear(), date.getUTCMonth(), 1).getTime()

      if (group[key])
        group[key].push(item)
      else
        group[key] = [item]

      return group
    }, {} as Record<number, ImageItemInAlbum[]>)
})

onBeforeMount(async () => {
  bread.set('All your uploaded photos')
  const raw = await album.fetchUserImages()
  data.value = raw.filter(
    (item: ImageItemInAlbum) => item.key !== user.settings.avatarKey && item.key !== user.settings.bannerKey,
  )
})

const imagesInAlbums = computed(() => data.value?.filter((item: ImageItemInAlbum) => item.albumKeys.length > 0).length)

async function uploadImage(e: any) {
  if (!e)
    return

  addLoading('upload')

  e.preventDefault()
  e.stopPropagation()

  const file = new FormData()
  file.append('file', e.target.files[0])

  return upload('/api/images/', file)
    .then((response: any) => data.value.unshift(response))
    .catch((error: FetchError) => toast.add(error.message, 'error'))
    .finally(() => delLoading('upload'))
}

/**
 * Image selecting
 */

const selected = ref(new Map())
const selectMode = ref(false)

function selectItem(item: Image) {
  if (selected.value.has(item.key))
    selected.value.delete(item.key)
  else
    selected.value.set(item.key, item)
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
      (item: ImageItemInAlbum) => item.key !== user.settings.avatarKey && item.key !== user.settings.bannerKey,
    )

    clearSelect()
  })
}

// Create new album with selected images
function createSelect() {
  router.push({
    name: 'Upload',
    params: {
      images: JSON.stringify([...selected.value.values()]),
    },
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
    album.fetchUserAlbums(user.user.username, true),
  ]).then((res: Album[]) => res.flat())) as Album[]

  setTimeout(() => {
    selectingLoading.value = false
  }, 25)
}

// Open album edit page with selected images

/**
 * Browsing images
 */
const imgIndex = ref(0)
function setIndex(val: number) {
  imgIndex.value = val
}

provide('image-index', { imgIndex, setIndex })
provide(
  'image-total',
  computed(() => data.value.length),
)

// Takes in a unix timestamp and returns a MMMM YYYY formatted date
function getGroupDate(timestamp: number) {
  const date = new Date()
  date.setTime(timestamp)

  return date.toLocaleDateString('default', {
    month: 'long',
    year: 'numeric',
  })
}

function scrollUp() {
  window.scrollTo({ top: 0, behavior: 'smooth' })
}

const { y } = useWindowScroll()
const shouldShowButtonUp = computed(() => y.value > 292)
</script>

<template>
  <div class="hi-image-list">
    <div class="hi-image-list-upload">
      <h1>All photos</h1>
      <p>
        List of every uploaded photo by you. It is not intended to use the site like that, but you can optionally upload
        a few photos which live outside of any albums. To store them or quickly share with someone.
      </p>

      <input id="imgfile" type="file" name="imgfile" accept="image/*" @input="uploadImage">
      <label for="imgfile">
        <span class="material-icons"> &#xe3f4; </span>
        <span>Upload a photo</span>

        <div class="flex-1" />

        <LoadingSpin v-if="getLoading('upload')" class="dark small" />
      </label>
    </div>

    <LoadingSpin v-if="getLoading('images')" class="dark center-page" />

    <div v-else class="hi-image-list-info" :class="{ 'is-selecting': selected.size > 0 }">
      <!-- <div>
        <p>Sorting by upload date</p>
      </div> -->

      <div>
        <p>{{ data?.length }} photos</p>
        <p>{{ imagesInAlbums }} in albums</p>
        <p v-if="selected.size > 0">
          <b>{{ selected.size }} {{ selected.size === 1 ? "photo" : "photos" }} selected</b>
        </p>
      </div>

      <div />

      <div>
        <template v-if="selected.size > 0 && selectMode">
          <div ref="wrap" style="position: relative">
            <button class="hover-bubble" :class="{ active: open }" @click="open = !open">
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
                <span v-if="getLoading('delete-album')" class="material-icons rotate">&#xe863;</span>
              </button>
            </div>
          </div>

          <button class="hover-bubble bubble-highlight" @click="clearSelect">
            <span class="material-icons">&#xe5cd;</span>
            Deselect
          </button>
        </template>

        <button v-else class="hover-bubble bubble-highlight" @click="selectMode = !selectMode">
          <span v-if="selectMode" class="material-icons"> &#xe5cd; </span>
          <span v-else class="material-icons"> &#xe03c;</span>
          {{ selectMode ? "Cancel" : "Select" }}
        </button>

        <button :class="{ active: shouldShowButtonUp }" class="go-up" data-title-bottom="Scroll Up" @click="scrollUp">
          <span class="material-icons"> &#xe5d8; </span>
        </button>
      </div>
    </div>

    <div class="hi-album-images">
      <!-- <div class="hi-album-image-col" v-for="(chunk, index) in chunks" :key="index"> -->
      <div v-for="(group, date) in dataByMonths" :key="date" class="image-group">
        <div class="image-group-title">
          <span>{{ getGroupDate(date) }}</span>
          <div class="flex-1" />
          <span>{{ group.length }} {{ group.length === 1 ? 'image' : 'images' }}</span>
        </div>

        <div class="image-group-items">
          <UserImageItem
            v-for="image in group"
            :key="image.key"
            :image="image"
            :is-select="selected.has(image.key)"
            :mode="selectMode"
            :index="data.findIndex((item) => item.key === image.key)"
            @select="selectItem"
          />
        </div>
      </div>

      <!-- </div> -->
    </div>
    <Teleport v-if="modal" to="body">
      <Modal @click="modal = false">
        <div class="modal-wrap modal-select-album">
          <button class="btn-close" data-title-left="Close" @click="open = false">
            <span class="material-icons">&#xe5cd;</span>
          </button>

          <h2>Select an album</h2>
          <p>Please choose an album to add the selected {{ selected.size === 1 ? "image" : "images" }} to.</p>

          <LoadingSpin v-if="selectingLoading" class="dark small" />

          <template v-else-if="albums && !isEmpty(albums)">
            <router-link
              v-for="album in albums"
              :key="album.key"
              class="select-album-item"
              :to="{
                name: 'AlbumEdit',
                params: { id: album.key, images: JSON.stringify([...selected.values()]) },
              }"
            >
              <div class="album-item-image">
                <img :src="imageUrl(album.coverKey, 'tiny')" alt="">
              </div>

              <div class="album-item-meta">
                <strong>{{ album.title }}</strong>
                <p>Uploaded {{ formatDate(album.publishedAt) }}</p>
              </div>
            </router-link>
          </template>

          <p v-else>
            You have no albums. <router-link :to="{ name: 'Upload' }">
              Uplaod one.
            </router-link>
          </p>
        </div>
      </Modal>
    </Teleport>
  </div>
</template>
