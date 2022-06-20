// import { AllImageItem, Image } from "../store/album"

/**
 * Composable for replacing @user with a link to user's profile
 */

export function formatTextUsernames(text: string, userStore: any) {
  const _regex = /@(\w+)/g
  return text.replaceAll(_regex, (original: string, parsed: string) => {
    // console.log(one, two, three)
    const user = userStore.getUser(parsed)

    if (!user) return original

    return /*html*/ `<button class="comment-user-link" data-comment-link="${user.username}" href="/${user.username}">${original}</button>`
  })
}

export function getImageChunks(images: Array<any>, columns = 3) {
  if (!images) return []

  const makeArray = (cols: number) => Array.from(Array(cols).keys()).map(() => [])

  const chunks: Array<Array<any>> = makeArray(columns)

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
