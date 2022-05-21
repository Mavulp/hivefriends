import { defineStore } from "pinia"

export type Toast = {
  text: string
  type?: string
  timeout?: number
}

interface State {
  items: Array<Toast>
}

export const useToast = defineStore("toast", {
  state: () =>
    ({
      items: []
    } as State),
  actions: {
    add(text: string, type: string = "info") {
      this.items.unshift({ text, type, timeout: type === "error" ? 6000 : 3000 })
    },
    del(index: number) {
      this.items.splice(index, 1)
    },
    clear() {
      this.items = []
    }
  }
})
