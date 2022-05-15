<script setup lang="ts">
import { imageUrl, Album } from "../../store/album"
import AlbumTimestamp from "../../components/albums/AlbumTimestamp.vue"

interface Props {
  data: Album
}

const { data } = defineProps<Props>()
</script>

<template>
  <router-link :to="{ name: 'AlbumDetail', params: { id: data.key } }" class="album-list-item">
    <div class="album-image">
      <span class="album-draft" v-if="data.draft">Draft</span>
      <img :src="imageUrl(data.images[0].key, 'medium')" alt="" />
    </div>

    <div class="album-meta">
      <AlbumTimestamp class="dark-light" :timeframe="data.timeframe" />
      <h2>{{ data.title }}</h2>
      <p>{{ data.description }}</p>
    </div>
  </router-link>
</template>
