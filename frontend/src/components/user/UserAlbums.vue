<script setup lang="ts">
import { computed, onBeforeMount, onBeforeUnmount, ref } from "vue"
import { useRoute } from "vue-router"
import { useAlbums, Album } from "../../store/album"
import { useLoading } from "../../store/loading"

import Button from "../Button.vue"
import Filters from "../form/Filters.vue"
import LoadingSpin from "../loading/LoadingSpin.vue"
import AlbumListItem from "../albums/AlbumListItem.vue"

import Search from "../form/Search.vue"
import { useUser } from "../../store/user"
import { useBread } from "../../store/bread"

const { getLoading } = useLoading()
const store = useAlbums()
const route = useRoute()
const user = useUser()
const bread = useBread()

const data = ref<Array<Album>>([])
const search = ref("")
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
    }
  )
}

const sortedAlbums = computed(() => {
  if (!search.value || !data.value || data.value.length === 0) return data.value

  return data.value.filter((album) => {
    const searchString = `${album.title}`.toLowerCase()

    return searchString.includes(search.value.toLowerCase())
  })
})
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

        <div class="album-subtitle">
          <p>Showing {{ sortedAlbums.length ?? 0 }} {{ sortedAlbums.length === 1 ? "album" : "albums" }}</p>
        </div>

        <Search placeholder="Search for albums..." v-model:value="search" />
        <!-- <hr /> -->
        <Filters
          class="active"
          :disable="['authors']"
          @call="queryAlbums()"
          :filters="{ authors: [username] }"
          :loading="getLoading('albums') && init"
        />
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
