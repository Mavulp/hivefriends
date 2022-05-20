<script setup lang="ts">
import LoadingSpin from "../loading/LoadingSpin.vue"

import { computed, onBeforeMount } from "vue"
import { useUser, User } from "../../store/user"
import { imageUrl, useAlbums } from "../../store/album"
import { useRoute } from "vue-router"
import { TEXT_CONTRAST, formatDate } from "../../js/utils"
import { useLoading } from "../../store/loading"

const { addLoading, delLoading, getLoading } = useLoading()
const users = useUser()
const route = useRoute()
const albums = useAlbums()

const _id = computed(() => `${route.params.id}`)

const user = computed<User>(() => users.users.find((item) => item.key === _id.value) as User)
const accent = computed(() => user.value.accentColor.split(",").map((item) => Number(item)))
const userAlbums = computed(() => Object.keys(albums.userAlbums[_id.value]).length)

onBeforeMount(() => {
  addLoading("user-profile")

  Promise.all([albums.fetchUserAlbums(user.value.key), users.fetchUsers()])
    .then(() => {
      console.log(albums.userAlbums)
    })
    .catch(() => {})
    .finally(() => {
      delLoading("user-profile")
    })
})
</script>

<template>
  <div class="hi-user-profile" v-if="user">
    <LoadingSpin class="center-page" v-if="getLoading('user-profile')" />

    <template v-else>
      <div class="hi-user-banner">
        <img class="banner" :src="imageUrl(user.bannerKey, 'medium')" alt="" />

        <div class="avatar-wrap">
          <img class="avatar" :src="imageUrl(user.avatarKey, 'medium')" alt="" />

          <div class="avatar-info">
            <p>
              Joined <b>{{ formatDate(user.createdAt) }}</b>
            </p>
            <p>
              <b>{{ userAlbums }}</b> {{ userAlbums === 1 ? "album" : "albums" }}
            </p>
          </div>
        </div>
      </div>

      <div class="user-information">
        <h1>{{ user.displayName ?? user.username }}</h1>
        <p :class="[TEXT_CONTRAST(accent[0], accent[1], accent[2])]">{{ user.bio }}</p>
      </div>
    </template>
  </div>
</template>
