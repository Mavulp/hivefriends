<script setup lang="ts">
import { computed } from "vue"
import { Image, useAlbums } from "../../../store/album.js"
import { sanitize, formatDate, RGB_TO_HEX } from "../../../js/utils"
import { User, useUser } from "../../../store/user.js"

const user = useUser()
const albums = useAlbums()
const props = defineProps<{ data: Image }>()

const data = computed(() => props.data)
</script>

<template>
  <div class="activity-item activity-image" v-if="false">
    <!-- <div class="activity-title">
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
        commented on "<router-link :to="{ name: 'AlbumDetail', params: { id: data.albumKey } }">{{
          album.title
        }}</router-link
        >" by
        <router-link :to="{ name: 'AlbumDetail', params: { id: album.author } }">{{
          user.getUsername(album.author)
        }}</router-link>
        on <i>{{ formatDate(data.createdAt, ["weekday", "year"]) }}</i>
      </p>
    </div> -->

    <!-- <div class="background" :style="{ backgroundColor: RGB_TO_HEX(author.accentColor) }"></div> -->
  </div>
</template>
