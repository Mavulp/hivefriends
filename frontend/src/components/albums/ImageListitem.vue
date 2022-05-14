<script setup lang="ts">
import { onBeforeMount, reactive, ref } from "vue"
import { Image, ImageMetadata, useAlbums, imageUrl } from "../../store/album"

const albums = useAlbums()

interface Props {
  image: Image
}

const { image } = defineProps<Props>()

// const metadata = reactive<ImageMetadata>({} as ImageMetadata)

// onBeforeMount(async () => {
//   const data = await albums.fetchImageMetadata(image.key)
//   Object.assign(metadata, data)
// })
</script>

<template>
  <div class="hi-album-image">
    <div class="image-wrap">
      <img :src="imageUrl(image.key, 'medium')" />
    </div>
    <!-- <pre>
      {{ image }}
    </pre> -->
  </div>
</template>
