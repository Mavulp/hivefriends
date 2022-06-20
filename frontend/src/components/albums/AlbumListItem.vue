<script setup lang="ts">
import { imageUrl, Album } from "../../store/album"
import AlbumTimestamp from "../../components/albums/AlbumTimestamp.vue"
import { useUser } from "../../store/user"
import { useRoute, useRouter } from "vue-router"

const user = useUser()
const router = useRouter()
const route = useRoute()

interface Props {
  data: Album
}

const props = defineProps<Props>()

function goto() {
  router.push({ name: "AlbumDetail", params: { id: props.data.key } })
}
</script>

<template>
  <div class="album-list-item" @click.self="goto">
    <div class="album-image" @click="goto">
      <img :src="imageUrl(props.data.coverKey, 'medium')" alt=" " />
    </div>

    <div class="album-meta">
      <AlbumTimestamp @click="goto" class="dark-light" :timeframe="props.data.timeframe" />
      <div @click="goto" class="draft-wrap">
        <span class="album-draft" v-if="props.data.draft">Draft</span>
      </div>

      <h2 @click="goto">{{ props.data.title }}</h2>
      <p @click="goto">{{ props.data.description }}</p>
      <div
        v-if="(props.data.author !== user.user.username && route.name !== 'UserProfile') || route.name === 'Albums'"
        class="draft-wrap"
      >
        <router-link
          :to="{ name: 'UserProfile', params: { user: props.data.author } }"
          class="album-author hover-bubble"
        >
          <img
            class="user-image"
            :src="imageUrl(user.getUser(props.data.author, 'avatarKey'), 'tiny')"
            :style="[`backgroundColor: rgb(${user.getUser(props.data.author, 'accentColor')})`]"
            alt=" "
            @error="(e: any) => e.target.classList.add('image-error')"
          />

          <span>{{ user.getUsername(props.data.author) }}</span>
        </router-link>
      </div>
    </div>
  </div>
</template>
