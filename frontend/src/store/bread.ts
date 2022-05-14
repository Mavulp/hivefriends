import { defineStore } from "pinia"

interface State {
  title: null | string
}

export const useBread = defineStore("bread", {
  state: () =>
    ({
      title: null
    } as State),
  actions: {
    set(value: any, title?: any) {
      this.title = value
      document.title = title ?? value
    },
    clear() {
      this.title = null
    }
  }
})
