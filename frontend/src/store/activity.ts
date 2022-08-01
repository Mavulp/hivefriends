import { defineStore } from "pinia"
import { get, post } from "../js/fetch"
import { useLoading } from "./loading"
import { useToast } from "./toast"
import { FetchError } from "../js/global-types"

export type ActivityType = "comment" | "album" | "user"

export type ActivityItem = {
  type: ActivityType
  timestamp: number
  user: string
  location: { [key: string]: any }
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
          this.items = response.items
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
  }
})
