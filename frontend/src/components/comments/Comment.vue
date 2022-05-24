<script setup lang="ts">
import { ref } from "vue"
import { Comment, useComments } from "../../store/comments"
import { imageUrl } from "../../store/album"
import { useUser } from "../../store/user"
import { formatDate } from "../../js/utils"

const user = useUser()
const comments = useComments()

interface Props {
  data: Comment
  imageKey: string
  uploader: string
}

const props = defineProps<Props>()

function formatTimestamp(date: number) {
  date *= 1000
  const d = new Date(date)

  return `${d.getUTCHours()}:${d.getUTCMinutes()}, ${d.toLocaleDateString("en-GB", {
    year: "numeric",
    month: "short",
    day: "numeric"
  })}`
}
</script>

<template>
  <div class="hi-comment">
    <div class="comment-header">
      <button
        v-if="props.data.author === user.user.username || props.uploader === user.user.username"
        class="control-button hover-bubble"
        data-title-top="Remove"
        @click="comments.delComment(props.imageKey, data.id)"
      >
        <span class="material-icons"> &#xe872; </span>
      </button>

      <router-link class="hover-bubble" :to="{ name: 'UserProfile', params: { user: props.data.author } }">
        <img
          class="user-image"
          :src="imageUrl(user.getUser(props.data.author, 'avatarKey'), 'tiny')"
          :style="{
            backgroundColor: `rgb(${user.getUser(props.data.author, 'accentColor')})`
          }"
          alt=" "
          @error="(e: any) => e.target.classList.add('image-error')"
        />

        <span>{{ user.getUsername(props.data.author) }}</span>
      </router-link>

      <div class="tag tag-blue" v-if="props.uploader === props.data.author">Author</div>
    </div>

    <div class="comment-body">
      <p>{{ props.data.text }}</p>
      <span class="timestamp">{{ formatTimestamp(props.data.createdAt) }}</span>
    </div>
  </div>
</template>
