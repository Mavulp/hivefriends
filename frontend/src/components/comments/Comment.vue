<script setup lang="ts">
import { computed } from 'vue'
import dayjs from 'dayjs'
import type { Comment } from '../../store/comments'
import { useComments } from '../../store/comments'
import { imageUrl } from '../../store/album'
import { useUser } from '../../store/user'
import { sanitize } from '../../js/utils'
import { formatTextImages, formatTextUsernames } from '../../js/_composables'
import { timeDateFormat } from '../../js/time'

const props = defineProps<Props>()
const user = useUser()
const comments = useComments()

interface Props {
  data: Comment
  imageKey: string
  uploader: string
}

const text = computed(() => {
  if (!props.data.text)
    return ''

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
            backgroundColor: `rgb(${user.getUser(props.data.author, 'accentColor')})`,
          }"
          alt=" "
          @error="(e: any) => e.target.classList.add('image-error')"
        >

        <span>{{ user.getUsername(props.data.author) }}</span>
      </router-link>

      <div v-if="props.uploader === props.data.author" class="tag tag-orange">
        Author
      </div>
    </div>

    <div class="comment-body">
      <p v-html="sanitize(text)" />
      <span class="timestamp">{{ dayjs(props.data.createdAt * 1000).format(timeDateFormat) }}</span>
    </div>
  </div>
</template>
