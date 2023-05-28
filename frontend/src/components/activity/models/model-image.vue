<script setup lang="ts">
import { computed } from 'vue'
import { take } from 'lodash'
import { useRoute, useRouter } from 'vue-router'
import { imageUrl } from '../../../store/album.js'
import type { User } from '../../../store/user.js'
import { useUser } from '../../../store/user.js'
import type { ReducedImage } from '../../../store/activity'
import { RGB_TO_HEX } from '../../../js/utils'

const props = defineProps<{ data: ReducedImage }>()
const user = useUser()
const router = useRouter()
const route = useRoute()
const data = computed(() => props.data)
const author = computed<User>(() => user.getUser(data.value.user))
const accent = computed(() => RGB_TO_HEX(author.value.accentColor))

const PHOTO_LIMIT = 20

function go(image: string, album: string) {
  if (!image || !album)
    return

  router.push({
    name: 'ImageDetail',
    params: { album, image },
  })
}
</script>

<template>
  <div class="activity-item activity-image">
    <div class="activity-title">
      <img
        class="user-image"
        :src="imageUrl(author.avatarKey, 'medium')"
        alt=" "
        @error="(e: any) => e.target.classList.add('image-error')"
      >

      <p>
        <router-link :to="{ name: 'UserProfile', params: { user: author.username } }">
          {{ author.displayName }}
        </router-link>

        uploaded {{ data.images.length }} {{ data.images.length > 1 ? "photos" : "photo" }}
      </p>

      <div class="photo-list">
        <img
          v-for="photo in take(data.images, PHOTO_LIMIT)"
          :key="photo.key"
          :src="imageUrl(photo.key, 'tiny')"
          :class="{ 'can-click': photo.albumKeys }"
          alt=" "
          @click="photo.albumKeys ? go(photo.key, photo.albumKeys[0]) : null"
        >

        <div v-if="data.images.length > PHOTO_LIMIT" class="img-additional">
          + {{ data.images.length - PHOTO_LIMIT }}
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped lang="scss">
.photo-list img.can-click {
  outline: 0px solid v-bind("accent");
  &:hover {
    outline-width: 3px;
  }
}
</style>
