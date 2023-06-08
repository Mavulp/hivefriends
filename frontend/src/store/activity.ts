import { defineStore } from 'pinia'
import { partition } from 'lodash'
import { get, post } from '../js/fetch'
import type { FetchError } from '../js/global-types'
import { padTo2Digits } from '../js/utils'
import { useLoading } from './loading'
import { useToast } from './toast'
import type { Comment } from './comments'
import type { Album, ImageItemInAlbum } from './album'
import type { User } from './user'

export interface ReducedImage {
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

export const useActivity = defineStore('activity', {
  state: () => ({
    items: [],
    sortedItems: {},
    hasNew: false,
    open: false,
  } as State),
  actions: {
    async fetchActivity() {
      const { addLoading, delLoading } = useLoading()

      addLoading('activity')

      return get('/api/activity')
        .then((response) => {
          this.items = response
          this.hasNew = true

          // Group activity by day
          const grouped: { [key: string]: ActivityItem[] } = {}

          this.items.forEach((item: ActivityItem) => {
            // Extract timestamp item from different data sources
            const timestamp = _extractTimestamp(item) * 1000
            const date = _formatDate(new Date(timestamp))

            // If date is not yet present, assign an empty array to it
            if (!grouped[date])
              grouped[date] = [item]
            else
              grouped[date].push(item)
          })

          // Sort object keys to be descending
          const sorted = Object.keys(grouped)
            .sort((a, b) => (new Date(a).getTime() > new Date(b).getTime() ? -1 : 1))
            .reduce((a, b: any) => {
              const day = grouped[b]
              const [images, rest] = partition(day, item => Reflect.has(item, 'image'))

              // Consolidate images into objects by the user
              const reduced: { [key: string]: ReducedImage } = images.reduce((user: any, item) => {
                // @ts-expect-error (We know this will only be an image you fucking idiot)
                const img: Image = item.image

                if (!user[img.uploader]) {
                  user[img.uploader] = {
                    user: img.uploader,
                    images: [],
                  }
                }

                user[img.uploader].images.push(img)

                return user
              }, {})

              for (const item of Object.values(reduced))
                rest.push({ image: item })

              a[b] = rest
              return a
            }, {} as any)

          this.sortedItems = sorted
          return this.items
        })
        .catch((error: FetchError) => {
          const toast = useToast()
          toast.add(error.message, 'error')
          return []
        })
        .finally(() => {
          delLoading('activity')
        })
    },
    async updateView() {
      return post('/api/activity', '')
    },
  },
  getters: {
    getActivity: state => state.items,
    /**
     * This is the gettter for feed page. Display images only
     *
     * 1. Filter out only image items from the activity
     * 2. Group them by date
     * 3. Within the date group, group them by albums
     */
    // getImageActivity: state => state.items
    //   .filter(item => Object.hasOwn(item, 'image'))
  },
})

// @ts-expect-error Just fuckiny cry about it
export function _extractTimestamp({ comment, image, album, user }: ActivityItem): number {
  if (comment)
    return comment.createdAt
  else if (image)
    return image.uploadedAt
  else if (album)
    return album.publishedAt
  else if (user)
    return user.createdAt
}

export function _formatDate(date: Date) {
  return [padTo2Digits(date.getMonth() + 1), padTo2Digits(date.getDate()), date.getFullYear()].join('/')
}
