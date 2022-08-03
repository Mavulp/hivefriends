<script setup lang="ts">
import { computed, ref } from "vue"
import { User, useUser } from "../../../store/user.js"
import { imageUrl } from "../../../store/album.js"
import { getRanMinMax, flag, RGB_TO_HEX } from "../../../js/utils.js"
import countries from "../../../js/countries.js"
import { useRouter } from "vue-router"

const router = useRouter()
const user = useUser()
const props = defineProps<{ data: User }>()

const data = computed(() => props.data)

const messages = ["Welcome!", "Say Hello!", "New person!", "Someone just joined!!"]
const message = ref(messages[getRanMinMax(0, 3)])

function go() {
  router.push({
    name: "UserProfile",
    params: {
      user: data.value.username
    }
  })
}
</script>

<template>
  <div class="activity-item activity-user" @click="go">
    <div class="title">
      <strong>{{ message }} 🎉 🎉</strong>
    </div>

    <div class="user">
      <img
        class="user-image"
        :src="imageUrl(data.avatarKey, 'medium')"
        alt=" "
        @error="(e: any) => e.target.classList.add('image-error')"
      />

      <div class="info">
        <span>{{ user.getUsername(data.username) }}</span>

        <div class="nationality-wrap" v-if="data.country">
          <img class="flag" :src="flag(data.country, 'png')" alt="" />
          <p>{{ countries[data.country].name }}</p>
        </div>
      </div>
    </div>

    <div class="background" :style="{ backgroundColor: RGB_TO_HEX(data.accentColor) }"></div>
  </div>
</template>
