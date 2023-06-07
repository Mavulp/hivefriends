<script setup lang="ts">
import { computed, onBeforeMount, onBeforeUnmount, onMounted, ref, watchEffect } from 'vue'
import { useRoute } from 'vue-router'
import { useCssVar, useMediaQuery } from '@vueuse/core'
import dayjs from 'dayjs'
import LoadingSpin from '../loading/LoadingSpin.vue'
import AlbumListItem from '../albums/AlbumListItem.vue'

import type { User } from '../../store/user'
import { useUser } from '../../store/user'
import type { Album } from '../../store/album'
import { imageUrl, useAlbums } from '../../store/album'
import { TEXT_CONTRAST, flag, sanitize } from '../../js/utils'
import { useLoading } from '../../store/loading'
import countries from '../../js/countries'
import { useBread } from '../../store/bread'
import { timeDateFormat } from '../../js/time'

const { addLoading, delLoading, getLoading } = useLoading()
const users = useUser()
const route = useRoute()
const albums = useAlbums()
const bread = useBread()

const wrap = ref(null)
const color = useCssVar('--color-highlight', wrap)
const isPhone = useMediaQuery('(max-width: 512px)')
const _id = computed(() => route?.params?.user?.toString() ?? null)
const userAlbums = ref<Array<Album>>([])
const bgscrollpos = ref('50%')
const imgheight = 512

const user = computed<User>(() => users.users.find(item => item.username === _id.value) as User)
const accent = computed(() => color.value.split(',').map(item => Number(item)))
// const flag = ref()

onBeforeMount(() => {
  addLoading('profile')

  bread.set(`${users.getUsername(_id.value)}'s profile`)

  Promise.all([albums.fetchUserAlbums(_id.value), users.fetchUser(_id.value, true)])
    .then(async ([albums]) => {
      userAlbums.value = albums
      color.value = user.value.accentColor
    })
    .catch(() => {})
    .finally(() => {
      delLoading('profile')
    })
})

onBeforeUnmount(() => {
  window.removeEventListener('scroll', () => {})
})

const strength = computed(() => (isPhone.value ? -4 : -2.5))

onMounted(() => {
  window.addEventListener('scroll', () => {
    const top = window.scrollY

    if (top < imgheight)
      bgscrollpos.value = `calc(50% - ${top / strength.value}px)`
  })
})

watchEffect(() => {
  if (user.value)
    color.value = user.value.accentColor ?? '128,128,128'
})
</script>

<template>
  <div ref="wrap" class="hi-user-profile">
    <LoadingSpin v-if="getLoading('profile')" class="center-page dark" />

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
              backgroundPositionY: bgscrollpos,
            }"
          />
        </div>

        <div class="user-information-wrap">
          <div class="user-information-container">
            <div class="user-information">
              <h1>{{ user.displayName ?? user.username }}</h1>
              <p v-html="sanitize(user.bio)" />
            </div>

            <div class="avatar-wrap">
              <img
                class="avatar"
                :src="imageUrl(user.avatarKey, 'large')"
                alt=" "
                @error="(e: any) => e.target.classList.add('image-error')"
              >
            </div>
          </div>
        </div>
      </div>

      <div class="user-expand">
        <div class="user-albums">
          <template v-if="userAlbums.length === 0">
            <p>
              Looks like <b>{{ user.displayName ?? user.username }}</b> has not uploaded any albums yet.
            </p>
          </template>
          <template v-else>
            <h2>
              Latest albums

              <router-link
                v-if="user.albumsUploaded.length > 0"
                class="hover-bubble"
                :to="{ name: 'UserAlbums', params: { user: user.username } }"
              >
                View All
              </router-link>
            </h2>
            <div class="user-albums-list">
              <AlbumListItem v-for="item in [...userAlbums].slice(0, 3)" :key="item.key" :data="item" />
            </div>
          </template>
        </div>
        <div class="user-met-with-wrap">
          <div class="user-met-with user-details">
            <div v-if="user.country">
              <span>Country</span>

              <div class="nationality-wrap">
                <img class="flag" :src="flag(user.country)" :alt="user.country">
                <p>{{ countries[user.country].name }}</p>
              </div>
            </div>

            <div>
              <span>Joined</span>
              <p>{{ dayjs(user.createdAt * 1000).format(timeDateFormat) }}</p>
            </div>
          </div>

          <div v-if="user.met.length > 0" class="user-met-with">
            <h4>
              <span class="material-icons">&#xe7fb;</span>
              Met with
              <!-- <p>{{ user.met.length }} people</p> -->
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
              >
              <span>{{ users.getUsername(item) }}</span>
              <div v-if="item === user.username" class="tag tag-orange">
                Author
              </div>
              <!-- <div class="background"></div> -->
              <div class="background" :style="[`backgroundColor: rgb(${users.getUser(item, 'accentColor')})`]" />
            </router-link>
          </div>
        </div>
      </div>
    </template>
  </div>
</template>
