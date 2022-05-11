<script setup lang="ts">
// import MenuIcon from 'vue-material-design-icons/Menu.vue';

import { computed, ref } from "vue"
import { useRoute } from "vue-router"

const route = useRoute()

const bread = computed(() => {
  return route.meta.title
})
</script>

<template>
  <div class="hi-header">
    <div class="logo-wrap" title="嘿，伙计，我在哪里可以买到火腿和鸡蛋">
      <img src="/Sharp.png" alt="" />
    </div>

    <div class="flex-1"></div>

    <span class="bread">{{ bread }}</span>

    <div class="flex-1"></div>

    <!-- <span>Sign In to see your user</span> -->

    <span class="user"> dolanske </span>

    <button data-title-bottom="Your albums">
      <span class="material-icons">&#xe413;</span>
    </button>
    <button data-title-bottom="Settings">
      <span class="material-icons">&#xe8b8;</span>
    </button>
    <button data-title-bottom="Log out">
      <span class="material-icons">&#xe9ba;</span>
    </button>
  </div>
</template>
