<script setup lang="ts">
import { computed } from "vue"
import { imageUrl } from "../../../store/album.js"
import { User, useUser } from "../../../store/user.js"
import { ReducedImage } from "../../../store/activity"
import { take } from "lodash"
import { useRouter, useRoute } from "vue-router"
import { RGB_TO_HEX } from "../../../js/utils"

const user = useUser()
const router = useRouter()
const route = useRoute()
const props = defineProps<{ data: ReducedImage }>()
const data = computed(() => props.data)
const author = computed<User>(() => user.getUser(data.value.user))
const accent = computed(() => RGB_TO_HEX(author.value.accentColor))

const PHOTO_LIMIT = computed(() => {
  if (route.name === "Home") return 18
  return 8
})

function go(id: string) {
  if (!id) return

  router.push({
    name: "AlbumDetail",
    params: { id }
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
      />

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
          @click="photo.albumKeys ? go(photo.albumKeys[0]) : null"
        />

        <div class="img-additional" v-if="data.images.length > PHOTO_LIMIT">
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
