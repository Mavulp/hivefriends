import { defineStore } from "pinia"
import { get, rootUrl, post, put, del } from "../js/fetch"
import { useLoading } from "./loading"
import { useToast } from "./toast"
import { FetchError } from "../js/global-types"
import { useFilters } from "./filters"
import { query } from "../js/query"
import { remove } from "lodash"

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

export interface AllImageItem extends Image {
  albumKeys: Array<string>
}

export interface ImageFile {
  values: Array<{
    name: string
    size: number
    loading: boolean
    error?: string
    key: string | null
  }>
}

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
  drafts: Array<Album>
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
      drafts: [],
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

    async fetchDrafts() {
      this.drafts = await this.fetchAlbums(true)
    },

    async fetchAlbums(draft: boolean = false) {
      const { addLoading, delLoading } = useLoading()
      const filters = useFilters()
      addLoading("albums")

      const q = query({
        filters: filters.active,
        draft
      })

      return get(`/api/albums/${q}`)
        .then((albums) => {
          return albums
        })
        .catch((error: FetchError) => {
          const toast = useToast()
          toast.add(error.message, "error")
        })
        .finally(() => delLoading("albums"))
    },

    async fetchUserAlbums(user: string, draft: boolean = false) {
      const { addLoading, delLoading } = useLoading()
      const filters = useFilters()

      addLoading(`albums`)

      const q = query({
        filters: {
          ...filters.active,
          authors: [user]
        },
        draft
      })

      return get(`/api/albums/${q}`)
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
      const toast = useToast()

      addLoading(key)

      return put(`/api/images/${key}`, form)
        .then((res) => {
          toast.add("Updated image metadata", "success")
        })
        .catch((error: FetchError) => {
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

    async deleteAlbum(key: string) {
      const { addLoading, delLoading } = useLoading()
      const toast = useToast()

      addLoading("delete-album")

      return del(`/api/albums/${key}`)
        .then(() => {
          remove(this.albums, (item) => item.key === key)
          toast.add("Successfully deleted album", "success")
          return true
        })
        .catch((error: FetchError) => {
          toast.add(error.message, "error")
        })
        .finally(() => delLoading("delete-album"))
    },

    async editAlbum(key: string, album: NewAlbum) {
      const { addLoading, delLoading } = useLoading()
      const toast = useToast()

      addLoading("edit-album-submit")

      return put(`/api/albums/${key}`, album)
        .then((res) => {
          console.log(res)
          toast.add("Successfully updated album", "success")
        })
        .catch((error: FetchError) => {
          toast.add(error.message, "error")
        })
        .finally(() => delLoading("edit-album-submit"))
    },

    async fetchUserImages() {
      const { addLoading, delLoading } = useLoading()
      const toast = useToast()

      addLoading("images")

      return get("/api/images")
        .then((data) => data)
        .catch((error: FetchError) => {
          toast.add(error.message, "error")
          return []
        })
        .finally(() => delLoading("images"))
    }
  },
  getters: {
    getUserAlbums: (state) => (username: string) => state.userAlbums[username],
    getAlbums: (state) => state.albums,
    getAlbum: (state) => (key: string) => state.albums.find((album) => album.key === key),
    getImageMetadata: (state) => (key: string) => state.imageMetadata[key] ?? null
  }
})

type sizes = "full" | "large" | "medium" | "tiny"

export function imageUrl(key: string, size: sizes = "full") {
  if (!key) return ""
  return rootUrl + `/data/image/${key}/${size}.jpg`
}
