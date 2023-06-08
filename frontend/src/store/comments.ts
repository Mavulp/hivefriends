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
  comments: Record<string, Comment[]>
}

export const useComments = defineStore('comments', {
  state: () => ({
    comments: {},
  } as State),
  actions: {
    async fetchComments({ albumKey, imageKey }: { albumKey: string; imageKey: string }, token?: string | string[]) {
      const { addLoading, delLoading } = useLoading()

      const commentListId = `comments-${albumKey}-${imageKey}`

      addLoading(commentListId)

      const query = token
        ? `/api/public/comments/${albumKey}/${imageKey}/${token}`
        : `/api/comments/${albumKey}/${imageKey}`

      return get(query)
        .then((response) => {
          this.comments[commentListId] = response
          return response
        })
        .catch((error: FetchError) => {
          const toast = useToast()
          toast.add(error.message, 'error')

          return []
        })
        .finally(() => {
          delLoading(commentListId)
        })
    },

    async addComment({ albumKey, imageKey, text }: { albumKey: string; imageKey: string; text: string }) {
      const { addLoading, delLoading } = useLoading()

      const addCommentId = `add-comment-${albumKey}-${imageKey}`

      addLoading(addCommentId)

      return post(`/api/comments/${albumKey}/${imageKey}`, text)
        .then((response) => {
          if (!this.comments[addCommentId])
            this.comments[addCommentId] = [response]
          else
            this.comments[addCommentId].push(response)
          return response
        })
        .catch((error: FetchError) => {
          const toast = useToast()
          toast.add(error.message, 'error')
        })
        .finally(() => {
          delLoading(addCommentId)
        })
    },

    async delComment(id: number) {
      const { add } = useToast()

      return del(`/api/comments/${id}`)
        .then(() => {
          for (const [key, comments] of Object.entries(this.comments)) {
            if (comments.find(c => c.id === id)) {
              this.comments[key] = this.comments[key].filter(c => c.id === id)
              break
            }
          }

          // this.comments = this.comments.filter((item: Comment) => item.id !== id)
          add('Successfully deleted comment', 'success')
        })
        .catch(() => add('Error deleting comment', 'error'))
    },
  },
})
