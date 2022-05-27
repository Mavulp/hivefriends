<script setup lang="ts">
import "./style/index.scss"

import Navigation from "./components/navigation/Navigation.vue"
import Toasts from "./components/navigation/Toasts.vue"
import LoadingSpin from "./components/loading/LoadingSpin.vue"

import { watch, computed, ref } from "vue"
import { useUser } from "./store/user"

import { useLoading } from "./store/loading"
import { useRoute } from "vue-router"
import { useToast } from "./store/toast"

const user = useUser()
const { addLoading, delLoading, getLoading } = useLoading()
const route = useRoute()
const toast = useToast()

const islogged = computed(() => user.logged)
const isInit = ref(false)

watch(
  islogged,
  (val) => {
    if (val && !isInit.value) {
      addLoading("app")

      Promise.all([user.fetchUsers(), user.fetchSettings()])
        .then(() => {
          const theme = user.settings.colorTheme ?? "light-theme"
          const r = document.querySelector(":root")
          if (r) {
            r.removeAttribute("class")
            r.classList.add(theme)
          }

          delLoading("app")
        })
        .catch((e) => {
          toast.add(e.message, "error")
        })
        .finally(() => {
          isInit.value = true
        })
    }
  },
  { immediate: true }
)
</script>

<template>
  <div class="hi-layout">
    <Navigation v-if="!route.meta.disableNav" />

    <Toasts />

    <LoadingSpin v-if="getLoading('app')" class="dark center-page" />

    <div class="hi-router-layout" v-else>
      <router-view v-slot="{ Component }">
        <transition name="pagetransition" mode="out-in">
          <component :is="Component" :key="route.path.includes('image') ? '' : route.fullPath" />
        </transition>
      </router-view>
    </div>
  </div>
</template>
