<script setup lang="ts">
import Button from "../../components/Button.vue"
import HomeUser from "../../components/user/HomeUser.vue"

import { ref, onBeforeMount, computed } from "vue"
import { Album, useAlbums, imageUrl } from "../../store/album"
import { useUser } from "../../store/user"
import { TEXT_CONTRAST } from "../../js/utils"
import { useBread } from "../../store/bread"

const user = useUser()
const album = useAlbums()
const bread = useBread()
const albums = ref([])
const latest = computed<Album>(() => albums?.value[0] ?? null)

onBeforeMount(async () => {
  albums.value = await album.fetchAlbums()

  bread.set("Homepage | url to irl")
})

const accent = computed(() => user.user.accentColor.split(",").map((item: string) => Number(item)))
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

      <Button
        size="48px"
        pad="48px"
        class="btn-highlight"
        :to="{ name: 'Albums' }"
        :class="[TEXT_CONTRAST(accent[0], accent[1], accent[2])]"
      >
        Browse Albums
      </Button>

      <template v-if="latest">
        <router-link :to="{ name: 'AlbumDetail', params: { id: latest.key } }" class="album-thumbnail">
          <span>Latest album</span>
          <img :src="imageUrl(latest.coverKey)" alt="" />
        </router-link>
      </template>
    </div>

    <div class="container" v-if="user.users && user.users.length > 0">
      <h4>Friends</h4>

      <div class="home-users">
        <HomeUser v-for="item in user.users" :key="item.username" :data="item" />
      </div>
    </div>

    <p class="copyright">
      <span class="material-icons"> &#xe86f; </span>
      Made by <a target="_blank" href="https://github.com/mavulp">Mavulp</a> in {{ new Date().getFullYear() }}
    </p>
  </div>
</template>
