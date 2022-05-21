<script setup lang="ts">
import LoadingSpin from "../loading/LoadingSpin.vue"
import AlbumListItem from "../albums/AlbumListItem.vue"

import { computed, onBeforeMount, ref, watchEffect } from "vue"
import { useUser, User } from "../../store/user"
import { Album, imageUrl, useAlbums } from "../../store/album"
import { useRoute } from "vue-router"
import { TEXT_CONTRAST, formatDate } from "../../js/utils"
import { useLoading } from "../../store/loading"
import { useCssVar } from "@vueuse/core"

const { addLoading, delLoading, getLoading } = useLoading()
const users = useUser()
const route = useRoute()
const albums = useAlbums()

const wrap = ref(null)
const color = useCssVar("--color-highlight", wrap)
const _id = computed(() => `${route.params.user}`)
const userAlbums = ref<Array<Album>>([])

const user = computed<User>(() => users.users.find((item) => item.username === _id.value) as User)
const accent = computed(() => color.value.split(",").map((item) => Number(item)))

onBeforeMount(() => {
  addLoading("user-profile")

  Promise.all([albums.fetchUserAlbums(_id.value), users.fetchUser(_id.value, true)])
    .then(([albums]) => {
      userAlbums.value = albums
      color.value = user.value.accentColor
    })
    .catch(() => {})
    .finally(() => {
      delLoading("user-profile")
    })
})

watchEffect(() => {
  if (user.value) {
    color.value = user.value.accentColor ?? "128,128,128"
  }
})
</script>

<template>
  <div class="hi-user-profile" v-if="user" ref="wrap">
    <LoadingSpin class="center-page dark" v-if="getLoading('user-profile')" />

    <template v-else>
      <div class="user-profile-upper" :class="[TEXT_CONTRAST(accent[0], accent[1], accent[2])]">
        <div class="hi-user-banner">
          <img
            class="banner"
            :src="imageUrl(user.bannerKey)"
            alt=""
            @error="(e: any) => e.target.classList.add('image-error')"
          />

          <div class="avatar-wrap">
            <img
              class="avatar"
              :src="imageUrl(user.avatarKey, 'medium')"
              alt=""
              @error="(e: any) => e.target.classList.add('image-error')"
            />
          </div>
        </div>

        <div class="user-information">
          <h1>{{ user.displayName ?? user.username }}</h1>
          <div class="user-info-meta">
            <span>
              Joined <b>{{ formatDate(user.createdAt) }}</b>
            </span>
            <span>
              <b>{{ user.albumsUploaded.length }}</b> {{ user.albumsUploaded.length === 1 ? "album" : "albums" }}
            </span>
            <span>
              met <b>{{ user.met.length }}</b> people
            </span>
          </div>
          <p>{{ user.bio }}</p>
        </div>
      </div>
      <div class="user-albums">
        <h2>Albums</h2>
        <div class="user-albums-list">
          <template v-if="userAlbums.length > 0">
            <AlbumListItem v-for="item in userAlbums" :data="item" />
          </template>
          <div v-else>
            <p>
              Looks like <b>{{ user.displayName ?? user.username }}</b> did not upload any albums yet. Are they touching
              grass rn?
            </p>
          </div>
        </div>
      </div>
    </template>
  </div>
</template>
