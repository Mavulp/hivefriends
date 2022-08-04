<script setup lang="ts">
import { computed } from "vue"
import { Comment } from "../../../store/comments.js"
import { User, useUser } from "../../../store/user.js"
import { imageUrl, useAlbums } from "../../../store/album.js"
import { formatTextUsernames, formatTextImages } from "../../../js/_composables"
import { sanitize, formatDate, RGB_TO_HEX } from "../../../js/utils"
import { useRouter } from "vue-router"

interface PropComment extends Comment {
  albumKey: string
}

const router = useRouter()
const albums = useAlbums()
const user = useUser()
const props = defineProps<{ data: PropComment }>()

// Getting right values
const data = computed(() => props.data)
const author = computed<User>(() => user.getUser(data.value.author))
const album = computed(() => albums.getListAlbum(data.value.albumKey))

const comment = computed(() => {
  if (!data.value.text) return ""

  let text = data.value.text

  text = formatTextImages(text)
  text = formatTextUsernames(text, user)

  return text
})

function go() {
  router.push({
    name: "ImageDetail",
    params: {
      album: data.value.albumKey,
      image: data.value.imageKey
    }
  })
}
</script>

<template>
  <div class="activity-item activity-comment" @click.self="go" v-if="album">
    <div class="activity-title">
      <img
        class="user-image"
        :src="imageUrl(author.avatarKey, 'medium')"
        alt=" "
        @error="(e: any) => e.target.classList.add('image-error')"
      />

      <p @click.self="go">
        <router-link :to="{ name: 'UserProfile', params: { user: author.username } }">
          {{ author.displayName }}
        </router-link>
        commented in
        <router-link :to="{ name: 'AlbumDetail', params: { id: data.albumKey } }">{{ album.title }}</router-link> by
        <router-link :to="{ name: 'AlbumDetail', params: { id: album.author } }">{{
          user.getUsername(album.author)
        }}</router-link>
        on <i>{{ formatDate(data.createdAt, ["weekday", "year"]) }}</i>
      </p>
    </div>

    <div class="activity-body">
      <p class="comment-body" v-html="sanitize(comment)"></p>
    </div>

    <div class="background" :style="{ backgroundColor: RGB_TO_HEX(author.accentColor) }"></div>
  </div>
</template>
