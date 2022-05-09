import { defineStore } from "pinia"

export const authStore = defineStore("auth", {
  state: () => ({
    user: null,
    logged: false
  }),
  actions: {},
  getters: {
    isLoggedIn: (state) => state.logged
  }
})
