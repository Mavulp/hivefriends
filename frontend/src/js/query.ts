import { Options } from "../store/filters"
import { isEmpty, isArray } from "lodash"

export interface QueryOptions {
  draft?: boolean
  filters?: Options
}

export function query(options: QueryOptions) {
  if (Object.keys(options).length === 0) return ""

  let q = ""

  const add = (key: string, value: any) => (q += `&${key}=${value}`)

  const { draft, filters } = options

  if (filters && !isEmpty(filters)) {
    Object.entries(filters).map(([key, value]) => {
      if (key === "years") {
        const sorted = value.map((year: any) => Number(year)).sort()

        const from = new Date(sorted.at(0) ?? 0, 0).getTime() / 1000
        const to = new Date((sorted.at(-1) ?? 0) + 1, 0).getTime() / 1000

        add("from", from)
        add("to", to)
      } else {
        add(key, isArray(value) ? value.join(",") : value)
      }
    })
  }

  add("draft", draft ? true : false)

  // Replace first "&" with "?"
  return "?" + q.substring(1)
}
