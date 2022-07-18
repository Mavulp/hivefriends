import { defineStore } from "pinia"
import { get } from "../js/fetch"
import { useLoading } from "./loading"
import { useToast } from "./toast"
import { FetchError } from "../js/global-types"

export type AlertTypes = "comment-mention" | "album-tag" | "user-new" | "album-comment" | "album-update"

export type Alert = {
  id: string | number
  title?: string
  text?: string
  url: string
  read: boolean
  user: string
  createdAt: number
  type: AlertTypes
}

interface State {
  items: Alert[]
  open: boolean
}

const exampleAlerts: Alert[] = [
  {
    id: 0,
    text: "'Have you seen this little fella?'",
    user: "tmtu",
    url: "/album/X47n8YsPT-q-NyO6-1Ap0A/image/JTk3wz1HQVe2zZNNbwPpOA",
    read: false,
    createdAt: (Date.now() - 60 * 60 * 24) / 1000,
    type: "comment-mention"
  },
  {
    id: 1,
    user: "Jokler",
    text: "How we went to hospital",
    url: "/album/9PSaVN92T5mU39Ze06fOdw",
    read: false,
    createdAt: (Date.now() - 60 * 60 * 24 * 3) / 1000,
    type: "album-tag"
  },
  {
    id: 2,
    user: "jlazar",
    text: "Check out their profile!",
    url: "/user/jlazar",
    read: false,
    createdAt: (Date.now() - 60 * 60 * 24 * 4) / 1000,
    type: "user-new"
  },
  {
    id: 3,
    title: "Amon",
    user: "apiratehat",
    text: "'What an ugly dog' & 'what a little shit'",
    url: "/album/X47n8YsPT-q-NyO6-1Ap0A",
    read: false,
    createdAt: Date.now() - 60 * 60 * 24 * 4 - 2500,
    type: "album-comment"
  }
  // {
  //   id: 3,
  //   title: "'Amom' has been updated",
  //   text: "The album 'Amom' has 17 new images, check it out",
  //   url: "/album/X47n8YsPT-q-NyO6-1Ap0A",
  //   read: false,
  //   createdAt: Date.now() - 60 * 60 * 24 * 4 - 2500,
  //   type: "album-update"
  // }
]

export const icons: { [key: string]: string } = {}

export const useNotifications = defineStore("notifications", {
  state: () =>
    ({
      items: exampleAlerts,
      open: false
    } as State),
  actions: {
    async fetchNotification() {
      const { addLoading, delLoading } = useLoading()

      addLoading("notifications")
      return get("")
        .then((response) => {
          this.items = response
          return response
        })
        .catch((error: FetchError) => {
          const toast = useToast()
          toast.add(error.message, "error")
        })
        .finally(() => {
          delLoading("notifications")
        })
    }
  },
  getters: {
    unreadCount: (state: State) => state.items.filter((alert) => !alert.read).length
  }
})
