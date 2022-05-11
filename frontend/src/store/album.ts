import { defineStore } from "pinia"
import { get, makeQuery } from "../js/fetch"
import { useLoading } from "./loading"

interface Album {
  id: string
  name: string
  description: string
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
    async fetchAlbum(id: string) {
      const { addLoading, delLoading } = useLoading()

      addLoading("albums")

      const query = makeQuery({ id })

      await get(`/api/albums${query}`)
        .then((data) => {
          console.log(data)
        })
        .catch((error) => {
          console.log(error)
        })

      delLoading("albums")
    },

    async fetchAlbums() {}
  },
  getters: {
    getAlbums: (state) => state.albums,
    getAlbum: (state) => (id: string) => state.albums.find((album) => album.id === id)
  }
})
