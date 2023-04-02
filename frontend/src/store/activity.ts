import { defineStore } from "pinia"
import { get, post } from "../js/fetch"
import { useLoading } from "./loading"
import { useToast } from "./toast"
import { FetchError } from "../js/global-types"
import { Comment } from "./comments"
import { Album, Image, ImageItemInAlbum } from "./album"
import { User } from "./user"
import { padTo2Digits } from "../js/utils"
import { partition } from "lodash"

export type ReducedImage = {
  user: string
  images: ImageItemInAlbum[]
}

export type ActivityItem =
  | {
    comment: Comment
  }
  | {
    image: ReducedImage
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
          // this.items = response.slice(0, 128)
          this.items = response
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
              const day = grouped[b]
              const [images, rest] = partition(day, (item) => Reflect.has(item, "image"))

              // Consolidate images into objects by the user
              const reduced: { [key: string]: ReducedImage } = images.reduce((user: any, item) => {
                //@ts-ignore (We know this will only be an image you fucking idiot)
                const img: Image = item.image

                if (!user[img.uploader]) {
                  user[img.uploader] = {
                    user: img.uploader,
                    images: []
                  }
                }

                user[img.uploader].images.push(img)

                return user
              }, {})

              for (const item of Object.values(reduced)) {
                rest.push({ image: item })
              }

              a[b] = rest
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
