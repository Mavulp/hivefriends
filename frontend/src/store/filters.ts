import { isArray } from "lodash"
import { defineStore } from "pinia"

type Filter = {
  [key: string]: Set<string>
}

interface Filters {
  active: Filter
  available: Filter
}

export const useFilters = defineStore("filters", {
  state: () =>
    ({
      active: {},
      available: {}
    } as Filters),
  actions: {
    set(key: string, value: string | Array<string> | null) {
      // If filters dont exist or value is null, remove the active filters completely
      if (!this.active[key] || !value) this.active[key] = new Set()

      if (typeof value === "string") {
        this.active[key].add(value)
      } else if (isArray(value)) {
        value.map((item) => this.active[key].add(item))
      }
    },
    del(key: string) {
      this.active[key] = new Set()
    },
    reset() {
      this.active = {}
      this.available = {}
    },
    createOptions(data: Array<any>) {
      // Iterate over all datasets and create options
      // Add custom filters
    }
  },
  getters: {
    getAvailableFilters: (state) => state.available,
    getActiveFilters: (state) => state.active,
    getActiveFilter: (state) => (key: string) => state.active[key]
  }
})
