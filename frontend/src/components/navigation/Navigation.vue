<script setup lang="ts">
import { computed, ref } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { onClickOutside, useMediaQuery } from '@vueuse/core'
import { useUser } from '../../store/user'
import { useBread } from '../../store/bread'
import { imageUrl } from '../../store/album'

import Modal from '../Modal.vue'

const router = useRouter()
const route = useRoute()
const auth = useUser()
const bread = useBread()

function signOut() {
  auth.signOut()
  router.push({ name: 'Login' })
}

// Phone menu opener
const open = ref(false)
const dropdown = ref(null)
const isPhone = useMediaQuery('(max-width: 512px)')

onClickOutside(dropdown, () => (open.value = false))

const isDark = computed(() => auth.settings.colorTheme === 'dark-normal')
</script>

<template>
  <div class="hi-header" :class="{ 'is-phone': isPhone, 'is-detail': ['AlbumDetail'].includes(String(route.name)) }">
    <router-link :to="{ name: 'Home' }" class="logo-wrap" title="嘿，伙计，我在哪里可以买到火腿和鸡蛋">
      <img :src="isDark ? '/Sharp.png' : '/Sharp2.png'" alt=" ">
    </router-link>

    <template v-if="!isPhone">
      <div class="nav-links-wrap">
        <router-link class="nav-link" :to="{ name: 'Home' }">
          Home
        </router-link>
        <!-- <router-link class="nav-link" :to="{ name: 'Feed' }">
          Feed
        </router-link> -->
        <router-link class="nav-link" :to="{ name: 'Albums' }">
          Albums
        </router-link>
        <router-link class="nav-link" :to="{ name: 'RouteActivity' }">
          Activity
        </router-link>
        <router-link class="nav-link" :to="{ name: 'About' }">
          About
        </router-link>
        <!--
        <router-link class="nav-link" :to="{ name: 'Upload' }">
          <span class="material-icons">&#xe2cc;</span>
          Upload
        </router-link> -->
      </div>

      <transition name="fade" appear mode="out-in">
        <span v-if="bread.title" class="bread">{{ bread.title }}</span>
      </transition>

      <div class="flex-1" />

      <template v-if="auth.isLoggedIn && auth.user.username">
        <router-link
          class="hover-bubble"
          data-title-bottom="Your profile"
          :to="{ name: 'UserProfile', params: { user: auth.user.username } }"
        >
          <img
            class="user-image"
            :src="imageUrl(auth.user.avatarKey, 'tiny')"
            alt=" "
            @error="(e: any) => e.target.classList.add('image-error')"
          >
          <span class="user"> {{ auth.getUsername() }} </span>
        </router-link>

        <router-link class="hover-bubble btn-icon" data-title-bottom="Upload album" :to="{ name: 'Upload' }">
          <span class="material-icons">&#xe2cc;</span>
        </router-link>

        <router-link
          class="hover-bubble btn-icon"
          data-title-bottom="Your albums"
          :to="{ name: 'UserAlbums', params: { user: auth.user.username } }"
        >
          <span class="material-icons">&#xe413;</span>
        </router-link>
        <router-link class="hover-bubble btn-icon" data-title-bottom="Your photos" :to="{ name: 'UserImages' }">
          <span class="material-icons">&#xe5c3;</span>
        </router-link>
        <router-link class="hover-bubble btn-icon" data-title-bottom="Settings" :to="{ name: 'UserSettings' }">
          <span class="material-icons">&#xe8b8;</span>
        </router-link>
        <button class="hover-bubble btn-icon" data-title-bottom-right="Log out" @click="signOut()">
          <span class="material-icons">&#xe9ba;</span>
        </button>
      </template>
    </template>

    <template v-else>
      <div class="flex-1" />

      <transition name="fade" appear mode="out-in">
        <span v-if="bread.title" class="bread">{{ bread.title }}</span>
      </transition>
      <div class="flex-1" />

      <div ref="dropdown" class="hi-phone-header-wrapper">
        <button class="hover-bubble" :class="{ active: open }" @click="open = !open">
          <span class="material-icons">&#xe5d2;</span>
        </button>

        <!-- <Teleport to="body"> -->
        <transition name="fade" mode="out-in">
          <Modal v-if="open" @close="open = false">
            <div class="hi-header-phone">
              <div class="header-phone-top">
                <h4>Menu</h4>

                <button @click="open = false">
                  <span class="material-icons">&#xe5cd;</span>
                </button>
              </div>

              <router-link :to="{ name: 'UserProfile', params: { user: auth.user.username } }" class="header-user">
                <img
                  class="user-image"
                  :src="imageUrl(auth.user.avatarKey, 'tiny')"
                  alt=" "
                  @error="(e: any) => e.target.classList.add('image-error')"
                >
                <!-- <div class="user-stuff"> -->
                <strong>{{ auth.getUsername() }}</strong>
              </router-link>

              <hr>

              <router-link class="nav-link-icon" :to="{ name: 'Upload' }">
                <span class="material-icons">&#xe2cc;</span>

                Upload album
              </router-link>

              <router-link class="nav-link-icon" :to="{ name: 'UserAlbums', params: { user: auth.user.username } }">
                <span class="material-icons">&#xe413;</span>

                Your albums
              </router-link>

              <router-link class="nav-link-icon" :to="{ name: 'UserImages' }">
                <span class="material-icons">&#xe5c3;</span>
                Your images
              </router-link>

              <router-link class="nav-link-icon" :to="{ name: 'UserSettings' }">
                <span class="material-icons">&#xe8b8;</span>
                Settings
              </router-link>

              <hr>

              <router-link class="nav-link" :to="{ name: 'Home' }">
                Home
              </router-link>
              <router-link class="nav-link" :to="{ name: 'Albums' }">
                Albums
              </router-link>
              <router-link class="nav-link" :to="{ name: 'About' }">
                About
              </router-link>

              <hr>

              <button class="nav-link-icon" @click="signOut()">
                <span class="material-icons">&#xe9ba;</span>
                Log out
              </button>
            </div>
          </Modal>
        </transition>
        <!-- </Teleport> -->
      </div>
    </template>
  </div>
</template>
