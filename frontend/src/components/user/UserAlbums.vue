<script setup lang="ts">
import { computed, onBeforeMount, onBeforeUnmount, ref } from "vue"
import { useRoute } from "vue-router"
import { useAlbums, Album } from "../../store/album"
import Button from "../Button.vue"
import Filters from "../form/Filters.vue"
import LoadingSpin from "../loading/LoadingSpin.vue"

import AlbumListItem from "../albums/AlbumListItem.vue"
import { useLoading } from "../../store/loading"

// TODO:
// - split into components (header)
// - reuse for all album lists

const store = useAlbums()
const route = useRoute()
const { getLoading } = useLoading()

const headerShrink = ref(false)
const data = ref<Array<Album>>([])
const search = ref("")
const open = ref(false)

onBeforeMount(async () => {
  const id = String(route.params.id)

  if (id) {
    data.value = await store.fetchUserAlbums(id)
  }

  document.addEventListener("scroll", handleScroll)
})

onBeforeUnmount(() => {
  document.removeEventListener("scroll", handleScroll)
})

function handleScroll() {
  if (window.scrollY > 56) {
    headerShrink.value = true
  } else {
    headerShrink.value = false
  }
}

const sortedAlbums = computed(() => {
  if (!search.value) return data.value

  return data.value.filter((album) => {
    const searchString = `${album.title}`.toLowerCase()

    return searchString.includes(search.value.toLowerCase())
  })
})
</script>

<template>
  <div class="hi-album-list-user alt">
    <div class="album-list-title" :class="{ shrink: headerShrink }">
      <div class="title-wrap">
        <div class="inline-wrap">
          <h3>Your Albums</h3>
          <!-- <Button @click="open = !open">{{ open ? "Close" : "Filter" }}</Button> -->
        </div>

        <div class="album-subtitle">
          <!-- <p>4 draft(s)</p> -->
          <p>{{ data?.length ?? 0 }} total</p>
          <p>{{ sortedAlbums?.length ?? 0 }} filtered</p>
        </div>

        <div class="album-list-search" :class="{ 'has-input': search }">
          <span class="material-icons">&#xe8b6;</span>
          <input v-model="search" placeholder="Search for an album..." />
        </div>
      </div>

      <Filters :class="{ active: open }" />
    </div>

    <div class="album-list-content">
      <div class="album-list-status" v-if="getLoading('albums')">
        <div class="flex">
          <LoadingSpin class="dark" />
          <h3>Loading</h3>
        </div>
      </div>
      <div class="album-list-status" v-else-if="data?.length === 0 || !data">
        <div>
          <h3><span class="material-icons">&#xe88b;</span>Lmao</h3>
          <p>No albums found</p>
        </div>
      </div>
      <template v-else>
        <AlbumListItem v-for="album in sortedAlbums" :data="album" :key="album.key" />
      </template>
    </div>
  </div>
</template>
