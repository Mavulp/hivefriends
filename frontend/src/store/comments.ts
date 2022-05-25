import { defineStore } from "pinia"
import { del, get, post } from "../js/fetch"
import { FetchError } from "../js/global-types"
import { useLoading } from "./loading"
import { useToast } from "./toast"

export type Comment = {
  id: number
  author: string
  imageKey: string
  createdAt: number
  text: string
}

interface State {
  comments: Array<Comment>
}

export const useComments = defineStore("comments", {
  state: () =>
    ({
      comments: []
    } as State),
  actions: {
    async fetchComments({ albumKey, imageKey }: { albumKey: string; imageKey: string }) {
      const { addLoading, delLoading } = useLoading()

      addLoading("comments")

      return get(`/api/comments/${albumKey}/${imageKey}/`)
        .then((response) => {
          this.comments = response
          return response
        })
        .catch((error: FetchError) => {
          const toast = useToast()
          toast.add(error.message, "error")

          return []
        })
        .finally(() => {
          delLoading("comments")
        })
    },

    async addComment({ albumKey, imageKey, text }: { albumKey: string; imageKey: string; text: string }) {
      const { addLoading, delLoading } = useLoading()

      addLoading("add-comment")

      return post(`/api/comments/${albumKey}/${imageKey}/`, text)
        .then((response) => {
          this.comments.push(response)
          return response
        })
        .catch((error: FetchError) => {
          const toast = useToast()
          toast.add(error.message, "error")
        })
        .finally(() => {
          delLoading("add-comment")
        })
    },

    async delComment(id: number) {
      const { add } = useToast()

      return del(`/api/comments/${id}`)
        .then(() => {
          this.comments = this.comments.filter((item: Comment) => item.id !== id)
          add("Successfully deleted comment", "success")
        })
        .catch(() => add("Error deleting comment", "error"))
    }
  }
})
