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
  validate().then(() => {
    comments.addComment({
      albumKey: props.albumKey,
      imageKey: props.imageKey,
      text: form.comment
    })

    nextTick(() => {
      form.comment = ""
      reset()
    })
  })
}

function insert(text: string) {
  let t = `!${text}`
  if (form.comment) t = " " + t

  form.comment += t
}
</script>

<template>
  <div class="hi-comments">
    <div class="hi-comments-header">
      <h5>Comments</h5>

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
        <InputTextarea v-model:value="form.comment" placeholder="Write comment..." :error="errors.comment" />

        <div class="buttons">
          <button type="submit" class="hover-bubble">Send</button>
          <LoadingSpin class="dark small" v-if="getLoading('add-comments')" />

          <div class="flex-1"></div>

          <button @click.prevent="modal = true" class="hover-bubble">
            <span class="material-icons"> &#xe1d3; </span>
          </button>

          <Teleport to="body" v-if="modal">
            <Modal @close="modal = false">
              <AliasModal @close="modal = false" @insert="insert" />
            </Modal>
          </Teleport>
        </div>
      </form>
    </div>
  </div>
</template>
