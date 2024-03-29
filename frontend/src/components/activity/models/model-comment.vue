<script setup lang="ts">
import { computed } from 'vue'
import { useRouter } from 'vue-router'
import type { Comment } from '../../../store/comments.js'
import type { User } from '../../../store/user.js'
import { useUser } from '../../../store/user.js'
import { imageUrl, useAlbums } from '../../../store/album.js'
import { formatTextImages, formatTextUsernames } from '../../../js/_composables'
import { RGB_TO_HEX, sanitize } from '../../../js/utils'

interface PropComment extends Comment {
  albumKey: string
}

const props = defineProps<{ data: PropComment }>()
const router = useRouter()
const albums = useAlbums()
const user = useUser()
const data = computed(() => props.data)
const author = computed<User>(() => user.getUser(data.value.author))
const album = computed(() => albums.getListAlbum(data.value.albumKey))

const comment = computed(() => {
  if (!data.value.text)
    return ''

  let text = data.value.text

  text = formatTextImages(text)
  text = formatTextUsernames(text, user)

  return text
})

function go() {
  router.push({
    name: 'ImageDetail',
    params: {
      album: data.value.albumKey,
      image: data.value.imageKey,
    },
  })
}
</script>

<template>
  <div v-if="album" class="activity-item activity-comment" @click.self="go">
    <div class="activity-title">
      <img
        class="user-image"
        :src="imageUrl(author.avatarKey, 'medium')"
        alt=" "
        @error="(e: any) => e.target.classList.add('image-error')"
      >

      <p @click.self="go">
        <router-link :to="{ name: 'UserProfile', params: { user: author.username } }">
          {{ author.displayName }}
        </router-link>
        commented in
        <router-link :to="{ name: 'AlbumDetail', params: { id: data.albumKey } }">
          {{ album.title }}
        </router-link> by
        <router-link :to="{ name: 'AlbumDetail', params: { id: album.author } }">
          {{
            user.getUsername(album.author)
          }}
        </router-link>
      </p>
    </div>

    <div class="activity-body">
      <p class="comment-body" v-html="sanitize(comment)" />
    </div>

    <div class="background" :style="{ backgroundColor: RGB_TO_HEX(author.accentColor) }" />
  </div>
</template>
