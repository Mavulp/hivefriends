<script setup lang='ts'>
import { computed, onMounted, ref } from 'vue'
import { Slider } from '@dolanske/slider'
import dayjs from 'dayjs'
import type { ImageItemInAlbum } from '../../store/album'
import { imageUrl } from '../../store/album'
import { useUser } from '../../store/user'

import CommentsWrap from '../comments/CommentsWrap.vue'
import Detail from '../Detail.vue'
import { formatFileSize } from '../../js/utils'
import { timeDateFormat } from '../../js/time'

const props = defineProps<{
  images: ImageItemInAlbum[]
  user: string
  index: number
}>()

/**
 * Render image or a carousel of images with comments and information. Button to the image or album
 *
 */
const user = useUser()

const sliderId = computed(() => props.user + props.index)

// Refs updated when albums and images change
const activeImage = ref('')

onMounted(() => {
  // If more than 1 images is present, initialize slider
  activeImage.value = props.images[0].key
  initSlider()
})

let sliderInst: any
function initSlider() {
  if (props.images.length > 1) {
    sliderInst = new Slider(`#${sliderId.value}`, {
      // height: window.innerHeight / 100 * 75,
      active: props.images.findIndex(i => i.key === activeImage.value),
    })

    sliderInst.onSlideChange(({ toEl }: { toEl: HTMLDivElement }) => {
      // Fetch comments on slide change
      activeImage.value = toEl.getAttribute('data-image-key') as string
    })
  }
}

const visibleImage = computed(() => {
  return props.images.find(i => i.key === activeImage.value)
})

// Scroll into the view
const wrapper = ref<HTMLDivElement>()
function scrollToMe() {
  if (wrapper.value) {
    wrapper.value.scrollIntoView({
      behavior: 'smooth',
      block: 'start',
    })
  }
}
</script>

<template>
  <div ref="wrapper" class="feed-user-wrap">
    <!-- <button class="scroll-to" @click="scrollToMe">
      me!
    </button> -->

    <div v-if="props.images.length > 1" :id="sliderId">
      <div v-for="image in props.images" :key="image.key" class="slider-image" :data-image-key="image.key">
        <img :src="imageUrl(image.key, 'large')" alt=" ">
        <img class="blurred" :src="imageUrl(image.key, 'large')" alt=" ">
      </div>
    </div>
    <div v-else class="image-wrap">
      <img :src="imageUrl(props.images[0].key, 'large')" alt="">
      <img class="blurred" :src="imageUrl(props.images[0].key, 'large')" alt=" ">
    </div>

    <div v-if="visibleImage" class="feed-image-info">
      <div class="image-info">
        <div>
          <router-link :data-title-bottom="`${visibleImage.uploader}s profile`" class="hover-bubble" :to="{ name: 'UserProfile', params: { user: visibleImage.uploader } }">
            <img
              class="user-image"
              :src="imageUrl(user.getUser(visibleImage.uploader, 'avatarKey'), 'tiny')"
              :style="{
                backgroundColor: `rgb(${user.getUser(visibleImage.uploader, 'accentColor')})`,
              }"
              alt=" "
              @error="(e: any) => e.target.classList.add('image-error')"
            >
          </router-link>
        </div>

        <div>
          <strong>{{ visibleImage.fileName }}</strong>
          <p>{{ visibleImage.description }}</p>
        </div>
      </div>

      <div class="image-links">
        <RouterLink :to="{ name: 'AlbumDetail', params: { id: visibleImage.albumKeys[0] } }">
          <span class="material-icons">&#xe89e;</span>
          Album
        </RouterLink>

        <RouterLink
          :to="{
            name: user.public_token ? 'PublicImageDetail' : 'ImageDetail',
            params: {
              album: visibleImage.albumKeys[0],
              image: visibleImage.key,
            },
          }"
        >
          <span class="material-icons">&#xe89e;</span>
          Image
        </RouterLink>
      </div>

      <div class="camera-settings">
        <Detail>
          <template #header="{ open, toggle }">
            <button :class="{ active: open }" @click="toggle">
              Camera Settings

              <span v-if="open" class="material-icons">&#xe5ce;</span>
              <span v-else class="material-icons">&#xe5cf;</span>
            </button>
          </template>
          <template #content>
            <div class="content-inner">
              <table>
                <tr v-if="visibleImage.cameraBrand">
                  <th>Camera</th>
                  <td>{{ visibleImage.cameraBrand }}, {{ visibleImage.cameraModel }}</td>
                </tr>
                <tr v-if="visibleImage.exposureTime">
                  <th>Exposure</th>
                  <td>{{ visibleImage.exposureTime }}</td>
                </tr>
                <tr v-if="visibleImage.fNumber">
                  <th>fNumber</th>
                  <td>{{ visibleImage.fNumber }}</td>
                </tr>
                <tr v-if="visibleImage.focalLength">
                  <th>Focal length</th>
                  <td>{{ visibleImage.focalLength }}</td>
                </tr>
                <tr v-if="visibleImage.sizeBytes">
                  <th>Size</th>
                  <td>{{ formatFileSize(visibleImage.sizeBytes, true) }}</td>
                </tr>
                <tr v-if="visibleImage.takenAt">
                  <th>Taken At</th>
                  <td>{{ dayjs(visibleImage.takenAt * 1000).format(timeDateFormat) }}</td>
                </tr>
              </table>
            </div>
          </template>
        </Detail>
      </div>

      <CommentsWrap
        :album-key="visibleImage.albumKeys[0]"
        :image-key="visibleImage.key"
        :uploader="visibleImage.uploader"
      />
    </div>
  </div>
</template>
