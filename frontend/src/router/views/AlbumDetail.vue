<script setup lang="ts">
import { computed, onBeforeMount, reactive, ref, watch, watchEffect } from 'vue'
import { onBeforeRouteLeave, useRoute, useRouter } from 'vue-router'
import { debounce, isEmpty } from 'lodash'
import { useClipboard, useCssVar, useLocalStorage, usePreferredDark, whenever } from '@vueuse/core'
import dayjs from 'dayjs'
import type { Album } from '../../store/album'
import { imageUrl, useAlbums } from '../../store/album'

import { useLoading } from '../../store/loading'
import { useUser } from '../../store/user'
import { useBread } from '../../store/bread'
import { url } from '../../js/fetch'
import { useToast } from '../../store/toast'

import Detail from '../../components/Detail.vue'

import LoadingSpin from '../../components/loading/LoadingSpin.vue'
import Modal from '../../components/Modal.vue'
import AlbumMap from '../../components/albums/AlbumMap.vue'
import { isValidMarker } from '../../js/map'
import ImageListitem from '../../components/albums/ImageListitem.vue'
import { formatTextUsernames } from '../../js/_composables'
import { sanitize } from '../../js/utils'
import { normalDateFormat } from '../../js/time'
import AlbumTimestamp from '../../components/albums/AlbumTimestamp.vue'

const albums = useAlbums()
const route = useRoute()
const router = useRouter()
const user = useUser()
const bread = useBread()
const toast = useToast()
const { addLoading, delLoading, getLoading } = useLoading()

const showUsers = ref(false)
const album = reactive<Album>({} as Album)

const wrap = ref(null)
const color = useCssVar('--color-highlight', wrap)

const _id = computed(() => route?.params?.id.toString() ?? null)

onBeforeMount(async () => {
  const token = route.params.token

  if (_id.value) {
    const data = await albums.fetchAlbum(_id.value, token)
    Object.assign(album, data)

    bread.set(`${album.title} ${album.draft ? '(draft)' : ''} by ${user.getUsername(data.author)}`)
  }

  const isDark = usePreferredDark()

  if (isDark.value && user.public_token) {
    const root = document.querySelector(':root')
    if (root)
      root.classList.add('dark-normal')
  }
})

watchEffect(() => {
  if (album.author) {
    const accent = user.getUser(album.author, 'accentColor')
    color.value = accent
  }
})

function openCoverImage() {
  router.push({
    name: user.public_token ? 'PublicImageDetail' : 'ImageDetail',
    params: {
      album: album.key,
      image: album.coverKey,
      ...(user.public_token && { token: user.public_token }),
    },
  })
}

/**
 * Generate public link
 */

const modal = ref(false)
const publicLink = ref('')
const { copy, isSupported } = useClipboard()

async function getPublicLink() {
  if (!publicLink.value) {
    addLoading('share-link')

    const token = await albums.genPublicAlbumToken(_id.value)
    delLoading('share-link')

    if (token)
      publicLink.value = `${url}/public${route.fullPath}/${token}`
  }

  if (!publicLink.value)
    return

  if (isSupported)
    copyPublic()
  else
    modal.value = true
}

function copyPublic() {
  copy(publicLink.value)
  toast.add('Album share link copied to clipboard')
  modal.value = false
}

/**
 *  Album map
 */
const map = ref(false)
const enableMap = computed(() => album.images.some(image => isValidMarker(image)))

/**
 * Remember scroll position
 */

onBeforeRouteLeave((to) => {
  if (to.name === 'ImageDetail')
    sessionStorage.setItem('album-scroll', window.scrollY.toString())
  else
    sessionStorage.removeItem('album-scroll')
})

watch(
  () => getLoading('get-album'),
  () => {
    const scroll = sessionStorage.getItem('album-scroll')

    if (scroll) {
      setTimeout(() => {
        window.scrollTo(0, parseInt(scroll))
      }, 50)
    }
  },
)

/**
 * Fixed title
 */

const showFixedTitle = ref(false)

window.addEventListener(
  'scroll',
  debounce(() => {
    if (window.scrollY > window.innerHeight)
      showFixedTitle.value = true
    else
      showFixedTitle.value = false
  }, 5),
)

function scrollUp() {
  window.scrollTo({ top: 0, behavior: 'smooth' })
}

function scrollDown() {
  window.scrollTo({ top: document.body.scrollHeight, behavior: 'smooth' })
}

whenever(showUsers, () => {
  if (showFixedTitle.value)
    scrollUp()
})

// Sort images

const descending = useLocalStorage('image-order', false)

const sortedImages = computed(() => {
  if (!descending.value)
    return album.images
  return [...album.images].sort((a, b) => a.uploadedAt > b.uploadedAt ? -1 : 1)
})
</script>

