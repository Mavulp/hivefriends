<script setup lang="ts">
import Navigation from "./components/navigation/Navigation.vue"
import "./style/index.scss"
</script>

<template>
  <!-- <div class="dev-screen">
    <h2>Under Consturction</h2>
    <p>Leave.</p>
  </div> -->

  <div class="irl-layout">
    <Navigation />

    <div class="irl-router-layout">
      <router-view></router-view>
    </div>
  </div>
</template>
