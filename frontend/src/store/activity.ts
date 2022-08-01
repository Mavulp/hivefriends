import { defineStore } from "pinia"
import { get } from "../js/fetch"
import { useLoading } from "./loading"
import { useToast } from "./toast"
import { FetchError } from "../js/global-types"

export type ActivityItem = {
  id: string | number
}

interface State {
  items: ActivityItem[]
  open: boolean
}

export const useActivity = defineStore("activity", {
  state: () =>
    ({
      items: [],
      open: false
    } as State),
  actions: {
    async fetchActivity() {}
  },
  getters: {
    // hasNew: (state: State) => state.items.filter((alert) => !alert.read).length
    hasNew: (state: State) => {
      // Check length of activity log

      //

      return false
    }
  }
})
