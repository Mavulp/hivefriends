<script setup lang="ts">
import { User, useUser } from "../../store/user"
import { flag } from "../../js/utils"
import { imageUrl } from "../../store/album"
import countries from "../../js/countries"

const user = useUser()

interface Props {
  data: User
}

const props = defineProps<Props>()
</script>

<template>
  <router-link class="home-user" :to="{ name: 'UserProfile', params: { user: props.data.username } }">
    <img
      class="user-image"
      :src="imageUrl(props.data.avatarKey, 'tiny')"
      alt=" "
      :style="{ backgroundColor: `rgb(${props.data.accentColor})` }"
      @error="(e: any) => e.target.classList.add('image-error')"
    />

    <h5>{{ user.getUsername(props.data.username) }}</h5>
    <div class="flex-1"></div>

    <div v-if="props.data.country" :data-title-top="countries[props.data.country].name">
      <img class="flag" :src="flag(props.data.country)" :alt="props.data.country" />
    </div>
  </router-link>
</template>
