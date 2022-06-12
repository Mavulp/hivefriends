import { Image } from "../store/album"

/**
 * Composable for replacing @user with a link to user's profile
 */

export function formatTextUsernames(text: string, userStore: any) {
  const _regex = /@(\w+)/g
  return text.replaceAll(_regex, (original: string, parsed: string) => {
    // console.log(one, two, three)

    if (!userStore.getUser(parsed)) return original

    return /*html*/ `<button class="comment-user-link" data-comment-link="${parsed}" href="/${parsed}">${original}</button>`
  })
}

export function getImageChunks(images: Array<Image>, columns = 3) {
  if (!images) return []

  const makeArray = (cols: number) => Array.from(Array(cols).keys()).map(() => [])

  const chunks: Array<Array<Image>> = makeArray(columns)

  let i: number = 0
  let j: number = 0

  while (i !== images.length) {
    chunks[j].push(images[i])

    if (j >= columns - 1) j = 0
    else j++

    i++
  }

  return chunks
}
