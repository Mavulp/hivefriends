<script setup lang="ts">
import { watch, computed, reactive, nextTick } from "vue"
import { Comment, useComments } from "../../store/comments"
import { useLoading } from "../../store/loading"
import { useUser } from "../../store/user"
import { minLength, required, useFormValidation } from "../../js/validation"

import LoadingSpin from "../loading/LoadingSpin.vue"
import InputTextarea from "../form/InputTextarea.vue"
import CommentVue from "./Comment.vue"

const comments = useComments()
const user = useUser()
const { getLoading } = useLoading()

interface Props {
  imageKey: string
  uploader: string
}

const props = defineProps<Props>()

const emit = defineEmits<{
  (e: "close"): void
}>()

const data = computed<Array<Comment>>(() => comments.getImageComments(props.imageKey))

watch(
  () => props.imageKey,
  async (value) => {
    await comments.fetchComments(value)
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
    comments.addComment(props.imageKey, form.comment)

    nextTick(() => {
      form.comment = ""
      reset()
    })
  })
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

    <div class="hi-add-comment">
      <form @submit.prevent="submit">
        <InputTextarea v-model:value="form.comment" placeholder="Write comment..." :error="errors.comment" />

        <div class="buttons">
          <button type="submit" class="hover-bubble">Save</button>
          <LoadingSpin class="dark small" v-if="getLoading('add-comments')" />
        </div>
      </form>
    </div>
  </div>
</template>
