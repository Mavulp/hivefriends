import { defineStore } from 'pinia'
import { del, get, post } from '../js/fetch'
import type { FetchError } from '../js/global-types'
import { useLoading } from './loading'
import { useToast } from './toast'

export interface Comment {
  id: number
  author: string
  imageKey: string
  albumKey: string
  createdAt: number
  text: string
}

interface State {
  comments: Array<Comment>
}

export const useComments = defineStore('comments', {
  state: () => ({
    comments: [],
  } as State),
  actions: {
    async fetchComments({ albumKey, imageKey }: { albumKey: string; imageKey: string }, token?: string | string[]) {
      const { addLoading, delLoading } = useLoading()

      addLoading('comments')

      const query = token
        ? `/api/public/comments/${albumKey}/${imageKey}/${token}`
        : `/api/comments/${albumKey}/${imageKey}`

      return get(query)
        .then((response) => {
          // TODO: group activity by day?
          this.comments = response
          return response
        })
        .catch((error: FetchError) => {
          const toast = useToast()
          toast.add(error.message, 'error')

          return []
        })
        .finally(() => {
          delLoading('comments')
        })
    },

    async addComment({ albumKey, imageKey, text }: { albumKey: string; imageKey: string; text: string }) {
      const { addLoading, delLoading } = useLoading()

      addLoading('add-comment')

      return post(`/api/comments/${albumKey}/${imageKey}`, text)
        .then((response) => {
          this.comments.push(response)
          return response
        })
        .catch((error: FetchError) => {
          const toast = useToast()
          toast.add(error.message, 'error')
        })
        .finally(() => {
          delLoading('add-comment')
        })
    },

    async delComment(id: number) {
      const { add } = useToast()

      return del(`/api/comments/${id}`)
        .then(() => {
          this.comments = this.comments.filter((item: Comment) => item.id !== id)
          add('Successfully deleted comment', 'success')
        })
        .catch(() => add('Error deleting comment', 'error'))
    },
  },
})
