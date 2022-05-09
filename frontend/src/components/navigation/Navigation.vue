<script setup lang="ts">
import { computed, ref } from "vue"
import { useRoute } from "vue-router"

const route = useRoute()

const bread = computed(() => {
  return route.meta.title
})
</script>

<template>
  <div class="hi-header">
    <div class="logo-wrap">
      <img src="/Sharp.png" alt="" />
    </div>

    <div class="flex-1"></div>

    <span class="bread">{{ bread }}</span>

    <div class="flex-1"></div>

    <span>Sign In to see your user</span>

    <!-- <span class="user"> dolanske </span>

    <button data-title-bottom="Your albums" class="material-symbols-rounded">photo_library</button>

    <button data-title-bottom="Settings" class="material-symbols-rounded">settings</button>

    <button data-title-bottom="Log out" class="material-symbols-rounded">logout</button> -->
  </div>
</template>
