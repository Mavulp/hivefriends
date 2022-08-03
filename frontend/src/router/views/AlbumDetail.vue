<script setup lang="ts">
import { onBeforeMount, computed, reactive, ref, watchEffect, onMounted, watch } from "vue"
import { onBeforeRouteLeave, useRoute, useRouter } from "vue-router"
import { useAlbums, Album, imageUrl, Image } from "../../store/album"
import { useLoading } from "../../store/loading"
import { debounce, isEmpty } from "lodash"
import { useUser } from "../../store/user"
import { formatDate, sanitize } from "../../js/utils"
import { useClipboard, useCssVar, usePreferredDark, whenever } from "@vueuse/core"
import { useBread } from "../../store/bread"
import { url } from "../../js/fetch"
import { useToast } from "../../store/toast"
import { formatTextUsernames, getImageChunks } from "../../js/_composables"

import LoadingSpin from "../../components/loading/LoadingSpin.vue"
import AlbumTimestamp from "../../components/albums/AlbumTimestamp.vue"
import ImageListitem from "../../components/albums/ImageListitem.vue"
import Modal from "../../components/Modal.vue"
import AlbumMap from "../../components/albums/AlbumMap.vue"
import Button from "../../components/Button.vue"

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

const _id = computed(() => route?.params?.id.toString() ?? null)

