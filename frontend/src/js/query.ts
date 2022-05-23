export function query(options: object) {
  if (Object.keys(options).length === 0) return ""

  let q = ""

  Object.entries(options).map(([key, value]) => {
    q += `&${key}=${value}`
  })

  // TODO parse key 'year' into a from

  // Replace first & with ?
  return "?" + q.substring(1)
}
