<script setup lang="ts">
import { ref } from "vue"
import { useUser } from "../../store/user"

import { imageUrl } from "../../store/album"

const users = useUser()
</script>

<template>
  <div class="hi-user-page">
    <h1>{{ users.user.displayName ?? users.user.username }}</h1>

    <img :src="imageUrl(users.user.avatarKey)" alt="" />

    <div v-html="users.user.bio"></div>
  </div>
</template>
