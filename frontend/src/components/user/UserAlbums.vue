<script setup lang="ts">
import { onBeforeMount, onMounted, ref } from "vue"
import { useRoute } from "vue-router"
// import { get, rootUrl } from "../../js/fetch"
import { useAlbums } from "../../store/album"

const store = useAlbums()
const route = useRoute()

const data = ref([])

onBeforeMount(async () => {
  const id = String(route.params.id)

  // route.meta.title = id + " albums"

  if (id) {
    data.value = await store.fetchUserAlbums(id)
  }
})

// store.fetchAlbum("0BpzhHisSUCNoqsNoB0SZg")

// // const image = {
// //   sizeSmall: "_link_",
// //   sizeMedium: "_link_",
// //   sizeLarge: "_link_",
// //   sizeThumbnail: "_link_"
// // }
</script>

<template>
  <div class="hi-album-list-user">
    <div class="album-list-title">
      <h2>Your Albums</h2>
      <span>{{ data.length }} Albums</span>
    </div>

    <!-- <pre>
      {{ data }}
    </pre> -->
  </div>
</template>
