import { defineStore } from "pinia"
import { get, post } from "../js/fetch"
import { useLoading } from "./loading"
import { useToast } from "./toast"
import { FetchError } from "../js/global-types"
import { Comment } from "./comments"
import { Album, Image } from "./album"
import { User } from "./user"
import { padTo2Digits } from "../js/utils"

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
  sortedItems: { [key: string]: ActivityItem[] }
  hasNew: boolean
  open: boolean
}

export const useActivity = defineStore("activity", {
  state: () =>
    ({
      items: [],
      sortedItems: {},
      hasNew: false,
      open: false
    } as State),
  actions: {
    async fetchActivity() {
      const { addLoading, delLoading } = useLoading()

      addLoading("activity")

      return get("/api/activity")
        .then((response) => {
          this.items = response.slice(0, 128)
          this.hasNew = true

          // Group activity by day
          const grouped: { [key: string]: ActivityItem[] } = {}

          this.items.map((item: ActivityItem) => {
            // Extract timestamp item from different data sources
            const timestamp = _extractTimestamp(item) * 1000
            const date = _formatDate(new Date(timestamp))

            // If date is not yet present, assign an empty array to it
            if (!grouped[date]) grouped[date] = []

            // Push item to the date's array
            grouped[date].push(item)
          })

          // Sort object keys to be descending

          const sorted = Object.keys(grouped)
            .sort((a, b) => (new Date(a).getTime() > new Date(b).getTime() ? -1 : 1))
            .reduce((a, b: any) => {
              a[b] = grouped[b]
              return a
            }, {} as any)

          this.sortedItems = sorted
          return this.items
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

//@ts-ignore
function _extractTimestamp({ comment, image, album, user }: ActivityItem): number {
  if (comment) return comment.createdAt
  else if (image) return image.uploadedAt
  else if (album) return album.publishedAt
  else if (user) return user.createdAt
}

export function _formatDate(date: Date) {
  return [padTo2Digits(date.getMonth() + 1), padTo2Digits(date.getDate()), date.getFullYear()].join("/")
}
