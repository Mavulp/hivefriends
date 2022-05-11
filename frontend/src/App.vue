<script setup lang="ts">
import Navigation from "./components/navigation/Navigation.vue"
import Toasts from "./components/navigation/Toasts.vue"
import "./style/index.scss"
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
