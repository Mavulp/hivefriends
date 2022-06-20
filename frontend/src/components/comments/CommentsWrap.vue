<script setup lang="ts">
import { watch, computed, reactive, nextTick, ref } from "vue"
import { Comment, useComments } from "../../store/comments"
import { useLoading } from "../../store/loading"
import { useUser } from "../../store/user"
import { minLength, required, useFormValidation } from "../../js/validation"

import LoadingSpin from "../loading/LoadingSpin.vue"
import InputTextarea from "../form/InputTextarea.vue"
import CommentVue from "./Comment.vue"
import AliasModal from "./AliasModal.vue"
import Modal from "../Modal.vue"

const comments = useComments()
const user = useUser()
const { getLoading } = useLoading()

interface Props {
  albumKey: string
  imageKey: string
  uploader: string
}

const props = defineProps<Props>()

const emit = defineEmits<{
  (e: "close"): void
}>()

const data = computed<Array<Comment>>(() => comments.comments)
const modal = ref(false)

watch(
  () => props.imageKey,
  async () => {
    await comments.fetchComments(
      {
        albumKey: props.albumKey,
        imageKey: props.imageKey
      },
      user.public_token
    )
  },
  { immediate: true }
)

/**
 * Form Handling
 */

const form = reactive({
  comment: ""
})

const rules = computed(() => ({
  comment: {
    required,
    minLength: minLength(1)
  }
}))

const { validate, errors, reset } = useFormValidation(form, rules, { autoclear: true })

async function submit() {
  modal.value = false

  validate().then(() => {
    comments.addComment({
      albumKey: props.albumKey,
      imageKey: props.imageKey,
      text: form.comment
    })

    setTimeout(() => {
      form.comment = ""
      reset()
    }, 25)
  })
}

function insert(text: string) {
  let t = `!${text}`
  if (form.comment) t = " " + t

  form.comment += t
}

/**
 * Detect if user pressed enter while writing
 */

let keys: Array<string> = []

function handleKeys(e: any) {
  keys.push(e.key)

  if (e.key === "Enter" && !keys.includes("Shift")) {
    keys = []
    submit()
  }

  if (keys.length > 5) keys = []
}
</script>

<template>
  <div class="hi-comments">
    <div class="hi-comments-header">
      <h5>Comments</h5>
      <span>({{ data.length }})</span>

      <div class="flex-1"></div>

      <button class="control-button" @click="emit('close')">
        <span class="material-icons">&#xe5cd;</span>
      </button>
    </div>

    <div class="hi-comments-list">
      <LoadingSpin class="dark center-parent" v-if="getLoading('comments')" />
      <div class="hi-no-comments" v-else-if="data.length === 0">
        <span>Nobody said anything</span>
        <strong>...yet</strong>
      </div>
      <template v-else>
        <CommentVue
          v-for="item in data"
          :key="item.id"
          :data="item"
          :imageKey="props.imageKey"
          :uploader="props.uploader"
        />
      </template>
    </div>

    <div class="hi-add-comment" v-if="!user.public_token">
      <form @submit.prevent="submit">
        <InputTextarea
          v-model:value="form.comment"
          placeholder="Write comment..."
          :error="errors.comment"
          @keydown="handleKeys"
        />

        <div class="buttons">
          <button type="submit" class="hover-bubble">Send</button>
          <LoadingSpin class="dark small" v-if="getLoading('add-comments')" />

          <div class="flex-1"></div>

          <button @click.prevent="modal = !modal" class="hover-bubble" :class="{ active: modal }">
            <span class="material-icons"> &#xe1d3; </span>
          </button>
        </div>
      </form>
    </div>
  </div>
  <AliasModal v-if="modal" @close="modal = false" @insert="insert" />
</template>
