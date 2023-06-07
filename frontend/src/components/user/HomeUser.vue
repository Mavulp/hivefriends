<script setup lang="ts">
import dayjs from 'dayjs'
import type { User } from '../../store/user'
import { useUser } from '../../store/user'
import { flag } from '../../js/utils'
import { imageUrl } from '../../store/album'
import countries from '../../js/countries'
import { normalDateFormat } from '../../js/time'

const props = defineProps<Props>()
const user = useUser()

interface Props {
  data: User
}
</script>

<template>
  <router-link class="home-user" :to="{ name: 'UserProfile', params: { user: props.data.username } }">
    <img
      class="user-image"
      :src="imageUrl(props.data.avatarKey, 'tiny')"
      alt=" "
      :style="{ backgroundColor: `rgb(${props.data.accentColor})` }"
      @error="(e: any) => e.target.classList.add('image-error')"
    >

    <div class="info">
      <strong>{{ user.getUsername(props.data.username) }}</strong>
      <p>Joined: {{ dayjs(props.data.createdAt * 1000).format(normalDateFormat) }}</p>
    </div>

    <div v-if="props.data.country" :data-title-top="countries[props.data.country].name">
      <img class="flag" :src="flag(props.data.country)" :alt="props.data.country">
    </div>
  </router-link>
</template>
