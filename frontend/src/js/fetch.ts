import { merge } from "lodash"

export const rootUrl = "https://friends.hivecom.net"

export function get(url: string, options?: object) {
  return _handleFetch(
    url,
    merge(
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
    merge(
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
    merge(
      {
        method: "PUT",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(body)
      },
      options
    )
  )
}

/**
 * Special function to handle file uploads
 */

export function upload(url: string, body: object, options?: object) {
  return _handleFetch(url, {
    method: "POST",
    body,
    ...options
  })
}

export function del(url: string) {
  const options = {
    method: "DELETE"
  }

  return _handleFetch(url, options)
}

// Private handler functions

async function _handleFetch(url: string, options: object) {
  const token = localStorage.getItem("bearer_token")

  merge(options, {
    mode: "cors",
    headers: {
      Authorization: `Bearer ${token}`
    }
  })

  return fetch(rootUrl + url, options).then(_handleResponse)
}

async function _handleResponse(response: Response) {
  return response.text().then((text: string) => {
    const data = text && JSON.parse(text)

    if (!response.ok) {
      const error = (data && data.message) || { message: response.statusText }
      return Promise.reject(error)
    }

    return data
  })
}
