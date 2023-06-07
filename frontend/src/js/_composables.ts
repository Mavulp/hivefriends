// import { ImageItemInAlbum, Image } from "../store/album"
import { computed } from 'vue'
import { useWindowScroll } from '@vueuse/core'
import { isValidImage } from './utils'

/**
 * Composable for replacing @user with a link to user's profile
 */

export function formatTextUsernames(text: string, userStore: any) {
  const _regex = /@(\w+)/g
  return text.replaceAll(_regex, (original: string, parsed: string) => {
    // console.log(one, two, three)
    const user = userStore.getUser(parsed)

    if (!user)
      return original

    return /* html */ `<button class="comment-user-link" data-comment-link="${user.username}" href="/${user.username}">${original}</button>`
  })
}

/**
 * Takes in an array of images and splits them into chunks
 */

export function getImageChunks(images: Array<any>, columns = 3) {
  if (!images)
    return []

  const makeArray = (cols: number) => Array.from(Array(cols).keys()).map(() => [])

  const chunks: Array<Array<any>> = makeArray(columns)

  let i = 0
  let j = 0

  while (i !== images.length) {
    chunks[j].push(images[i])

    if (j >= columns - 1)
      j = 0
    else j++

    i++
  }

  return chunks
}

/**
 *
 */

export function formatTextImages(text: string) {
  const _regex = /\bhttps?:\/\/\S+/gi

  const urls = [...new Set(text.match(_regex))]

  const _img = (_url: string) => `<img src="${_url}" />`
  const _a = (_url: string) => `<a href="${_url}" target="_blank">${_url}</a>`

  if (urls.length > 0) {
    // Loop over each url
    for (const url of urls) {
      let chunk

      if (isValidImage(url)) {
        // Is an image
        chunk = _img(url)
      }
      else {
        // is a link
        chunk = _a(url)
      }

      text = text.replaceAll(url, chunk)
    }
  }

  return text
}

// Returns scroll Up function and a check if users scrolled beyond a threshold

export function useThresholdScroll(threshold: number) {
  const { y } = useWindowScroll()

  return {
    scroll: () => window.scrollTo({ top: 0, behavior: 'smooth' }),
    passed: computed(() => y.value >= threshold),
  }
}