<template>
  <div class="hi-album-detail">
    <LoadingSpin v-if="getLoading('get-album')" dark />

    <div v-else-if="isEmpty(album)" class="hi-album-detail-error">
      <div class="centered">
        <h3>Uh oh</h3>
        <p>Error loading album</p>
      </div>
    </div>

    <div v-else class="hi-double">
      <div class="hi-album-title">
        <Teleport to="body">
          <Modal v-if="modal" @close="modal = false">
            <div class="modal-wrap modal-copy">
              <div class="modal-title">
                <h4>Album sharing link</h4>
                <button class="modal-close" @click="modal = false">
                  <span class="material-icons">&#xe5cd;</span>
                </button>
              </div>
              <p>Anyone with this link will be able to view this album</p>
              <input readonly :value="publicLink">
            </div>
          </Modal>

          <Modal v-if="map" @close="map = false">
            <AlbumMap :album="album" @close="map = false" />
          </Modal>
        </Teleport>

        <div class="album-thumbnail">
          <img class="cover-image" :src="imageUrl(album.coverKey)" alt=" " @click="openCoverImage">

          <div v-if="album.coverKey" class="blur-bg">
            <img :src="imageUrl(album.coverKey, 'medium')">
          </div>
        </div>

        <div class="hi-album-title-meta">
          <div v-if="album.draft" class="is-draft hover-bubble bubble-orange active">
            Draft
          </div>

          <AlbumTimestamp class="dark" :timeframe="album.timeframe" />

          <h1>{{ album.title }}</h1>
          <p v-if="album.description" v-html="sanitize(formatTextUsernames(album.description, user))" />

          <div class="album-meta-cells">
            <span class="material-icons">&#xe3f4;</span>
            <p class="mr-32">
              {{ album.images.length }} {{ album.images.length === 1 ? "Photo" : "Photos" }}
            </p>

            <span class="material-icons">&#xe851;</span>
            <router-link :to="{ name: 'UserProfile', params: { user: album.author } }" class="mr-32">
              by: {{ user.getUsername(album.author) }}
            </router-link>

            <span class="material-icons">&#xe8df;</span>
            <p>Uploaded</p>
            <p>{{ dayjs(album.publishedAt * 1000).format(normalDateFormat) }}</p>
          </div>
        </div>
      </div>

      <div>
        <div class="detail-buttons">
          <router-link
            v-if="user.user.username === album.author"
            :to="{ name: 'AlbumEdit', params: { id: album.key } }"
            class="hover-bubble bubble-orange"
          >
            <span class="material-icons">&#xe3c9;</span>
            Edit
          </router-link>

          <button v-if="enableMap" class="hover-bubble" @click="map = true">
            <span class="material-icons">&#xe55b;</span>
            Map
          </button>

          <button class="hover-bubble" :class="{ active: showUsers }" @click="showUsers = !showUsers">
            <span class="material-icons">&#xe7fb;</span>
            People {{ album.taggedUsers.length ?? 0 }}
          </button>

          <button v-if="!user.public_token" class="hover-bubble data-title-width-156" @click="getPublicLink">
            <span class="material-icons">&#xe80d;</span>
            Share
            <span v-if="getLoading('share-link')" class="material-icons rotate">&#xe863;</span>
          </button>

          <div class="flex-1" />

          <button
            class="hover-bubble"
            :data-title-bottom="descending ? 'Sorting by newest' : 'Sorting by oldest'"
            @click="descending = !descending"
          >
            <div :style="[descending ? 'transform: scaleY(-1);' : '']">
              <span class="material-icons">&#xe164;</span>
            </div>
            {{ descending ? 'Newest' : "Oldest" }}
          </button>

          <button class="hover-bubble" data-title-bottom-right="Scroll Up" @click="scrollUp()">
            <span class="material-icons"> &#xe5d8; </span>
          </button>
        </div>

        <Detail :open="showUsers" class="tagged-users">
          <template #content>
            <div class="wrapper">
              <div v-if="album.taggedUsers.length > 0" class="user-list">
                <router-link
                  v-for="item in album.taggedUsers"
                  :key="item"
                  class="album-tagged-user"
                  :to="{ name: 'UserProfile', params: { user: item } }"
                >
                  <img
                    class="user-image"
                    :src="imageUrl(user.getUser(item, 'avatarKey'), 'tiny')"
                    :style="[`backgroundColor: rgb(${user.getUser(item, 'accentColor')})`]"
                    alt=" "
                    @error="(e: any) => e.target.classList.add('image-error')"
                  >
                  <span>{{ user.getUsername(item) }}</span>
                  <div v-if="item === album.author" class="tag tag-orange">
                    Author
                  </div>
                  <div class="background" />
                  <div class="background" :style="[`backgroundColor: rgb(${user.getUser(item, 'accentColor')})`]" />
                </router-link>
              </div>
              <p v-else>
                Nobody is here.
              </p>
            </div>

            <div class="divider" />
          </template>
        </Detail>

        <div v-if="album.images" class="hi-album-images">
          <ImageListitem v-for="image in sortedImages" :key="image.key" :image="image" :album-key="album.key" />
        </div>
      </div>
    </div>

    <!-- <div v-if="album.coverKey" class="blur-bg">
      <img :src="imageUrl(album.coverKey, 'medium')">
    </div> -->
  </div>
</template>
