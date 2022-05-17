<script setup lang="ts">
import "./style/index.scss"

import Navigation from "./components/navigation/Navigation.vue"
import Toasts from "./components/navigation/Toasts.vue"

import { onBeforeMount } from "vue"
import { useUser } from "./store/user"

import { useLoading } from "./store/loading"
import { useRoute } from "vue-router"

const auth = useUser()
const loading = useLoading()
const route = useRoute()

onBeforeMount(() => {
  loading.addLoading("app")

  Promise.allSettled([auth.fetchUsers(), auth.fetchSettings]).then(() => {
    loading.delLoading("app")
  })

  document.title = "hi!friends"
})
</script>

<template>
  <div class="hi-layout">
    <Navigation v-if="!route.meta.disableNav" />

    <Toasts />

    <div class="hi-router-layout">
      <router-view></router-view>
      <!-- <router-view v-slot="{ Component }">
        <transition name="pagetransition" mode="out-in">
          <component :is="Component" :key="route.fullPath" />
        </transition>
      </router-view> -->
    </div>
  </div>
</template>
