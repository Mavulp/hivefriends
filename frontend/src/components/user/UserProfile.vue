<script setup lang="ts">
import LoadingSpin from "../loading/LoadingSpin.vue"
import AlbumListItem from "../albums/AlbumListItem.vue"
import Button from "../Button.vue"

import { computed, onBeforeMount, onBeforeUnmount, onMounted, ref, watchEffect } from "vue"
import { useUser, User } from "../../store/user"
import { Album, imageUrl, useAlbums } from "../../store/album"
import { useRoute } from "vue-router"
import { TEXT_CONTRAST, formatDate, flag, sanitize } from "../../js/utils"
import { useLoading } from "../../store/loading"
import { useCssVar, useMediaQuery } from "@vueuse/core"
import countries from "../../js/countries"
import { useBread } from "../../store/bread"

const { addLoading, delLoading, getLoading } = useLoading()
const users = useUser()
const route = useRoute()
const albums = useAlbums()
const bread = useBread()

const wrap = ref(null)
const color = useCssVar("--color-highlight", wrap)
const isPhone = useMediaQuery("(max-width: 512px)")
const _id = computed(() => route?.params?.user?.toString() ?? null)
const userAlbums = ref<Array<Album>>([])
const bgscrollpos = ref("50%")
const imgheight = 512

const user = computed<User>(() => users.users.find((item) => item.username === _id.value) as User)
const accent = computed(() => color.value.split(",").map((item) => Number(item)))
// const flag = ref()

onBeforeMount(() => {
  addLoading("profile")

  bread.set(`${users.getUsername(_id.value)}'s profile`)

  Promise.all([albums.fetchUserAlbums(_id.value), users.fetchUser(_id.value, true)])
    .then(async ([albums]) => {
      userAlbums.value = albums
      color.value = user.value.accentColor
    })
    .catch(() => {})
    .finally(() => {
      delLoading("profile")
    })
})

onBeforeUnmount(() => {
  window.removeEventListener("scroll", () => {})
})

const strength = computed(() => (isPhone.value ? 1.2 : 2))

onMounted(() => {
  window.addEventListener("scroll", () => {
    const top = window.scrollY

    if (top < imgheight) {
      bgscrollpos.value = `calc(50% - ${top / strength.value}px)`
    }
  })
})

watchEffect(() => {
  if (user.value) {
    color.value = user.value.accentColor ?? "128,128,128"
  }
})
</script>

<template>
  <div class="hi-user-profile" ref="wrap">
    <LoadingSpin class="center-page dark" v-if="getLoading('profile')" />

    <div v-else-if="!user" class="hi-user-profile" style="padding: 128px">
      <h1>Bruh</h1>
      <p>Unfortunately user does not exist</p>
    </div>

    <template v-else>
      <div class="user-profile-upper" :class="[TEXT_CONTRAST(accent[0], accent[1], accent[2])]">
        <div class="hi-user-banner">
          <div
            class="banner"
            :style="{
              backgroundImage: `url(${imageUrl(user.bannerKey)})`,
              backgroundPositionY: bgscrollpos
            }"
          />
        </div>

        <div class="user-information-wrap">
          <div class="user-information">
            <h1>{{ user.displayName ?? user.username }}</h1>
            <div class="user-info-meta">
              <span v-if="user.country" :data-title-top="countries[user.country].name">
                <img class="flag" :src="flag(user.country, 'png')" alt="" />
              </span>
              <span>
                Joined <b>{{ formatDate(user.createdAt) }}</b>
              </span>
              <span>
                <b>{{ user.albumsUploaded.length }}</b> {{ user.albumsUploaded.length === 1 ? "album" : "albums" }}
              </span>
              <router-link
                :class="[TEXT_CONTRAST(accent[0], accent[1], accent[2])]"
                :to="{ name: 'UserAlbums', params: { user: user.username } }"
              >
                View All
              </router-link>
            </div>
            <p v-html="sanitize(user.bio)"></p>
          </div>

          <div class="avatar-wrap">
            <img
              class="avatar"
              :src="imageUrl(user.avatarKey, 'large')"
              alt=" "
              @error="(e: any) => e.target.classList.add('image-error')"
            />
          </div>
        </div>
      </div>

      <div class="user-expand">
        <div class="user-albums">
          <template v-if="userAlbums.length === 0">
            <p>
              Looks like <b>{{ user.displayName ?? user.username }}</b> did not upload any albums yet. Are they touching
              grass rn?
            </p>
          </template>
          <template v-else>
            <h2>Latest albums</h2>
            <div class="user-albums-list">
              <AlbumListItem v-for="item in [...userAlbums].slice(0, 3)" :data="item" />
            </div>
          </template>
        </div>
        <div class="user-met-with-wrap">
          <div class="user-met-with" v-if="user.met.length > 0">
            <h4>
              <span class="material-icons">&#xe7fb;</span>
              Met with
              <!-- {{ user.displayName ?? user.username }} has met -->
            </h4>

            <router-link
              v-for="item in user.met"
              :key="item"
              class="album-tagged-user"
              :to="{ name: 'UserProfile', params: { user: item } }"
            >
              <img
                class="user-image"
                :src="imageUrl(users.getUser(item, 'avatarKey'), 'tiny')"
                :style="[`backgroundColor: rgb(${users.getUser(item, 'accentColor')})`]"
                alt=" "
                @error="(e: any) => e.target.classList.add('image-error')"
              />
              <span>{{ users.getUsername(item) }}</span>
              <div class="tag tag-orange" v-if="item === user.username">Author</div>
              <div class="background"></div>
              <div class="background" :style="[`backgroundColor: rgb(${users.getUser(item, 'accentColor')})`]"></div>
            </router-link>
          </div>
        </div>
      </div>
    </template>
  </div>
</template>
