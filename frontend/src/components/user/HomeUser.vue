<script setup lang="ts">
import { User, useUser } from "../../store/user"
import { flag } from "../../js/utils"
import { imageUrl } from "../../store/album"

const { getUsername } = useUser()

interface Props {
  data: User
}

const { data } = defineProps<Props>()
</script>

<template>
  <router-link class="home-user" :to="{ name: 'UserProfile', params: { user: data.username } }">
    <img
      class="user-image"
      :src="imageUrl(data.avatarKey, 'tiny')"
      alt=" "
      :style="{ backgroundColor: `rgb(${data.accentColor})` }"
      @error="(e: any) => e.target.classList.add('image-error')"
    />

    <h5>{{ getUsername(data.username) }}</h5>

    <img class="flag" :src="flag('eu')" alt="" data-title-top="Eu" />
    <div class="flex-1"></div>

    <button class="hover-bubble">View Profile</button>
  </router-link>
</template>
