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

const store = useAlbums()
const route = useRoute()

const { getLoading } = useLoading()
const { getKey, getUsername } = useUser()

// const headerShrink = ref(false)
const data = ref<Array<Album>>([])
const search = ref("")
const open = ref(false)

onBeforeMount(async () => {
  const username = String(route.params.user)

  if (username) {
    data.value = await store.fetchUserAlbums(username)
  }
})

const sortedAlbums = computed(() => {
  if (!search.value || !data.value || data.value.length === 0) return data.value

  return data.value.filter((album) => {
    const searchString = `${album.title}`.toLowerCase()

    return searchString.includes(search.value.toLowerCase())
  })
})
</script>

<template>
  <div class="hi-album-list-user">
    <div class="album-list-title">
      <div class="title-wrap">
        <div class="inline-wrap">
          <h3>
            {{ route.params.id === getKey ? "Your albums" : `${getUsername(route.params.user)}'s albums` }}
          </h3>
          <!-- <Button @click="open = !open">{{ open ? "Close" : "Filter" }}</Button> -->
        </div>

        <div class="album-subtitle">
          <!-- <p>4 draft(s)</p> -->
          <p>{{ data?.length ?? 0 }} total</p>
          <p>{{ sortedAlbums?.length ?? 0 }} filtered</p>
        </div>

        <Search placeholder="Search for albums..." v-model:value="search" />
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
          <!-- <span class="material-icons">&#xe88b;</span> -->
          <h3>Lmao</h3>
          <p>No albums found</p>
          <Button class="center auto btn-black" :to="{ name: 'Upload' }">Add New?</Button>
        </div>
      </div>
      <template v-else>
        <AlbumListItem v-for="album in sortedAlbums" :data="album" :key="album.key" />
      </template>
    </div>
  </div>
</template>
