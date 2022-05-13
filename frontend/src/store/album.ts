import { defineStore } from "pinia"
import { get, makeQuery, post } from "../js/fetch"
import { useLoading } from "./loading"
import { useToast } from "./toast"
import { isObject } from "lodash"
import { FetchError } from "../js/types/errorTypes"
export interface Image {
  key: string
  createdAT: number
}
export interface Album {
  key: string
  createdAt: number
  images: Array<Image>
}

interface State {
  albums: Array<Album>
  userAlbums: {
    [key: string]: Album
  }
}

export interface NewAlbum {
  title: string
  description?: string
  locations?: Array<string>
  timeframe: {
    from: number
    to: number
  }
  imageKeys: Array<string>
  userKeys?: Array<string>
}

export const useAlbums = defineStore("album", {
  state: () =>
    ({
      albums: [],
      userAlbums: {}
    } as State),
  actions: {
    async fetchAlbum(id: string | string[]) {
      const { addLoading, delLoading } = useLoading()

      addLoading("get-album")

      return get(`/api/albums/${id}`)
        .then((data) => {
          this.albums.push(data)
          return data
        })
        .catch((error: FetchError) => {
          const toast = useToast()
          toast.add(isObject(error) ? error.message : String(error), "error")
        })
        .finally(() => delLoading("get-album"))
    },

    async fetchAlbums() {
      const { addLoading, delLoading } = useLoading()
      addLoading("albums")

      return get("/api/albums/")
        .then((albums) => {
          return albums
        })
        .catch((error: FetchError) => {})
        .finally(() => delLoading("albums"))
    },

    async fetchUserAlbums(user: string) {
      const { addLoading, delLoading } = useLoading()
      addLoading(`${user}-album`)

      return get(`/api/albums?user=${user}`)
        .then((albums) => {
          this.userAlbums[user] = albums

          return albums
        })
        .catch((error: FetchError) => {
          const toast = useToast()
          toast.add(isObject(error) ? error.message : String(error), "error")
        })
        .finally(() => delLoading(`${user}-album`))
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
          console.log(error)
        })
        .finally(() => delLoading("add-album"))
    }
  },
  getters: {
    getAlbums: (state) => state.albums,
    getAlbum: (state) => (id: string) => state.albums.find((album) => album.key === id)
  }
})
