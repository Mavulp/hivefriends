<script setup lang="ts">
import { useRoute, useRouter } from "vue-router"
import { useAuth } from "../../store/auth"

const route = useRoute()
const router = useRouter()
const auth = useAuth()

function signOut() {
  auth.signOut()
  router.push({ name: "Login" })
}
</script>

<template>
  <div class="hi-header">
    <router-link :to="{ name: 'Home' }" class="logo-wrap" title="嘿，伙计，我在哪里可以买到火腿和鸡蛋">
      <img src="/Sharp.png" alt="" />
    </router-link>

    <div class="flex-1"></div>

    <span class="bread">{{ route.meta.bread ?? route.meta.title }}</span>

    <div class="flex-1"></div>

    <template v-if="auth.isLoggedIn">
      <router-link
        class="hover-bubble"
        data-title-bottom="Your profile"
        :to="{ name: 'UserProfile', params: { id: auth.user.username } }"
      >
        <span class="user"> {{ auth.user.username }} </span>
      </router-link>

      <router-link class="hover-bubble" data-title-bottom="Upload Album" :to="{ name: 'Upload' }">
        <span class="material-icons">&#xe2cc;</span>
      </router-link>

      <router-link
        class="hover-bubble"
        data-title-bottom="Your albums"
        :to="{ name: 'UserAlbums', params: { id: auth.user.username } }"
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
