<script setup lang="ts">
import { useRoute, useRouter } from "vue-router"
import { useUser } from "../../store/user"
import { useBread } from "../../store/bread"
import { imageUrl } from "../../store/album"

const router = useRouter()
const auth = useUser()
const bread = useBread()

function signOut() {
  auth.signOut()
  router.push({ name: "Login" })
}
</script>

<template>
  <div class="hi-header">
    <router-link :to="{ name: 'Home' }" class="logo-wrap" title="嘿，伙计，我在哪里可以买到火腿和鸡蛋">
      <img src="/Sharp.png" alt=" " />
    </router-link>

    <div class="nav-links-wrap">
      <router-link class="nav-link" :to="{ name: 'Home' }">Home</router-link>
      <router-link class="nav-link" :to="{ name: 'Albums' }">Albums</router-link>
    </div>

    <transition name="fade" appear mode="out-in">
      <span class="bread" v-if="bread.title">{{ bread.title }}</span>
    </transition>

    <div class="flex-1"></div>

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
        />
        <span class="user"> {{ auth.getUsername() }} </span>
      </router-link>

      <router-link class="hover-bubble" data-title-bottom="Upload album" :to="{ name: 'Upload' }">
        <span class="material-icons">&#xe2cc;</span>
      </router-link>

      <router-link
        class="hover-bubble"
        data-title-bottom="Your albums"
        :to="{ name: 'UserAlbums', params: { user: auth.user.username } }"
      >
        <span class="material-icons">&#xe413;</span>
      </router-link>
      <router-link class="hover-bubble" data-title-bottom="Settings" :to="{ name: 'UserSettings' }">
        <span class="material-icons">&#xe8b8;</span>
      </router-link>
      <button class="hover-bubble" data-title-bottom="Log out" @click="signOut()">
        <span class="material-icons">&#xe9ba;</span>
      </button>
    </template>
  </div>
</template>
