<script setup lang="ts">
import { computed } from "vue"
import { Image, useAlbums, imageUrl } from "../../../store/album.js"
import { sanitize, formatDate, RGB_TO_HEX } from "../../../js/utils"
import { User, useUser } from "../../../store/user.js"
import { ReducedImage } from "../../../store/activity"
import { take } from "lodash"

const user = useUser()
const props = defineProps<{ data: ReducedImage }>()
const data = computed(() => props.data)
const author = computed<User>(() => user.getUser(data.value.user))

const PHOTO_LIMIT = 6
</script>

<template>
  <div class="activity-item activity-image">
    <div class="activity-title">
      <img
        class="user-image"
        :src="imageUrl(author.avatarKey, 'medium')"
        alt=" "
        @error="(e: any) => e.target.classList.add('image-error')"
      />

      <p>
        <router-link :to="{ name: 'UserProfile', params: { user: author.username } }">
          {{ author.displayName }}
        </router-link>

        uploaded {{ data.images.length }} {{ data.images.length > 1 ? "photos" : "photo" }}
      </p>

      <div class="photo-list">
        <img v-for="photo in take(data.images, PHOTO_LIMIT)" :key="photo.key" :src="imageUrl(photo.key)" alt=" " />

        <div class="img-additional" v-if="data.images.length > PHOTO_LIMIT">
          + {{ data.images.length - PHOTO_LIMIT }}
        </div>
      </div>
    </div>
  </div>
</template>
