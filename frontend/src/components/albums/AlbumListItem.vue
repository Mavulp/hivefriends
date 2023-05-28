<script setup lang="ts">
import { useRoute, useRouter } from 'vue-router'
import type { Album } from '../../store/album'
import { imageUrl } from '../../store/album'
import { useUser } from '../../store/user'

const props = defineProps<Props>()
const user = useUser()
const router = useRouter()
const route = useRoute()

interface Props {
  data: Album
}

function goto() {
  router.push({ name: 'AlbumDetail', params: { id: props.data.key } })
}
</script>

<template>
  <div class="album-list-item" @click.self="goto">
    <div class="album-image" @click="goto">
      <img :src="imageUrl(props.data.coverKey, 'medium')" alt=" ">
    </div>

    <div class="album-meta">
      <AlbumTimestamp class="dark-light" :timeframe="props.data.timeframe" @click="goto" />
      <div class="draft-wrap" @click="goto">
        <span v-if="props.data.draft" class="album-draft">Draft</span>
      </div>

      <h2 @click="goto">
        {{ props.data.title }}
      </h2>
      <p v-if="props.data.description" @click="goto">
        {{ props.data.description }}
      </p>
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
          >

          <span>{{ user.getUsername(props.data.author) }}</span>
        </router-link>
      </div>
    </div>
  </div>
</template>