onBeforeMount(async () => {
  const token = route.params.token

  if (_id.value) {
    const data = await albums.fetchAlbum(_id.value, token)
    Object.assign(album, data)

    bread.set(`${album.title} ${album.draft ? "(draft)" : ""} by ${user.getUsername(data.author)}`)
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

const chunks = computed(() => getImageChunks(album.images))

function openCoverImage() {
  router.push({
    name: user.public_token ? "PublicImageDetail" : "ImageDetail",
    params: {
      album: album.key,
      image: album.coverKey,
      ...(user.public_token && { token: user.public_token })
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
    copyPublic()
  } else {
    modal.value = true
  }
}

function copyPublic() {
  copy(publicLink.value)
  toast.add("Album share link copied to clipboard")
  modal.value = false
}

/**
 *  Album map
 */
const map = ref(false)
const enableMap = computed(() =>
  album.images.some((image) => image.location && image.location.latitude && image.location.longitude)
)

/**
 * Remember scroll position
 */

onBeforeRouteLeave((to) => {
  if (to.name === "ImageDetail") {
    sessionStorage.setItem("album-scroll", window.scrollY.toString())
  } else {
    sessionStorage.removeItem("album-scroll")
  }
})

watch(
  () => getLoading("get-album"),
  () => {
    const scroll = sessionStorage.getItem("album-scroll")

    if (scroll) {
      setTimeout(() => {
        window.scrollTo(0, parseInt(scroll))
      }, 50)
    }
  }
)

/**
 * Fixed title
 */

const showFixedTitle = ref(false)

window.addEventListener(
  "scroll",
  debounce(() => {
    if (window.scrollY > window.innerHeight) {
      showFixedTitle.value = true
    } else {
      showFixedTitle.value = false
    }
  }, 5)
)

function scrollUp() {
  window.scrollTo({ top: 0, behavior: "smooth" })
}

whenever(showUsers, () => {
  if (showFixedTitle.value) {
    scrollUp()
  }
})
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

    <template v-else>
      <div class="hi-album-title">
        <Teleport to="body">
          <Modal @close="modal = false" v-if="modal">
            <div class="modal-wrap modal-copy">
              <div class="modal-title">
                <h4>Album sharing link</h4>
                <button class="modal-close" @click="modal = false">
                  <span class="material-icons">&#xe5cd;</span>
                </button>
              </div>
              <p>Anyone with this link will be able to view this album</p>
              <input readonly :value="publicLink" />
            </div>
          </Modal>

          <Modal v-if="map" @close="map = false">
            <AlbumMap :album="album" @close="map = false" />
          </Modal>
        </Teleport>

        <div class="hi-album-title-meta">
          <div class="is-draft hover-bubble bubble-orange active" v-if="album.draft">Draft</div>

          <AlbumTimestamp class="dark" :timeframe="album.timeframe" />

          <h1>{{ album.title }}</h1>
          <p v-if="album.description" v-html="sanitize(formatTextUsernames(album.description, user))"></p>

          <div class="album-meta-cells">
            <span class="material-icons">&#xe3f4;</span>
            <p class="mr-32">{{ album.images.length }} {{ album.images.length === 1 ? "Photo" : "Photos" }}</p>

            <span class="material-icons">&#xe851;</span>
            <router-link :to="{ name: 'UserProfile', params: { user: album.author } }" class="mr-32">
              by: {{ user.getUsername(album.author) }}
            </router-link>

            <span class="material-icons">&#xe8df;</span>
            <p>Uploaded</p>
            <p>{{ formatDate(album.publishedAt) }}</p>
          </div>
        </div>

        <div class="hi-album-title-thumbnail">
          <div class="detail-buttons">
            <router-link
              :to="{ name: 'AlbumEdit', params: { id: album.key } }"
              class="hover-bubble bubble-orange"
              v-if="user.user.username === album.author"
            >
              <span class="material-icons">&#xe3c9;</span>
              Edit
            </router-link>

            <button class="hover-bubble" @click="map = true" v-if="enableMap">
              <span class="material-icons">&#xe55b;</span>
              Map
            </button>

            <button class="hover-bubble" @click="showUsers = !showUsers" :class="{ active: showUsers }">
              <span class="material-icons">&#xe7fb;</span>
              People {{ album.taggedUsers.length ?? 0 }}
            </button>

            <button class="hover-bubble data-title-width-156" @click="getPublicLink" v-if="!user.public_token">
              <span class="material-icons">&#xe80d;</span>
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
                  <div class="tag tag-orange" v-if="item === album.author">Author</div>
                  <div class="background"></div>
                  <div class="background" :style="[`backgroundColor: rgb(${user.getUser(item, 'accentColor')})`]"></div>
                </router-link>
              </template>
              <p v-else>Nobody is here.</p>
            </div>

            <img @click="openCoverImage" class="cover-image" :src="imageUrl(album.coverKey)" alt=" " />
          </div>
        </div>
      </div>

      <div class="hi-album-title-fixed" v-if="showFixedTitle && !map && !modal">
        <div>
          <p class="uploaded">Uploaded {{ formatDate(album.publishedAt) }}</p>
          <h5>{{ album.title }}</h5>
        </div>

        <div>
          <!-- <div class="detail-buttons"> -->
          <router-link
            :to="{ name: 'AlbumEdit', params: { id: album.key } }"
            class="hover-bubble bubble-orange"
            v-if="user.user.username === album.author"
          >
            <span class="material-icons">&#xe3c9;</span>
            Edit
          </router-link>

          <button class="hover-bubble" @click="map = true" v-if="enableMap">
            <span class="material-icons">&#xe55b;</span>
            Map
          </button>

          <button class="hover-bubble" @click="showUsers = !showUsers" :class="{ active: showUsers }">
            <span class="material-icons">&#xe7fb;</span>
            People {{ album.taggedUsers.length ?? 0 }}
          </button>

          <button class="hover-bubble data-title-width-156" @click="getPublicLink" v-if="!user.public_token">
            <span class="material-icons">&#xe80d;</span>
            Share
            <span class="material-icons rotate" v-if="getLoading('share-link')">&#xe863;</span>
          </button>

          <!-- </div> -->
          <button class="go-up" @click="scrollUp" data-title-bottom="Scroll Up">
            <span class="material-icons"> &#xe5d8; </span>
          </button>
        </div>
      </div>
    </template>

    <div class="hi-album-images">
      <div class="hi-album-image-col" v-for="chunk in chunks" :key="chunk.length">
        <ImageListitem v-for="image in chunk" :key="image.key" :image="image" :album-key="album.key" />
      </div>
    </div>
  </div>
</template>
