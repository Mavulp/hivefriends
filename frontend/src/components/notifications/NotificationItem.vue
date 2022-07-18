<script setup lang="ts">
import { computed } from "vue"
import { Alert } from "../../store/notification"
import { useUser } from "../../store/user"
import { imageUrl } from "../../store/album"
import { formatTimestamp } from "../../js/utils"
import { useRouter } from "vue-router"

const router = useRouter()
const user = useUser()
const props = defineProps<{
  data: Alert
}>()

const tagline = computed(() => {
  //prettier-ignore
  switch(props.data.type) {
    case 'comment-mention': return 'has mentioned you in a comment.'
    case 'album-tag': return `has tagged you in an album '${props.data.title}'`
    case 'user-new': return 'has joined hi!friends!'
    case 'album-comment': return `has commented on your album '${props.data.title}'`
  }
})

const tagged = computed(() => user.getUser(props.data.user))

/**
 * Route to place
 */

function go() {
  if (props.data.url) {
    router.push({ path: props.data.url })
  }
}
</script>

<template>
  <div class="notification-item" @click.self="go()" v-if="tagged">
    <div class="title-wrap">
      <img
        class="user-image"
        :src="imageUrl(tagged.avatarKey, 'tiny')"
        alt=" "
        @error="(e: any) => e.target.classList.add('image-error')"
      />

      <p>
        <router-link
          :style="{ color: tagged.accentColor }"
          :to="{ name: 'UserProfile', params: { user: tagged.username } }"
        >
          {{ user.getUsername(tagged.username) }}
        </router-link>
        <span>
          {{ tagline }}
        </span>
      </p>
    </div>
    <div class="notification-body" v-if="props.data.text">
      <p>{{ props.data.text }}</p>
    </div>

    <div class="notification-footer">
      <p>
        {{ formatTimestamp(props.data.createdAt) }}
      </p>
    </div>

    <button class="mark-read">
      <span class="material-icons" data-title-left="Mark as read">&#xe5c9;</span>
    </button>

    <div class="background" :style="[`backgroundColor: rgb(${tagged.accentColor})`]"></div>
  </div>
</template>
