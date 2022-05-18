<script setup lang="ts">
import { ref, computed } from "vue"
import { useUser, User } from "../../store/user"

import { imageUrl } from "../../store/album"
import { useRoute } from "vue-router"

const users = useUser()
const route = useRoute()

const user = computed<User>(() => users.users.find((item) => item.key === `${route.params.id}`) as User)
</script>

<template>
  <div class="hi-user-page hi-user-profile" v-if="user">
    <h1>{{ user.displayName ?? user.username }}</h1>
    <!-- <pre>
      {{ user }}
    </pre> -->
    <!-- 

    <hr />
    <pre>
      {{ settings }}
    </pre> -->

    <img :src="imageUrl(user.avatarKey)" alt="" />

    <div v-html="user.bio"></div>

    <img :src="imageUrl(user.bannerKey)" alt="" />
  </div>
</template>
