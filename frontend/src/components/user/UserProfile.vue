<script setup lang="ts">
import LoadingSpin from "../loading/LoadingSpin.vue"
import AlbumListItem from "../albums/AlbumListItem.vue"

import { computed, onBeforeMount, onBeforeUnmount, ref, watchEffect } from "vue"
import { useUser, User } from "../../store/user"
import { Album, imageUrl, useAlbums } from "../../store/album"
import { useRoute } from "vue-router"
import { TEXT_CONTRAST, formatDate } from "../../js/utils"
import { useLoading } from "../../store/loading"
import { useCssVar } from "@vueuse/core"
import Button from "../Button.vue"

const { addLoading, delLoading, getLoading } = useLoading()
const users = useUser()
const route = useRoute()
const albums = useAlbums()

const wrap = ref(null)
const color = useCssVar("--color-highlight", wrap)
const _id = computed(() => `${route.params.user}`)
const userAlbums = ref<Array<Album>>([])
const bgscrollpos = ref("50%")
const imgheight = 456

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

onBeforeUnmount(() => {
  window.removeEventListener("scroll", () => {})
})

watchEffect(() => {
  if (user.value) {
    color.value = user.value.accentColor ?? "128,128,128"
  }
})

window.addEventListener("scroll", () => {
  const top = window.scrollY

  if (top < imgheight) {
    bgscrollpos.value = `calc(50% - ${top / 2}px)`
  }
})
</script>

<template>
  <div class="hi-user-profile" ref="wrap">
    <LoadingSpin class="center-page dark" v-if="getLoading('user-profile')" />

    <div v-else-if="!user" class="hi-user-profile" style="padding: 128px">
      <h1>Bruh</h1>
      <p>Unfortunately user does not exist</p>
    </div>

    <template v-else>
      <div class="user-profile-upper" :class="[TEXT_CONTRAST(accent[0], accent[1], accent[2])]">
        <div class="hi-user-banner">
          <!-- <img
            class="banner"
            :src="imageUrl(user.bannerKey, 'large')"
            alt=" "
            @error="(e: any) => e.target.classList.add('image-error')"
          /> -->

          <div
            class="banner"
            :style="{
              backgroundImage: `url(${imageUrl(user.bannerKey, 'large')})`,
              backgroundPositionY: bgscrollpos
            }"
          />

          <div class="avatar-wrap">
            <img
              class="avatar"
              :src="imageUrl(user.avatarKey, 'large')"
              alt=" "
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
          <p v-html="user.bio"></p>
        </div>
      </div>
      <div class="user-albums">
        <div class="albums-title-wrap">
          <h2>Latest albums</h2>
          <Button
            :class="[TEXT_CONTRAST(accent[0], accent[1], accent[2])]"
            class="btn-highlight"
            :to="{ name: 'UserAlbums', params: { user: _id } }"
            >All Albums</Button
          >
        </div>
        <div class="user-albums-list">
          <template v-if="userAlbums.length > 0">
            <AlbumListItem v-for="item in [...userAlbums].slice(0, 2)" :data="item" />
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
