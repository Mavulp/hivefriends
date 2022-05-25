<script setup lang="ts">
import Button from "../../components/Button.vue"
import HomeUser from "../../components/user/HomeUser.vue"

import { ref, onBeforeMount, computed } from "vue"
import { Album, useAlbums, imageUrl } from "../../store/album"
import { useUser } from "../../store/user"
import { TEXT_CONTRAST } from "../../js/utils"

const user = useUser()
const album = useAlbums()
const albums = ref([])
const latest = computed<Album>(() => albums.value[0])

onBeforeMount(async () => {
  albums.value = await album.fetchAlbums()
})

const accent = computed(() =>
  user
    .getUser(latest.value.author, "accentColor")
    .split(",")
    .map((item: string) => Number(item))
)
</script>

<template>
  <div class="hi-home">
    <div class="home-landing">
      <h1>hi<b>!</b>friends</h1>
      <h3>
        Internet friends <br />
        bringing the IRL <br />
        to the URL.
      </h3>
      <p>Say one word about my writing and you get hands.</p>

      <div class="album-thumbnail">
        <template v-if="latest">
          <Button
            :to="{ name: 'AlbumDetail', params: { id: latest.key } }"
            class="btn-highlight"
            :class="[TEXT_CONTRAST(accent[0], accent[1], accent[2])]"
          >
            Latest Album
          </Button>

          <img :src="imageUrl(latest.coverKey)" alt="" />
        </template>
      </div>
    </div>

    <div class="container">
      <h4>Friends</h4>

      <div class="home-users" v-if="user.users">
        <HomeUser v-for="item in user.users" :key="item.username" :data="item" />
      </div>
    </div>

    <p class="copyright">Copyright {{ new Date().getFullYear() }} Mavulp</p>
  </div>
</template>
