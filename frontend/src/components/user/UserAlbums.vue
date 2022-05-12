<script setup lang="ts">
import { onMounted, ref } from "vue"
import { get, rootUrl } from "../../js/fetch"
import { useAlbums } from "../../store/album"

// const store = useAlbums()

// store.fetchAlbum("0BpzhHisSUCNoqsNoB0SZg")

// // const image = {
// //   sizeSmall: "_link_",
// //   sizeMedium: "_link_",
// //   sizeLarge: "_link_",
// //   sizeThumbnail: "_link_"
// // }
</script>

<template>
  <div style="padding: 40px">
    <h2>User Albums</h2>

    <!-- <pre>
      {{ store.albums }}
    </pre>

    <div v-for="album in store.albums">
      <div v-for="image in album.images">
        <img :src="rootUrl + `/data/image/${image.key}/medium.png`" alt="" />
      </div>
    </div> -->
  </div>
</template>
