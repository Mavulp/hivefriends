<script setup lang="ts">
import { computed, onBeforeMount, ref } from 'vue'
import { useRoute } from 'vue-router'
import type { Album } from '../../store/album'
import { useAlbums } from '../../store/album'
import { useLoading } from '../../store/loading'

import LoadingSpin from '../loading/LoadingSpin.vue'
import AlbumListItem from '../albums/AlbumListItem.vue'

import Search from '../form/Search.vue'
import { useUser } from '../../store/user'
import { useBread } from '../../store/bread'
import { useThresholdScroll } from '../../js/_composables'

const { getLoading } = useLoading()
const store = useAlbums()
const route = useRoute()
const user = useUser()
const bread = useBread()

const data = ref<Array<Album>>([])
const search = ref('')
const username = computed(() => String(route.params.user))

const init = ref(false)

onBeforeMount(async () => {
  bread.set(`${user.getUsername(username.value)}'s albums`)
  await queryAlbums()
  init.value = true
})

async function queryAlbums() {
  return Promise.all([store.fetchUserAlbums(username.value), store.fetchUserAlbums(username.value, true)]).then(
    (res) => {
      data.value = res.flat()
    },
  )
}

const sortedAlbums = computed(() => {
  if (!search.value || !data.value || data.value.length === 0)
    return data.value

  return data.value.filter((album) => {
    const searchString = `${album.title}`.toLowerCase()

    return searchString.includes(search.value.toLowerCase())
  })
})

const { scroll, passed } = useThresholdScroll(292)
</script>

<template>
  <div class="hi-album-list hi-albums-user">
    <div class="hi-album-list-layout">
      <div class="layout-item album-list-controls">
        <h1>
          Albums by
          <router-link :to="{ name: 'UserProfile', params: { user: route.params.user } }">
            {{ user.getUsername(route.params.user) }}
          </router-link>
        </h1>

        <Search v-model:value="search" placeholder="Search for albums..." />

        <div class="album-subtitle">
          <p>Showing {{ sortedAlbums.length ?? 0 }} {{ sortedAlbums.length === 1 ? "album" : "albums" }}</p>

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
          <AlbumListItem v-for="album in sortedAlbums" :key="album.key" :data="album" />
        </div>
      </div>
    </div>
  </div>
</template>
