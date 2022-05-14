<script setup lang="ts">
import { onBeforeMount, computed } from "vue"
import { useAlbums, Album } from "../../store/album"
// import AlbumListItem from "../albums/AlbumListItem.vue"
import AlbumListItem from "../../components/albums/AlbumListItem.vue"

const albums = useAlbums()
// const data = reactive<Album | {}>({})

const data = computed(() => albums.getAlbums)

onBeforeMount(() => {
  albums.fetchAlbums()
})
</script>

<template>
  <div class="hi-album-list">
    <template v-if="data">
      <AlbumListItem v-for="album in data" :data="album" :key="album.key" />
    </template>
  </div>
</template>
