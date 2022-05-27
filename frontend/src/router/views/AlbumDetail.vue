<script setup lang="ts">
import { onBeforeMount, computed, reactive, ref, watchEffect } from "vue"
import { useRoute, useRouter } from "vue-router"
import { useAlbums, Album, imageUrl, Image } from "../../store/album"
import { useLoading } from "../../store/loading"
import { isEmpty } from "lodash"
import { useUser } from "../../store/user"
import { formatDate } from "../../js/utils"
import { useClipboard, useCssVar, usePreferredDark } from "@vueuse/core"
import { useBread } from "../../store/bread"
import { url } from "../../js/fetch"
// import { useHead } from "@vueuse/head"

import LoadingSpin from "../../components/loading/LoadingSpin.vue"
import AlbumTimestamp from "../../components/albums/AlbumTimestamp.vue"
import ImageListitem from "../../components/albums/ImageListitem.vue"
import Modal from "../../components/Modal.vue"
import { useToast } from "../../store/toast"

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
const color = useCssVar("--color-highlight", wrap)

const _id = computed(() => `${route.params.id}`)

onBeforeMount(async () => {
  const token = route.params.token

  if (_id.value) {
    const data = await albums.fetchAlbum(_id.value, token)
    Object.assign(album, data)

    bread.set(`${album.title} by ${user.getUsername(data.author)}`)

    // Set metadata
    // useHead({
    //   meta: [
    //     {
    //       name: "og:title",
    //       content: album.title
    //     },
    //     {
    //       name: "og:descrption",
    //       content: album.description
    //     },
    //     {
    //       name: "og:image",
    //       content: imageUrl(album.coverKey, "medium")
    //     }
    //   ]
    // })
  }

  const isDark = usePreferredDark()

  if (isDark.value && user.public_token) {
    const root = document.querySelector(":root")
    if (root) {
      root.classList.add("dark-normal")
    }
  }
})

watchEffect(() => {
  if (album.author) {
    const accent = user.getUser(album.author, "accentColor")
    color.value = accent
  }
})

const chunks = computed(() => {
  if (!album.images) return []

  const images: any = album.images
  const chunks: Array<Array<Image>> = [[], [], []]

  let i: number = 0
  let j: number = 0

  while (i !== images.length) {
    chunks[j].push(images[i])

    if (j >= 2) j = 0
    else j++

    i++
  }

  return chunks
})

function openFirstImage() {
  router.push({
    name: "ImageDetail",
    params: {
      album: album.key,
      image: album.images[0].key
    }
  })
}

/**
 * Generate public link
 */

const modal = ref(false)
const publicLink = ref("")
const { copy, isSupported } = useClipboard()

async function getPublicLink() {
  if (!publicLink.value) {
    addLoading("share-link")

    const token = await albums.genPublicAlbumToken(_id.value)
    delLoading("share-link")

    if (token) publicLink.value = `${url}/public${route.fullPath}/${token}`
  }

  if (!publicLink.value) return

  if (isSupported) {
    copy(publicLink.value)
    toast.add("Album link copied to clipboard")
  } else {
    modal.value = true
  }
}
</script>

<template>
  <div class="hi-album-detail">
    <LoadingSpin dark v-if="getLoading('get-album')" />

    <div class="hi-album-detail-error" v-else-if="isEmpty(album)">
      <div class="centered">
        <h3>Lmao</h3>
        <p>Error loading album</p>
      </div>
    </div>

    <div class="hi-album-title" v-else>
      <Teleport to="body">
        <Modal @close="modal = false" v-if="modal">
          <div class="modal-wrap modal-copy">
            <div class="modal-title">
              <h4>Public link</h4>
              <button class="modal-close" @click="modal = false">
                <span class="material-icons">&#xe5cd;</span>
              </button>
            </div>
            <p>Anyone with this link will be able to view this album</p>
            <input :value="publicLink" />
          </div>
        </Modal>
      </Teleport>

      <div class="hi-album-title-meta">
        <AlbumTimestamp class="dark" :timeframe="album.timeframe" />

        <h1>{{ album.title }}</h1>
        <p>{{ album.description }}</p>

        <div class="album-meta-cells">
          <span class="material-icons">&#xe3f4;</span>
          <p class="mr-32">{{ album.images.length }} {{ album.images.length === 1 ? "Photo" : "Photos" }}</p>

          <span class="material-icons">&#xe851;</span>
          <router-link :to="{ name: 'UserProfile', params: { user: album.author } }" class="mr-32">
            by: {{ user.getUsername(album.author) }}
          </router-link>

          <span class="material-icons">&#xe8df;</span>
          <p>Uploaded</p>
          <p>{{ formatDate(album.createdAt) }}</p>
        </div>
      </div>

      <div class="hi-album-title-thumbnail">
        <div class="detail-buttons">
          <button
            class="hover-bubble bubble-red"
            data-title-top="Edit Album / Images"
            v-if="user.user.username === album.author"
          >
            <span class="material-icons">&#xe3c9;</span>
            Edit
          </button>

          <button class="hover-bubble" data-title-top="WIP">
            <span class="material-icons">&#xe55b;</span>
            Map
          </button>

          <button class="hover-bubble" @click="showUsers = !showUsers" :class="{ active: showUsers }">
            <span class="material-icons">&#xe7fb;</span>
            People {{ album.taggedUsers.length ?? 0 }}
          </button>

          <button class="hover-bubble data-title-width-156" @click="getPublicLink" v-if="!user.public_token">
            <span class="material-icons">&#xe157;</span>
            Share
            <span class="material-icons rotate" v-if="getLoading('share-link')">&#xe863;</span>
          </button>
        </div>
        <div class="thumbnail-image-wrap">
          <div class="album-tagged-users" :class="{ active: showUsers }">
            <button @click="showUsers = false">
              <span class="material-icons">&#xe5cd;</span>
            </button>
            <h6>Tagged people</h6>

            <template v-if="album.taggedUsers.length > 0">
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
                />
                <span>{{ user.getUsername(item) }}</span>
                <div class="tag tag-blue" v-if="item === album.author">Author</div>
                <div class="background"></div>
                <div class="background" :style="[`backgroundColor: rgb(${user.getUser(item, 'accentColor')})`]"></div>
              </router-link>
            </template>
            <p v-else>Nobody is here.</p>
          </div>

          <img @click="openFirstImage" class="cover-image" :src="imageUrl(album.coverKey)" alt=" " />
        </div>
      </div>
    </div>

    <div class="hi-album-images">
      <div class="hi-album-image-col" v-for="chunk in chunks" :key="chunk.length">
        <ImageListitem v-for="image in chunk" :key="image.key" :image="image" :album-key="album.key" />
      </div>
    </div>
  </div>
</template>
