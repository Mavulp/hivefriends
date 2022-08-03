<script setup lang="ts">
import { computed } from "vue"
import { Album, imageUrl, useAlbums } from "../../../store/album.js"
import { useUser, User } from "../../../store/user.js"
import { formatDate, RGB_TO_HEX } from "../../../js/utils"
import { useRouter } from "vue-router"

const router = useRouter()
const user = useUser()

const props = defineProps<{ data: Album }>()

const data = computed(() => props.data)
const author = computed<User>(() => user.getUser(data.value.author))

function go() {
  router.push({
    name: "AlbumDetail",
    params: { id: data.value.key }
  })
}
</script>

<template>
  <div class="activity-item activity-album" v-if="data" @click.self="go">
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
        uploaded an album "<router-link :to="{ name: 'AlbumDetail', params: { id: data.key } }">{{
          data.title
        }}</router-link
        >" on <i>{{ formatDate(data.publishedAt, ["weekday", "year"]) }}</i>
      </p>
    </div>

    <div class="activity-body">
      <img :src="imageUrl(data.coverKey, 'medium')" alt=" " />

      <div class="title">
        <strong>{{ data.title }}</strong>
        <p>{{ data.description }}</p>

        <div class="info" v-if="data.taggedUsers.length > 1">
          <span class="material-icons">&#xe7fb;</span>
          {{ data.taggedUsers.length }} people
        </div>
      </div>
    </div>

    <div class="background" :style="{ backgroundColor: RGB_TO_HEX(author.accentColor) }"></div>
  </div>
</template>
