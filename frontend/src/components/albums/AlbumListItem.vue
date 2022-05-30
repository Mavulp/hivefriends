<script setup lang="ts">
import { imageUrl, Album } from "../../store/album"
import AlbumTimestamp from "../../components/albums/AlbumTimestamp.vue"
import { useUser } from "../../store/user"

const user = useUser()

interface Props {
  data: Album
  author?: string
}

const { data, author } = defineProps<Props>()
</script>

<template>
  <router-link :to="{ name: 'AlbumDetail', params: { id: data.key } }" class="album-list-item">
    <div class="album-image">
      <img :src="imageUrl(data.coverKey, 'medium')" alt=" " />
    </div>

    <div class="album-meta">
      <AlbumTimestamp class="dark-light" :timeframe="data.timeframe" />
      <div class="draft-wrap">
        <span class="album-draft" v-if="data.draft">Draft</span>
      </div>

      <h2>{{ data.title }}</h2>

      <template v-if="author">
        <div class="album-author">
          <img
            class="user-image"
            :src="imageUrl(user.getUser(author, 'avatarKey'), 'tiny')"
            :style="[`backgroundColor: rgb(${user.getUser(author, 'accentColor')})`]"
            alt=" "
            @error="(e: any) => e.target.classList.add('image-error')"
          />

          <span>{{ author }}</span>
        </div>
      </template>
      <p v-else>{{ data.description }}</p>
    </div>
  </router-link>
</template>
