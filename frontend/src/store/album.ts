import { defineStore } from "pinia"
import { get, rootUrl, post, put, del } from "../js/fetch"
import { useLoading } from "./loading"
import { useToast } from "./toast"
import { FetchError } from "../js/global-types"

type Modify<T, R> = Omit<T, keyof R> & R

export interface Image {
  key: string
  fileName: string
  sizeBytes: number
  takenAt: number
  location?: {
    latitude: string | number
    longitude: string | number
  }
  cameraBrand: string
  cameraModel: string
  exposureTime: string
  fNumber: string
  focalLength: string
  description?: string
  uploader: string
  uploadedAt: number
}

// export type ImageWithLocation = Omit<Image, "location"> & {
//   location: {
//     latitude: string | number
//     longitude: string | number
//   }
// }

export interface Album {
  key: string
  title: string
  description: string
  createdAt: number
  draft?: boolean
  images: Array<Image>
  timeframe: {
    from: number
    to: number
  }
  author: string
  coverKey: string
  taggedUsers: Array<string>
}

interface State {
  albums: Array<Album>
  userAlbums: {
    [key: string]: Album
  }
  imageMetadata: {
    [key: string]: Image
  }
}

export interface NewAlbum {
  title: string
  description?: string
  timeframe: {
    from: number
    to: number
  }
  imageKeys: Array<string>
  taggedUsers?: Array<string>
  coverKey: string
  draft: boolean
}

export const useAlbums = defineStore("album", {
  state: () =>
    ({
      albums: [],
      userAlbums: {},
      imageMetadata: {}
    } as State),
  actions: {
    async genPublicAlbumToken(albumKey: string) {
      return post(`/api/public/albums/${albumKey}/`, {})
        .then((response) => response.token)
        .catch((error: FetchError) => {
          const toast = useToast()
          toast.add(error.message, "error")
        })
    },
    async fetchAlbum(id: string | string[], token?: string | string[]) {
      const { addLoading, delLoading } = useLoading()

      addLoading("get-album")

      const query = token ? `/api/public/albums/${id}/${token}/` : `/api/albums/${id}/`

      return get(query)
        .then((data) => {
          this.albums.push(data)
          return data
        })
        .catch((error: FetchError) => {
          const toast = useToast()
          toast.add(error.message, "error")
        })
        .finally(() => delLoading("get-album"))
    },

    async fetchAlbums(draft: boolean = false) {
      const { addLoading, delLoading } = useLoading()
      addLoading("albums")

      return get(`/api/albums/?draft=${draft}`)
        .then((albums) => {
          return albums
        })
        .catch((error: FetchError) => {
          const toast = useToast()
          toast.add(error.message, "error")
        })
        .finally(() => delLoading("albums"))
    },

    async fetchUserAlbums(user: string) {
      const { addLoading, delLoading } = useLoading()
      addLoading(`albums`)

      return get(`/api/albums/?user=${user}`)
        .then((albums) => {
          this.userAlbums[user] = albums

          return albums
        })
        .catch((error: FetchError) => {
          const toast = useToast()
          toast.add(error.message, "error")
        })
        .finally(() => {
          delLoading(`albums`)
        })
    },

    async fetchImageMetadata(key: string) {
      const { addLoading, delLoading } = useLoading()

      addLoading(key)

      return get(`/api/images/${key}`)
        .then((data) => {
          this.imageMetadata[key] = data
          return data
        })
        .catch((error: FetchError) => {
          const toast = useToast()
          toast.add(error.message, "error")
        })
        .finally(() => delLoading(key))
    },

    async saveImageMetadata(key: string, form: object) {
      const { addLoading, delLoading } = useLoading()

      addLoading(key)

      return put(`/api/images/${key}`, { form })
        .then(() => {})
        .catch((error: FetchError) => {
          const toast = useToast()
          toast.add(error.message, "error")
        })
        .finally(() => delLoading(key))
    },

    async addAlbum(album: NewAlbum) {
      const { addLoading, delLoading } = useLoading()

      addLoading("add-album")

      return post("/api/albums/", album)
        .then((key) => {
          // Redirect to page with data.key
          return key
        })
        .catch((error: FetchError) => {
          const toast = useToast()
          toast.add(error.message, "error")
        })
        .finally(() => delLoading("add-album"))
    },

    async deleteAlbum(albumKey: string) {
      const { addLoading, delLoading } = useLoading()
      const toast = useToast()

      addLoading("delete-album")

      return del(`/api/albums/${albumKey}`)
        .then(() => {
          toast.add("Successfully deleted album", "success")
          return true
        })
        .catch((error: FetchError) => {
          console.log(error)

          toast.add(error.message, "error")
        })
        .finally(() => delLoading("delete-album"))
    }
  },
  getters: {
    getUserAlbums: (state) => (username: string) => state.userAlbums[username],
    getAlbums: (state) => state.albums,
    getAlbum: (state) => (key: string) => state.albums.find((album) => album.key === key),
    getImageMetadata: (state) => (key: string) => state.imageMetadata[key] ?? null
  }
})

export function imageUrl(key: string, size: string = "original") {
  if (!key) return ""
  return rootUrl + `/data/image/${key}/${size}.png`
}
