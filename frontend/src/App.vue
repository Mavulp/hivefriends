<script setup lang="ts">
import "./style/index.scss"

import Navigation from "./components/navigation/Navigation.vue"
import Toasts from "./components/navigation/Toasts.vue"
import LoadingSpin from "./components/loading/LoadingSpin.vue"

import { watch, computed, ref, onMounted, onUpdated, nextTick } from "vue"
import { useUser } from "./store/user"
import { useLoading } from "./store/loading"
import { useRoute } from "vue-router"
import { useToast } from "./store/toast"
import router from "./router"
import { useMediaQuery } from "@vueuse/core"

const user = useUser()
const { addLoading, delLoading, getLoading } = useLoading()
const route = useRoute()
const toast = useToast()

const islogged = computed(() => user.logged)
const isInit = ref(false)
const isPhone = useMediaQuery("(max-width: 512px)")
const root = document.documentElement

watch(
  islogged,
  (val) => {
    if (val && !isInit.value) {
      addLoading("app")

      Promise.all([user.fetchUsers(), user.fetchSettings()])
        .then(() => delLoading("app"))
        .catch((e) => toast.add(e.message, "error"))
        .finally(() => {
          const theme = user.settings.colorTheme ?? "light-theme"
          const r = document.querySelector(":root")
          if (r) {
            r.removeAttribute("class")
            r.classList.add(theme)
          }

          isInit.value = true
        })
    }

    if (!val) {
      isInit.value = false
    }
  },
  { immediate: true }
)

onMounted(() => {
  document.addEventListener("click", (e: any) => {
    const attr = e.target.attributes["data-comment-link"]

    if (attr) {
      e.preventDefault()
      router.push({
        name: "UserProfile",
        params: { user: attr.value }
      })
    }
  })
})

watch(
  () => route.fullPath,
  () => {
    // If route is NOT one of these, change highlight color to default users

    if (
      !["AlbumDetail", "PublicAlbumDetail", "ImageDetail", "PublicImageDetail", "UserProfile"].includes(
        String(route.name)
      )
    ) {
      root.style.setProperty("--color-highlight", user.user.accentColor)
    }
  }
)
</script>

<template>
  <div class="hi-layout">
    <Navigation v-if="!route.meta.disableNav" />
    <!-- <Navigation /> -->

    <Toasts />

    <LoadingSpin v-if="getLoading('app')" class="dark center-page" />

    <div class="hi-router-layout" v-else>
      <router-view v-slot="{ Component }">
        <transition :name="isPhone ? 'fade' : 'pagetransition'" mode="out-in">
          <component :is="Component" :key="route.path.includes('image') ? '' : route.fullPath" />
        </transition>
      </router-view>
    </div>
  </div>
</template>
