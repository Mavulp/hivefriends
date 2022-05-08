import { defineStore } from "pinia"

export const authStore = defineStore("auth", {
  state: () => ({
    user: null
  }),
  actions: {},
  getters: {}
})
