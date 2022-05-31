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
