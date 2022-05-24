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
  comments: {
    [key: string]: Array<Comment>
  }
}

export const useComments = defineStore("comments", {
  state: () =>
    ({
      comments: {}
    } as State),
  actions: {
    async fetchComments(key: string) {
      const { addLoading, delLoading } = useLoading()

      addLoading("comments")

      return get(`/api/images/${key}/comments/`)
        .then((response) => {
          this.comments[key] = response
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

    async addComment(key: string, text: string) {
      const { addLoading, delLoading } = useLoading()

      addLoading("add-comment")

      return post(`/api/images/${key}/comments/`, text)
        .then((response) => {
          this.comments[key].push(response)
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

    async delComment(key: string, id: number) {
      const { add } = useToast()

      return del(`/api/images/${key}/comments/${id}`)
        .then(() => add("Successfully deleted comment", "success"))
        .catch(() => add("Error deleting comment", "error"))
    }
  },
  getters: {
    getImageComments: (state) => (key: string) => {
      if (!state.comments[key] || state.comments[key].length === 0) return []
      return state.comments[key]
    }
  }
})
