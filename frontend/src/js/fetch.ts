const rootUrl = "https://friends.hivecom.net"

export function get(url: string, options?: object) {
  return _handleFetch(
    url,
    Object.assign(
      {
        method: "GET"
      },
      options
    )
  )
}

export function post(url: string, body: object, options?: object) {
  return _handleFetch(
    url,
    Object.assign(
      {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(body)
      },
      options
    )
  )
}

export function put(url: string, body: object, options?: object) {
  return _handleFetch(
    url,
    Object.assign(
      {
        method: "PUT",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(body)
      },
      options
    )
  )
}

export function del(url: string) {
  const options = {
    method: "DELETE"
  }

  return _handleFetch(url, options)
}

export function makeQuery(options: object) {
  if (Object.keys(options).length === 0) return ""

  let q = ""

  Object.entries(options).map(([value, key]) => {
    q += `&${key}=${value}`
  })

  // Replace first & with ?
  return "?" + q.substring(1)
}

// Private handler functions

async function _handleFetch(url: string, options: object) {
  const token = localStorage.getItem("bearer_token")

  Object.assign(options, {
    mode: "cors",
    Authorization: `Bearer ${token}`
  })

  return fetch(rootUrl + url, options).then(_handleResponse)
}

async function _handleResponse(response: Response) {
  return response.text().then((text: string) => {
    const data = text && JSON.parse(text)

    if (!response.ok) {
      const error = (data && data.message) || response.statusText
      return Promise.reject(error)
    }

    return data
  })
}
