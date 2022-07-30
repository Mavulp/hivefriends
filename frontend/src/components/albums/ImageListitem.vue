<script setup lang="ts">
import { Image, imageUrl } from "../../store/album"
import { useUser } from "../../store/user"

const user = useUser()

interface Props {
  image: Image
  albumKey: string
}

const { image, albumKey } = defineProps<Props>()
</script>

<template>
  <router-link
    :to="{
      name: user.public_token ? 'PublicImageDetail' : 'ImageDetail',
      params: {
        album: albumKey,
        image: image.key,
        ...(user.public_token && { token: user.public_token })
      }
    }"
    class="hi-album-image"
  >
    <div class="image-wrap">
      <div class="image-comment-count" v-if="image.commentCount">
        <span class="material-icons">&#xe0b7;</span>
        <p>{{ image.commentCount }}</p>
      </div>

      <img :src="imageUrl(image.key, 'medium')" />
    </div>
  </router-link>
</template>
