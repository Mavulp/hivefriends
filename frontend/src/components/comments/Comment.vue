<script setup lang="ts">
import { computed, ref } from "vue"
import { Comment, useComments } from "../../store/comments"
import { imageUrl } from "../../store/album"
import { useUser } from "../../store/user"
import { sanitize, formatTimestamp } from "../../js/utils"
import { formatTextUsernames, formatTextImages } from "../../js/_composables"
const user = useUser()
const comments = useComments()

interface Props {
  data: Comment
  imageKey: string
  uploader: string
}

const props = defineProps<Props>()

const text = computed(() => {
  if (!props.data.text) return ""

  let text = props.data.text

  text = formatTextImages(text)
  text = formatTextUsernames(text, user)

  return text
})
</script>

<template>
  <div class="hi-comment">
    <div class="comment-header">
      <button
        v-if="props.data.author === user.user.username || props.uploader === user.user.username"
        class="control-button hover-bubble"
        data-title-top="Remove"
        @click="comments.delComment(data.id)"
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

      <div class="tag tag-orange" v-if="props.uploader === props.data.author">Author</div>
    </div>

    <div class="comment-body">
      <p v-html="sanitize(text)"></p>
      <span class="timestamp">{{ formatTimestamp(props.data.createdAt) }}</span>
    </div>
  </div>
</template>
