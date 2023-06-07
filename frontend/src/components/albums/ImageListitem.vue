<script setup lang="ts">
import type { Image } from '../../store/album'
import { imageUrl } from '../../store/album'
import { useUser } from '../../store/user'

const { image, albumKey } = defineProps<Props>()

const user = useUser()

interface Props {
  image: Image
  albumKey: string
}
</script>

<template>
  <router-link
    :to="{
      name: user.public_token ? 'PublicImageDetail' : 'ImageDetail',
      params: {
        album: albumKey,
        image: image.key,
        ...(user.public_token && { token: user.public_token }),
      },
    }"
    class="hi-album-image"
  >
    <!-- <div class="image-wrap"> -->
    <div v-if="image.commentCount" class="image-comment-count">
      <p>{{ image.commentCount }}</p>
      <span class="material-icons">&#xe0b7;</span>
      <!-- <p>{{ image.commentCount }} {{ image.commentCount === 1 ? 'comment' : 'comments' }}</p> -->
    </div>

    <img :src="imageUrl(image.key, 'medium')">
    <!-- </div> -->
  </router-link>
</template>
