<script setup lang="ts">
import { computed, onBeforeMount, ref } from 'vue'
import dayjs from 'dayjs'
import HomeUser from '../../components/user/HomeUser.vue'
import Activity from '../../components/activity/Activity.vue'

import type { Album } from '../../store/album'
import { imageUrl, useAlbums } from '../../store/album'
import { useUser } from '../../store/user'
import { useBread } from '../../store/bread'
import { useActivity } from '../../store/activity'
import { TEXT_CONTRAST, seedRndMinMax } from '../../js/utils'
import LoadingSpin from '../../components/loading/LoadingSpin.vue'

const user = useUser()
const album = useAlbums()
const bread = useBread()
const albums = ref([] as Album[])
const latest = computed<Album>(() => albums?.value[0] ?? null)
const activity = useActivity()

onBeforeMount(async () => {
  activity.fetchActivity()
  albums.value = await album.fetchAlbums()

  bread.set('Homepage | url -> irl')
})

const accent = computed(() => user.user.accentColor.split(',').map((item: string) => Number(item)))

// MOTD
const messages = [
  'They will never make a website for friends which want to cherish moments spent together.',
  'Comes with max 1 heat stroke.',
  'Yep we got MOTD now',
  'Now with even more watermelon',
  'Based and minted',
  'Chantyarno knows your location and is rapidly approaching',
  'Half of the people in these albums live in your walls',
  'Wang, wang, wang, wang, wang, wang, wang, wang, wang, HUUUUUUH???..., wang, wang, wang, wang',
  'Did you forget to post your daily picture?',
  'Are you more of a Ublumpf or a Schklabumf guy?',
  'This is the last UI update',
  'TTT and prop hunt next friday?',
  'Fishstick bouldering gym when',
  'Hide the shrooms before tmtu finds them',
  'WE ARE MAKING PESTO BITCH!!!!!!!!!!',
  'Wouldn\'t it be funny if you could play a banana?',
  'Fishstick house will feature a washing machine',
  'Never go full WANG WANG',
  'Caboose with moderation',
  'Don\'t forget to bring your controller to the next hike',
  'Can you help me sort the chests?',
  'How many warcrimes have you commited in IRC today?',
  'Daily lethal dosage of pepperonis',
  'SINGHAM!!',
  '!randota',
  '40. Invoker',
  'Pack it up there is no MOTD',
  'Tenacity in 1 word: 😐',
  '🤓🤓🤓',
  'Fishstick company will NOT sell fish',
  `Mavulp web services dropping in ${dayjs().add(1, 'month').format('MMMM')}`,
  `Big reveal on ${dayjs().add(3, 'day').format('dddd')}`,
  'NO WAY IM A MOTD TEXT!!!! HELLO MOM!!!!!!!!!!!!!',
  '>13750Hz (tinnitus)',
  '',
]

// We want to randomize the MOTD every hour
const date = dayjs().format('YYYYMMDDHH')
const randomIndex = seedRndMinMax(0, messages.length, date)
const motd = messages[randomIndex]
</script>

<template>
  <div class="hi-home">
    <div class="hi-double">
      <div>
        <div class="home-landing">
          <div class="album-thumbnail">
            <LoadingSpin v-if="!latest" dark />

            <router-link v-else :to="{ name: 'AlbumDetail', params: { id: latest.key } }">
              <div class="thumbnail-info" :class="[TEXT_CONTRAST(accent[0], accent[1], accent[2])]">
                <span>{{ latest.title }} by {{ user.getUsername(latest.author) }}</span>
              </div>

              <img :src="imageUrl(latest.coverKey, 'large')" alt="">
            </router-link>
          </div>

          <h1>friends</h1>
          <p>
            {{ motd }}
          </p>

          <!-- <div class="flex-1" />

          <div class="hero">
            <button class="button size-normal btn-highlight">
              Upload
            </button>
            <button class="button size-normal btn-white">
              View Albums
            </button>
          </div> -->
        </div>
      </div>

      <div class="home-other">
        <h5> Latest activity</h5>
        <Activity class="activity-home active" limit />

        <div v-if="user.users && user.users.length > 0">
          <h5>The squad</h5>

          <div class="home-users">
            <HomeUser v-for="item in user.users" :key="item.username" :data="item" />
          </div>
        </div>
      </div>
    </div>

    <!-- <p class="copyright">
      <span class="material-icons"> &#xe86f; </span>
      Made by <a target="_blank" href="https://github.com/mavulp">Mavulp</a> in {{ new Date().getFullYear() }}
    </p> -->

    <!-- <div v-if="latest" class="blur-bg">
      <img :src="imageUrl(latest.coverKey, 'tiny')" alt="">
    </div> -->
  </div>
</template>
