<script setup lang="ts">
import { computed, onBeforeMount, ref } from "vue"
import { useRoute } from "vue-router"
import { Album, useAlbums } from "../../store/album"
import { useLoading } from "../../store/loading"

import LoadingSpin from "../../components/loading/LoadingSpin.vue"

const route = useRoute()
const albums = useAlbums()
const { addLoading, getLoading, delLoading } = useLoading()

const _id = computed(() => `${route.params.id}`)
const album = ref<Album>()

onBeforeMount(async () => {
  addLoading("edit")

  Promise.all([albums.fetchAlbums(true), albums.fetchAlbums()])
    .then((datasets) => {
      album.value = datasets.flat().find((item: Album) => item.key === _id.value)
    })
    .finally(() => {
      delLoading("edit")
    })
})
</script>

<template>
  <div class="hi-album-upload album-edit">
    <LoadingSpin v-if="getLoading('edit')" class="center-page dark" />

    <template v-else-if="false"> no data </template>
    <template v-else>
      <h1>edit albme</h1>

      <p>can edit, delete, and whatnot</p>

      <pre>
        {{ album }}
      </pre>
    </template>
  </div>
</template>
