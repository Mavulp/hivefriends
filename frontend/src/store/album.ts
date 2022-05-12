import { defineStore } from "pinia"
import { get, makeQuery, post } from "../js/fetch"
import { useLoading } from "./loading"

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
}

export const useAlbums = defineStore("album", {
  state: () =>
    ({
      albums: []
    } as State),
  actions: {
    async fetchAlbum(id: string | string[]) {
      const { addLoading, delLoading } = useLoading()

      addLoading("get-album")

      await new Promise((resolve) => setTimeout(() => resolve(true), 1000))

      return await get(`/api/albums/${id}`)
        .then((data) => {
          this.albums.push(data)
          return data
        })
        .catch((error) => {
          console.log(error)
        })
        .finally(() => {
          delLoading("get-album")
        })
    },

    async fetchAlbums() {},
    async addAlbum(album: Album) {
      const { addLoading, delLoading } = useLoading()

      addLoading("add-album")

      return await post("/api/albums", album)
        .then((key) => {
          // Redirect to page with data.key
          return key
        })
        .catch((error) => {
          console.log(error)
        })
        .finally(() => {
          delLoading("add-album")
        })
    }
  },
  getters: {
    getAlbums: (state) => state.albums,
    getAlbum: (state) => (id: string) => state.albums.find((album) => album.key === id)
  }
})
