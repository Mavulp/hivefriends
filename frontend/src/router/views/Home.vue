<script setup lang="ts">
import Button from "../../components/Button.vue"
// import { ref } from "vue"

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
      <p>Shut up idk what to write here.</p>

      <div class="album-thumbnail">
        <template v-if="latest">
          <div class="thumbnail-controls">
            <Button class="btn-highlight" :class="[TEXT_CONTRAST(accent[0], accent[1], accent[2])]">
              Latest Album
            </Button>

            by

            <div class="user">
              <router-link :to="{ name: 'UserProfile', params: { user: latest.author } }" class="user hover-bubble">
                <img
                  class="user-image"
                  :src="imageUrl(user.getUser(latest.author, 'avatarKey'), 'tiny')"
                  alt=" "
                  @error="(e: any) => e.target.classList.add('image-error')"
                />

                {{ user.getUsername(latest.author) }}
              </router-link>
            </div>
          </div>

          <img :src="imageUrl(latest.coverKey)" alt="" />
        </template>
      </div>
    </div>
  </div>
  <!-- <h1>hi!friends</h1>
    <p>
      Hello, welcome to hi!friends. Unfortunately homepage is the second last item on the backlog. Wanna see all the
      albums tho?
    </p>
    <Button :to="{ name: 'Albums' }" class="btn-red">View Albums</Button> -->
</template>
