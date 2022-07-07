import { defineStore } from "pinia"

export type AlertTypes = "comment-mention" | "album-tag" | "user-new" | "album-comment" | "album-update"

export type Alert = {
  id: string | number
  title: string
  text: string
  url: string
  read: boolean
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
    title: "New mention",
    text: "@tmtu mentioned you in a comment on @jokler's post",
    url: "/album/X47n8YsPT-q-NyO6-1Ap0A/image/JTk3wz1HQVe2zZNNbwPpOA",
    read: false,
    createdAt: Date.now() - 60 * 60 * 24,
    type: "comment-mention"
  },
  {
    id: 1,
    title: "Tagged in an album",
    text: "@jokler tagged you in an album 'How we went to hospital'",
    url: "/album/9PSaVN92T5mU39Ze06fOdw",
    read: false,
    createdAt: Date.now() - 60 * 60 * 24 * 2,
    type: "album-tag"
  },
  {
    id: 2,
    title: "@northcode joined!",
    text: "Check out @northcode's profile",
    url: "/user/northcode",
    read: false,
    createdAt: Date.now() - 60 * 60 * 24 * 4,
    type: "user-new"
  },
  {
    id: 3,
    title: "New comments in 'Amom'",
    text: "You have 3 new comments in your album 'Amom'",
    url: "/album/X47n8YsPT-q-NyO6-1Ap0A",
    read: false,
    createdAt: Date.now() - 60 * 60 * 24 * 4 - 2500,
    type: "album-comment"
  },
  {
    id: 3,
    title: "'Amom' has been updated",
    text: "The album 'Amom' has 17 new images, check it out",
    url: "/album/X47n8YsPT-q-NyO6-1Ap0A",
    read: false,
    createdAt: Date.now() - 60 * 60 * 24 * 4 - 2500,
    type: "album-update"
  }
]

export const icons: { [key: AlertTypes]: string } = {}

export const useNotifications = defineStore("notifications", {
  state: () =>
    ({
      items: exampleAlerts,
      open: false
    } as State),
  actions: {},
  getters: {
    unreadCount: (state: State) => state.items.filter((alert) => !alert.read).length
  }
})
