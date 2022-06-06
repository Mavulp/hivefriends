<script setup lang="ts">
import { computed, ref } from "vue"
import { Comment, useComments } from "../../store/comments"
import { imageUrl } from "../../store/album"
import { useUser } from "../../store/user"
import { sanitize } from "../../js/utils"
import { formatTextUsernames } from "../../js/_composables"
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

function formatTextImages(text: string) {
  const _regex = /\bhttps?:\/\/\S+/gi
  const formats = [".jpeg", ".gif", ".png", ".apng", ".svg", ".bmp", ".bmp", ".ico", ".jpg", ".webp"]

  const urls = [...new Set(props.data.text.match(_regex))]

  const _img = (_url: string) => `<img src="${_url}" />`
  const _a = (_url: string) => `<a href="${_url}" target="_blank">${_url}</a>`

  if (urls && urls.length > 0) {
    // Loop over each url
    urls.map((url) => {
      let chunk

      if (formats.some((format) => url.endsWith(format))) {
        // Is an image
        chunk = _img(url)
      } else {
        // is a link
        chunk = _a(url)
      }

      text = text.replaceAll(url, chunk)
    })
  }

  return text
}

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
