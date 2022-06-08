<script setup lang="ts">
import { ref, computed } from "vue"
import { Album, useAlbums } from "../../store/album"
import { formatDate } from "../../js/utils"
import { onClickOutside } from "@vueuse/core"
import { useLoading } from "../../store/loading"

interface Props {
  data: Album
}

const albums = useAlbums()
const { getLoading } = useLoading()
const props = defineProps<Props>()
const open = ref(false)
const wrap = ref(null)

onClickOutside(wrap, () => {
  open.value = false
})

const timestamp = computed(() => {
  const d = new Date(props.data.createdAt * 1000)
  return d.toLocaleDateString("en-GB", {
    year: "numeric",
    month: "numeric",
    day: "numeric"
  })
})
</script>

<template>
  <div class="draft">
    <strong :title="props.data.title">{{ props.data.title }}</strong>
    <p>Created: {{ timestamp }}</p>

    <!-- This div does nothing but enforce a boundary for outside clicking detection -->
    <div ref="wrap">
      <button class="hover-bubble dropdown-button" :class="{ active: open }" @click="open = !open">
        <!-- <span class="material-icons"> &#xe5d2; </span> -->
        <span class="material-icons"> &#xe5d4; </span>
      </button>

      <div class="draft-dropdown" :class="{ active: open }">
        <router-link :to="{ name: 'AlbumDetail', params: { id: props.data.key } }" class="hover-bubble">
          <span class="material-icons">&#xe3b6;</span>
          View
        </router-link>

        <router-link :to="{ name: 'AlbumEdit', params: { id: props.data.key } }" class="hover-bubble bubble-info">
          <span class="material-icons">&#xe3c9;</span>
          Edit
        </router-link>

        <button
          class="hover-bubble bubble-red"
          :class="{ 'btn-disabled': getLoading('delete-album') }"
          @click="albums.deleteAlbum(props.data.key)"
        >
          <span class="material-icons"> &#xe872; </span>

          Delete

          <span class="material-icons rotate" v-if="getLoading('delete-album')">&#xe863;</span>
        </button>
      </div>
    </div>
  </div>
</template>
