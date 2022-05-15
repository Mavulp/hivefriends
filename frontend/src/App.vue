<script setup lang="ts">
import "./style/index.scss"

import { onBeforeMount } from "vue"
import { useAuth } from "./store/auth"

import Navigation from "./components/navigation/Navigation.vue"
import Toasts from "./components/navigation/Toasts.vue"
import { useLoading } from "./store/loading"

const auth = useAuth()
const loading = useLoading()

onBeforeMount(() => {
  loading.addLoading("app")

  Promise.allSettled([auth.fetchUsers()]).then(() => {
    loading.delLoading("app")
  })

  document.title = "hi!friends"
})
</script>

<template>
  <div class="hi-layout">
    <Navigation />

    <Toasts />

    <div class="hi-router-layout">
      <router-view></router-view>
    </div>
  </div>
</template>
