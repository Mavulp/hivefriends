<script setup lang="ts">
import "./style/index.scss"

import Navigation from "./components/navigation/Navigation.vue"
import Toasts from "./components/navigation/Toasts.vue"

import { onBeforeMount, onErrorCaptured, watchEffect } from "vue"
import { useUser } from "./store/user"

import { useLoading } from "./store/loading"
import { useRoute, useRouter } from "vue-router"

const user = useUser()
const router = useRouter()
const { addLoading, delLoading, getLoading } = useLoading()
const route = useRoute()

watchEffect(() => {
  if (user.logged) {
    getData()
  }
})

onBeforeMount(() => {
  document.title = "hi!friends"
  getData()
})

function getData() {
  addLoading("app")

  Promise.all([user.fetchUsers(), user.fetchSettings()]).then(() => {
    delLoading("app")

    const theme = user.settings.colorTheme ?? "light-theme"
    const r = document.querySelector(":root")
    if (r) {
      r.removeAttribute("class")
      r.classList.add(theme)
    }
  })
}
</script>

<template>
  <div class="hi-layout">
    <Navigation v-if="!route.meta.disableNav" />

    <Toasts />

    <div class="hi-router-layout">
      <!-- <router-view ></router-view> -->
      <router-view v-slot="{ Component }" v-if="!getLoading('app')">
        <transition name="pagetransition" mode="out-in">
          <component :is="Component" :key="route.path.includes('image') ? '' : route.fullPath" />
        </transition>
      </router-view>
    </div>
  </div>
</template>
