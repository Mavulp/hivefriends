<script setup lang='ts'>
import { computed, onBeforeMount, ref } from 'vue'
import dayjs from 'dayjs'
import { vElementVisibility } from '@vueuse/components'
import { useScroll, whenever } from '@vueuse/core'
import { useActivity } from '../../store/activity'
import type { ReducedImage } from '../../store/activity'
import type { ImageItemInAlbum } from '../../store/album'
import UserUpload from '../../components/feed/UserUpload.vue'
import { useThresholdScroll } from '../../js/_composables'

/**
 * SECTION Feed todo
 *
 * 3. Fix slider breaking when window is resized
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

// Save is last scroll was up or down
const { directions, y } = useScroll(window)
const scrolledUp = ref(false)

whenever(() => directions.top, () => scrolledUp.value = true)
whenever(() => directions.bottom, () => scrolledUp.value = false)
whenever(() => y.value < 156, () => activeSection.value = visible.value[0])

function setVisibleEl(isVisible: boolean, date: string) {
  if (isVisible) {
    if (scrolledUp.value) {
      visible.value.unshift({
        date,
        el: document.querySelector(`[data-date="${dayjs(date).unix()}"]`) as HTMLDivElement,
      })
    }
    else {
      visible.value.push({
        date,
        el: document.querySelector(`[data-date="${dayjs(date).unix()}"]`) as HTMLDivElement,
      })
    }
  }
  else {
    visible.value = visible.value.filter(item => item.date !== date)
    activeSection.value = visible.value.at(scrolledUp.value ? -1 : 0)

    return
  }

  // Ran on first iteration
  if (!activeSection.value)
    activeSection.value = visible.value.at(scrolledUp.value ? -1 : 0)
}

// Scroll up
const { passed, scroll } = useThresholdScroll(292)

// Sroll to next post

function scrollNext() {
  window.scrollBy({
    top: window.innerHeight / 100 * 75,
    behavior: 'smooth',
  })
}
</script>

<template>
  <div class="hi-feed">
    <div class="feed-sidebar">
      <div v-if="activeSection" class="feed-sidebar-scroll">
        <Transition mode="out-in" :name="scrolledUp ? 'fadedown' : 'fadeup'" appear>
          <span :key="activeSection.date"> {{ dayjs(activeSection.date).format('DD MMMM') }}</span>
        </Transition>

        <div style="height:25.5px">
          <Transition name="fade" mode="out-in">
            <button v-if="passed" class="hover-bubble" data-title-right="Scroll Up" @click="scroll">
              <span class="material-icons"> &#xe5ce; </span>
            </button>
          </Transition>
        </div>

        <button class="hover-bubble" data-title-right="Next Post" @click="scrollNext">
          <span class="material-icons"> &#xe313; </span>
        </button>
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
