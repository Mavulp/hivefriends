<script setup lang="ts">
import { computed } from "vue"
import { useRoute, useRouter } from "vue-router"
import { useAuth } from "../../store/auth"

const route = useRoute()
const router = useRouter()
const auth = useAuth()

// const bread = computed(() => {
//   return route.meta.title
// })

function signOut() {
  auth.signOut()
  router.push({ name: "Login" })
}
</script>

<template>
  <div class="hi-header">
    <div class="logo-wrap" title="嘿，伙计，我在哪里可以买到火腿和鸡蛋">
      <img src="/Sharp.png" alt="" />
    </div>

    <div class="flex-1"></div>

    <span class="bread">{{ route.meta.bread ?? route.meta.title }}</span>

    <div class="flex-1"></div>

    <!-- <span>Sign In to see your user</span> -->

    <template v-if="auth.isLoggedIn">
      <span class="user"> dolanske </span>

      <button data-title-bottom="Your albums">
        <span class="material-icons">&#xe413;</span>
      </button>
      <button data-title-bottom="Settings">
        <span class="material-icons">&#xe8b8;</span>
      </button>
      <button data-title-bottom="Log out" @click="signOut()">
        <span class="material-icons">&#xe9ba;</span>
      </button>
    </template>
  </div>
</template>
