<script setup lang='ts'>
import { computed, onBeforeMount } from 'vue'
import dayjs from 'dayjs'
import { useActivity } from '../../store/activity'
import type { ReducedImage } from '../../store/activity'
import type { ImageItemInAlbum } from '../../store/album'
import UserUpload from '../../components/feed/UserUpload.vue'

/**
 * SECTION Feed todo
 *
 * 1. Refactor comments to store comments by album & image key
 * 2. Add key x album key loading to comments
 * 3. Re-initialize slider on window reload
 * 4. Add an arrow button to the left sidebar which will scroll to the next album item (that is not on the screen) (intersection observer crap?)
 * 5. Move sidebar next to the entire for-loop wrapper and display dates based on intersection observer.
 *    This allows for a nice transition between dates.
 */

const activity = useActivity()
onBeforeMount(activity.fetchActivity)

const formattedActivity = computed(() => {
  const raw = activity.sortedItems

  // Limit the activity to 30 items
  const iterationLimit = 30
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
</script>

<template>
  <div class="hi-feed">
    <div class="hi-feed-wrap">
      <div v-for="(users, date, index) in formattedActivity" :key="date" class="feed-item-wrap">
        <div class="feed-sidebar">
          <div class="feed-sidebar-scroll">
            <span>
              {{ dayjs(date).format('MMMM DD, YYYY') }}
            </span>
          </div>
        </div>
        <div class="feed-date">
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
  </div>
</template>
