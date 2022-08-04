import { defineStore } from "pinia"
import { get, post } from "../js/fetch"
import { useLoading } from "./loading"
import { useToast } from "./toast"
import { FetchError } from "../js/global-types"
import { Comment } from "./comments"
import { Album, Image } from "./album"
import { User } from "./user"

export type ActivityItem =
  | {
      comment: Comment
    }
  | {
      image: Image
    }
  | {
      album: Album
    }
  | {
      user: User
    }

interface State {
  items: ActivityItem[]
  hasNew: boolean
  open: boolean
}

export const useActivity = defineStore("activity", {
  state: () =>
    ({
      items: [],
      hasNew: false,
      open: false
    } as State),
  actions: {
    async fetchActivity() {
      const { addLoading, delLoading } = useLoading()

      addLoading("activity")

      return get("/api/activity")
        .then((response) => {
          response = response.slice(0, 200)

          // TODO: group activity by days
          this.items = response
          this.hasNew = true
          return response
        })
        .catch((error: FetchError) => {
          const toast = useToast()
          toast.add(error.message, "error")
          return []
        })
        .finally(() => {
          delLoading("activity")
        })
    },
    async updateView() {
      return post("/api/activity", "")
    }
  },
  getters: {
    getActivity: (state: State) => state.items
  }
})
