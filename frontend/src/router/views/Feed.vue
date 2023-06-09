<script setup lang='ts'>
import { computed, onBeforeMount, ref } from 'vue'
import dayjs from 'dayjs'
import { vElementVisibility } from '@vueuse/components'
import { useScroll } from '@vueuse/core'
import { useActivity } from '../../store/activity'
import type { ReducedImage } from '../../store/activity'
import type { ImageItemInAlbum } from '../../store/album'
import UserUpload from '../../components/feed/UserUpload.vue'

/**
 * SECTION Feed todo
 *
 * 1. [x] Refactor comments to store comments by album & image key
 * 2. [x] Add key x album key loading to comments
 * 3. Fix slider breaking when window is resized
 * 4. Add an arrow button to the left sidebar which will scroll to the next album item (that is not on the screen) (intersection observer crap?)
 * 5. Move sidebar next to the entire for-loop wrapper and display dates based on intersection observer.
 *    This allows for a nice transition between dates.
 * 6. [x] Fix comments overflowing (should have a wrapper with leftover flex1 and entire comments component will inset:0 into it)
 */

const activity = useActivity()
onBeforeMount(activity.fetchActivity)

const formattedActivity = computed(() => {
  const raw = activity.sortedItems

  // Limit the activity to 30 items
  const iterationLimit = 15
  let iterations = 0

  return Object.entries(raw)
    .reduce((group, [key, value]) => {
      if (iterations >= iterationLimit)
        return group

      const filtered = value
        .filter(item => Object.hasOwn(item, 'image'))
      // @ts-expect-error The previous iteration removes all types except the image one
        .map(item => item.image)

      // Ignore empty dates
      if (filtered.length < 1)
        return group

      group[key] = filtered
        // Iterate over each activity item and sort users images from the newest to the oldest
        .map((user) => {
          user.images.sort((a: ImageItemInAlbum, b: ImageItemInAlbum) => b.uploadedAt - a.uploadedAt)
          return user
        })
        // Now sort all entries within the date based on user's newest image upload
        .sort((a, b) => {
          return b.images[0].uploadedAt - a.images[0].uploadedAt
        })

      iterations++
      return group
    }, {} as Record<string, ReducedImage[]>)
})

// Observe scrolling and get the .feed-date element that is currently on screen
interface VisibleItem {
  date: any
  el: HTMLDivElement
}

const visible = ref<VisibleItem[]>([])
const activeSection = ref<VisibleItem>()

const { directions } = useScroll(window)

function setVisibleEl(isVisible: boolean, date: string) {
  if (isVisible) {
    visible.value.push({
      date,
      el: document.querySelector(`[data-date="${dayjs(date).unix()}"]`) as HTMLDivElement,
    })
  }
  else {
    visible.value = visible.value.filter(item => item.date !== date)
    activeSection.value = visible.value[0]

    return
  }

  // Ran on first iteration
  if (!activeSection.value)
    activeSection.value = visible.value[0]
}
</script>

<template>
  <div class="hi-feed">
    <div class="feed-sidebar">
      <div v-if="activeSection" class="feed-sidebar-scroll">
        <Transition mode="out-in" name="pagetransition" appear>
          <span :key="activeSection.date"> {{ dayjs(activeSection.date).format('DD MMMM YYYY') }}</span>
        </Transition>
      </div>
    </div>
    <div class="hi-feed-wrap">
      <div
        v-for="(users, date, index) in formattedActivity"
        :key="date"
        v-element-visibility="(state) => setVisibleEl(state, date)"
        class="feed-date"
        :data-date="dayjs(date).unix()"
      >
        <UserUpload
          v-for="item in users"
          :key="item.user"
          :user="item.user"
          :images="item.images"
          :index="index"
        />
      </div>
    </div>
  </div>
</template>
