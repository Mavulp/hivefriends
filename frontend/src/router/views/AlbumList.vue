<script setup lang="ts">
import { computed, onBeforeMount, ref } from 'vue'
import AlbumListItem from '../../components/albums/AlbumListItem.vue'
import LoadingSpin from '../../components/loading/LoadingSpin.vue'

// import Button from "../../components/Button.vue"
import Search from '../../components/form/Search.vue'

import type { Album } from '../../store/album'
import { useAlbums } from '../../store/album'
import { useLoading } from '../../store/loading'
import { useBread } from '../../store/bread'
import { useThresholdScroll } from '../../js/_composables'

const { getLoading } = useLoading()

const album = useAlbums()
const data = ref<Array<Album>>()
const bread = useBread()
const search = ref('')

// This is used to diasble loading after the initial load
// When filtering, we will use a spinner somewhere else
// to prevent flashing of the page
const init = ref(false)

onBeforeMount(async () => {
  bread.set('All public albums')

  data.value = await album.fetchAlbums()
  init.value = true
})

// async function fetchUpdate() {
//   data.value = await album.fetchAlbums()
// }

const sortedAlbums = computed(() => {
  if (!search.value || !data.value || data.value.length === 0)
    return data.value

  return data.value.filter((album) => {
    const searchString = `${album.title} ${album.author}`.toLowerCase()

    return searchString.includes(search.value.toLowerCase())
  })
})

const { scroll, passed } = useThresholdScroll(292)
</script>

<template>
  <div class="hi-album-list">
    <div class="hi-album-list-layout">
      <div class="layout-item album-list-controls">
        <h1>Albums</h1>

        <Search v-model:value="search" placeholder="Search for albums..." />

        <div class="album-subtitle">
          <p>Showing {{ sortedAlbums?.length ?? 0 }} {{ sortedAlbums?.length === 1 ? "album" : "albums" }}</p>

          <button :class="{ active: passed }" class="go-up" data-title-bottom="Scroll Up" @click="scroll">
            <span class="material-icons"> &#xe5d8; </span>
          </button>
        </div>
      </div>

      <div class="layout-item">
        <div v-if="getLoading('albums') && !init" class="album-list-status">
          <div class="flex">
            <LoadingSpin class="dark" />
            <h3>Loading</h3>
          </div>
        </div>
        <div v-else-if="data?.length === 0 || !data" class="album-list-status">
          <div>
            <h3>Cringe</h3>
            <p>No albums found</p>
          </div>
        </div>
        <div v-else class="album-list-wrap">
          <AlbumListItem v-for="item in sortedAlbums" :key="item.key" :data="item" />
        </div>
      </div>
    </div>
  </div>
</template>
