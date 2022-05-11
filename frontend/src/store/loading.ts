import { defineStore } from "pinia"

export const useLoading = defineStore("loading", {
  state: () => ({
    loading: new Set()
  }),
  actions: {
    addLoading(...items: Array<string>) {
      if (items.length > 0) {
        for (const item in items) {
          this.loading.add(item)
        }
      }
    },
    delLoading(...items: Array<string>) {
      if (items.length > 0) {
        for (const item in items) {
          this.loading.delete(item)
        }
      }
    }
  },
  getters: {
    getLoading:
      (state) =>
      (...items: Array<string>) => {
        if (items.length > 0) {
          return [...state.loading].some((item: any) => items.includes(item))
        }

        return state.loading.size > 0
      }
  }
})
