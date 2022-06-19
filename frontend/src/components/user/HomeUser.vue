<script setup lang="ts">
import { User, useUser } from "../../store/user"
import { fetchFlag } from "../../js/utils"
import { imageUrl } from "../../store/album"
import countries from "../../js/countries"
import { onBeforeMount, ref } from "vue"

const user = useUser()

interface Props {
  data: User
}

const props = defineProps<Props>()
const flag = ref()

onBeforeMount(async () => {
  if (props.data.country) {
    flag.value = await fetchFlag(props.data.country)
  }
})
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

    <div v-if="props.data.country" :data-title-top="countries[props.data.country].name">
      <div class="flag" v-html="flag"></div>
    </div>
    <div class="flex-1"></div>

    <button class="hover-bubble">View Profile</button>
  </router-link>
</template>
