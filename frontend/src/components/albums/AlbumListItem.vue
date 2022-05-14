<script setup lang="ts">
import { ref } from "vue"
import { imageUrl, Album } from "../../store/album"

interface Props {
  data: Album
}

const { data } = defineProps<Props>()
</script>

<template>
  <router-link :to="{ name: 'AlbumDetail', params: { id: data.key } }" class="album-list-item">
    <div class="album-image">
      <span class="album-draft" v-if="data.draft">Draft</span>
      <img :src="imageUrl(data.images[0].key, 'original')" alt="" />
    </div>

    <div class="album-meta">
      <h2>{{ data.title }}</h2>
      <!-- <span>by: dolanske</span> -->
      <p>
        {{ data.description }}
      </p>
    </div>
  </router-link>
</template>
