<script setup lang="ts">
import { computed, reactive, ref, watch } from 'vue'
import type { Comment } from '../../store/comments'
import { useComments } from '../../store/comments'
import { useLoading } from '../../store/loading'
import { useUser } from '../../store/user'
import { minLength, required, useFormValidation } from '../../js/validation'
import { sanitize } from '../../js/utils'
import { formatTextUsernames } from '../../js/_composables'

import LoadingSpin from '../loading/LoadingSpin.vue'
import InputTextarea from '../form/InputTextarea.vue'
import { useAlbums } from '../../store/album'
import CommentVue from './Comment.vue'
import AliasModal from './AliasModal.vue'

const props = defineProps<Props>()
const emit = defineEmits<{
  (e: 'close'): void
}>()
const comments = useComments()
const user = useUser()
const album = useAlbums()
const { getLoading } = useLoading()

interface Props {
  albumKey: string
  imageKey: string
  uploader: string
}

const data = computed<Comment[]>(() => comments.comments[`comments-${props.albumKey}-${props.imageKey}`] ?? [])
const modal = ref(false)

watch(
  () => props.imageKey,
  async () => {
    await comments.fetchComments(
      {
        albumKey: props.albumKey,
        imageKey: props.imageKey,
      },
      user.public_token,
    )
  },
  { immediate: true },
)

/**
 * Form Handling
 */

const form = reactive({
  comment: '',
})

const rules = computed(() => ({
  comment: {
    required,
    minLength: minLength(1),
  },
}))

const { validate, errors, reset } = useFormValidation(form, rules, { autoclear: true })

async function submit() {
  modal.value = false

  validate().then(() => {
    comments.addComment({
      albumKey: props.albumKey,
      imageKey: props.imageKey,
      text: form.comment,
    })

    setTimeout(() => {
      form.comment = ''
      reset()
    }, 25)
  })
}

function insert(text: string) {
  let t = `!${text}`
  if (form.comment)
    t = ` ${t}`

  form.comment += t
}

/**
 * Detect if user pressed enter while writing
 */

let keys: Array<string> = []

function handleKeys(e: any) {
  e.stopPropagation()

  keys.push(e.key)

  if (e.key === 'Enter' && !keys.includes('Shift')) {
    keys = []
    submit()
  }

  if (keys.length > 5)
    keys = []
}

const metadata = computed(() => {
  if (!user.public_token)
    return album.getImageMetadata(props.imageKey)

  return undefined
})
</script>

<template>
  <div class="hi-comments">
    <div v-if="metadata?.fileName || metadata?.description" class="hi-comments-info">
      <div class="icon">
        <span class="material-icons">&#xe412;</span>
      </div>

      <strong v-if="metadata?.fileName" class="file-name">{{ metadata.fileName }}</strong>
      <p v-if="metadata?.description" v-html="sanitize(formatTextUsernames(metadata.description, user))" />
    </div>
    <div class="hi-comments-header">
      <h6>Comments</h6>
      <span>({{ data.length }})</span>

      <div class="flex-1" />

      <button class="control-button" @click="emit('close')">
        Hide
        <span class="material-icons">&#xe5dc;</span>
      </button>
    </div>

    <div class="hi-comments-list">
      <LoadingSpin v-if="getLoading(`comments-${props.albumKey}-${props.imageKey}`)" class="dark center-parent" />
      <div v-else-if="data.length === 0" class="hi-no-comments">
        <span>...</span>
      </div>
      <template v-else>
        <CommentVue
          v-for="item in data"
          :key="item.id"
          :data="item"
          :image-key="props.imageKey"
          :uploader="props.uploader"
        />
      </template>
    </div>

    <div v-if="!user.public_token" class="hi-add-comment">
      <form @submit.prevent="submit">
        <InputTextarea
          v-model:value="form.comment"
          placeholder="Write comment..."
          :error="errors.comment"
          @keydown="handleKeys"
        />

        <div class="buttons">
          <button type="submit" class="hover-bubble">
            Send
          </button>
          <LoadingSpin v-if="getLoading(`add-comments-${props.albumKey}-${props.imageKey}`)" class="dark small" />

          <div class="flex-1" />

          <button class="hover-bubble" :class="{ active: modal }" @click.prevent="modal = !modal">
            <span class="material-icons"> &#xe1d3; </span>
          </button>
        </div>
      </form>
    </div>
  </div>
  <AliasModal v-if="modal" @close="modal = false" @insert="insert" />
</template>
