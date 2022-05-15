<script setup lang="ts">
import { Image, useAlbums, imageUrl } from "../../store/album"

interface Props {
  image: Image
  albumKey: string
}

const { image, albumKey } = defineProps<Props>()
</script>

<template>
  <router-link
    :to="{
      name: 'ImageDetail',
      params: {
        album: albumKey,
        image: image.key
      }
    }"
    class="hi-album-image"
  >
    <div class="image-wrap">
      <img :src="imageUrl(image.key, 'medium')" />
    </div>
  </router-link>
</template>
