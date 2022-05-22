<script setup lang="ts">
import { onBeforeMount, computed, reactive, ref, watchEffect } from "vue"
import { useRoute, useRouter } from "vue-router"
import { useAlbums, Album, imageUrl, Image } from "../../store/album"
import { useLoading } from "../../store/loading"
import { isEmpty } from "lodash"
import { useUser } from "../../store/user"
import { formatDate } from "../../js/utils"

import LoadingSpin from "../../components/loading/LoadingSpin.vue"
import AlbumTimestamp from "../../components/albums/AlbumTimestamp.vue"
import ImageListitem from "../../components/albums/ImageListitem.vue"
import { useCssVar } from "@vueuse/core"

const albums = useAlbums()
const route = useRoute()
const router = useRouter()
const user = useUser()
const { getLoading } = useLoading()

const showUsers = ref(false)
const album = reactive<Album>({} as Album)

const wrap = ref(null)
const color = useCssVar("--color-highlight", wrap)

onBeforeMount(async () => {
  const id = route.params.id

  if (id) {
    const data = await albums.fetchAlbum(id)
    Object.assign(album, data)
  }
})

watchEffect(() => {
  if (album.author) {
    const accent = user.getUser(album.author, "accentColor")
    color.value = accent
  }
})

const tagged = computed(() => [album.author, ...album.taggedUsers])

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
          <!-- <button class="hover-bubble">
            <span class="material-icons">&#xe0b9;</span>
            Comments
          </button> -->

          <button class="hover-bubble">
            <span class="material-icons">&#xe55b;</span>
            Map
          </button>

          <button class="hover-bubble" @click="showUsers = !showUsers" :class="{ active: showUsers }">
            <span class="material-icons">&#xe7fb;</span>
            People {{ tagged.length ?? 0 }}
          </button>
        </div>
        <div class="thumbnail-image-wrap">
          <div class="album-tagged-users" :class="{ active: showUsers }">
            <button @click="showUsers = false">
              <span class="material-icons">&#xe5cd;</span>
            </button>

            <h6>Tagged people</h6>

            <router-link
              v-for="item in tagged"
              :key="item"
              class="album-tagged-user"
              :to="{ name: 'UserProfile', params: { user: item } }"
            >
              <img
                class="user-image"
                :src="imageUrl(user.getUser(item, 'avatarKey'), 'tiny')"
                :style="[`backgroundColor: rgb(${user.getUser(item, 'accentColor')})`]"
              />
              <span>{{ user.getUsername(item) }}</span>

              <div class="tag tag-blue" v-if="item === album.author">Author</div>

              <div class="background"></div>

              <div class="background" :style="[`backgroundColor: rgb(${user.getUser(item, 'accentColor')})`]"></div>
            </router-link>
          </div>

          <img @click="openFirstImage" class="cover-image" :src="imageUrl(album.images[0].key)" alt=" " />
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
