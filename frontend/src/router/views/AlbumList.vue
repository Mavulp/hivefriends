<script setup lang="ts">
import AlbumListItem from "../../components/albums/AlbumListItem.vue"
import LoadingSpin from "../../components/loading/LoadingSpin.vue"
import Filters from "../../components/form/Filters.vue"
// import Button from "../../components/Button.vue"
import Search from "../../components/form/Search.vue"

import { onBeforeMount, computed, ref } from "vue"
import { useAlbums, Album } from "../../store/album"
import { useLoading } from "../../store/loading"
import { useBread } from "../../store/bread"

const { getLoading } = useLoading()

const album = useAlbums()
const data = ref<Array<Album>>()
const bread = useBread()
const search = ref("")

// This is used to diasble loading after the initial load
// When filtering, we will use a spinner somewhere else
// to prevent flashing of the page
const init = ref(false)

onBeforeMount(async () => {
  bread.set("All public albums")

  data.value = await album.fetchAlbums()
  init.value = true
})

async function fetchUpdate() {
  data.value = await album.fetchAlbums()
}

const sortedAlbums = computed(() => {
  if (!search.value || !data.value || data.value.length === 0) return data.value

  return data.value.filter((album) => {
    const searchString = `${album.title} ${album.author}`.toLowerCase()

    return searchString.includes(search.value.toLowerCase())
  })
})
</script>

<template>
  <div class="hi-album-list">
    <div class="hi-album-list-layout">
      <div class="layout-item album-list-controls">
        <h1>Albums</h1>

        <div class="album-subtitle">
          <p>Showing {{ sortedAlbums?.length ?? 0 }} {{ sortedAlbums?.length === 1 ? "album" : "albums" }}</p>
        </div>

        <Search placeholder="Search for albums..." v-model:value="search" />
        <Filters @call="fetchUpdate" :loading="getLoading('albums') && init" />
      </div>

      <div class="layout-item">
        <div class="album-list-status" v-if="getLoading('albums') && !init">
          <div class="flex">
            <LoadingSpin class="dark" />
            <h3>Loading</h3>
          </div>
        </div>
        <div class="album-list-status" v-else-if="data?.length === 0 || !data">
          <div>
            <h3>Cringe</h3>
            <p>No albums found</p>
          </div>
        </div>
        <div class="album-list-wrap" v-else>
          <AlbumListItem v-for="album in sortedAlbums" :data="album" :key="album.key" />
        </div>
      </div>
    </div>
  </div>
</template>
