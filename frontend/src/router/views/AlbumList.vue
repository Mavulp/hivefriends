<script setup lang="ts">
import AlbumListItem from "../../components/albums/AlbumListItem.vue"
import LoadingSpin from "../../components/loading/LoadingSpin.vue"
// import Button from "../../components/Button.vue"

import { onBeforeMount, computed, ref } from "vue"
import { useAlbums, Album } from "../../store/album"
import { useLoading } from "../../store/loading"

const { getLoading } = useLoading()
const album = useAlbums()
const data = ref<Array<Album>>()

onBeforeMount(async () => {
  data.value = await album.fetchAlbums()
})
</script>

<template>
  <div class="hi-album-list">
    <div class="hi-album-list-layout">
      <div class="layout-item album-list-controls">
        <h1>Albums</h1>
        <p>All the available albums on hi!friends</p>

        <hr />
      </div>

      <div class="layout-item">
        <div class="album-list-status" v-if="getLoading('albums')">
          <div class="flex">
            <LoadingSpin class="dark" />
            <h3>Loading</h3>
          </div>
        </div>
        <div class="album-list-status" v-else-if="data?.length === 0 || !data">
          <div>
            <!-- <span class="material-icons">&#xe88b;</span> -->
            <h3>Cringe</h3>
            <p>No albums found</p>
          </div>
        </div>
        <div class="album-list-wrap" v-else>
          <AlbumListItem v-for="album in data" :data="album" :key="album.key" :author="album.author" />
        </div>
      </div>
    </div>
  </div>
</template>
