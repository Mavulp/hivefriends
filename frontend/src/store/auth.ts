import { defineStore } from "pinia"

export const useAuth = defineStore("auth", {
  state: () => ({
    user: null,
    logged: false
  }),
  actions: {},
  getters: {
    isLoggedIn: (state) => state.logged
  }
})
