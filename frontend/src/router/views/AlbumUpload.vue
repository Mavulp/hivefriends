<script setup lang="ts">
import { computed, nextTick, onBeforeMount, onMounted, reactive, ref } from 'vue'
import { clone, isEmpty } from 'lodash'
import InputSelect from '../../components/form/InputSelect.vue'
import InputText from '../../components/form/InputText.vue'
import InputTextarea from '../../components/form/InputTextarea.vue'
import ImageUploadItem from '../../components/upload/ImageUploadItem.vue'
import Button from '../../components/Button.vue'
import LoadingSpin from '../../components/loading/LoadingSpin.vue'
import InputCheckbox from '../../components/form/InputCheckbox.vue'
import DraftItem from '../../components/upload/DraftItem.vue'

import { upload } from '../../js/fetch'
import { maxLength, required, useFormValidation } from '../../js/validation'
import type { Album, ImageFile, NewAlbum } from '../../store/album'
import { imageUrl, useAlbums } from '../../store/album'
import type { User } from '../../store/user'
import { useUser } from '../../store/user'
import { useLoading } from '../../store/loading'
import { useBread } from '../../store/bread'

const props = defineProps<Props>()
const store = useAlbums()
const user = useUser()
const bread = useBread()
const { addLoading, delLoading, getLoading } = useLoading()

interface Props {
  images?: string
}

/**
 * Setup
 */

const files = reactive<ImageFile>({ values: [] })
const album = reactive<NewAlbum>({
  title: '',
  description: '',
  timeframe: {
    from: 0,
    to: 0,
  },
  imageKeys: [],
  taggedUsers: [],
  coverKey: '',
  draft: false,
})

// const drafts = ref<Array<Album>>([])
const drafts = computed<Array<Album>>(() => store.drafts)

// If album was successfuly generated, this will get populated
const albumKey = ref()

const draggingOver = ref(false)
const singleDate = ref(false)
const rawFileLength = ref(0)

const isLoading = computed(() => files.values.some(file => file.loading))
const remainingProgress = computed(() => rawFileLength.value - [...files.values].filter(item => item.key).length)
const uploadPercentage = computed(
  () => ([...files.values].filter(item => item.key).length / rawFileLength.value) * 100,
)
const imageKeys = computed<Array<any>>(() => files.values.map(file => file.key).filter(item => item))

/**
 * Lifecycle
 */

const taggedUsers = ref<Array<string>>([])

onMounted(() => {
  const el = document.getElementById('drop-area')

  if (el) {
    el.addEventListener('dragenter', onSubmitHandler, false)
    el.addEventListener('dragleave', onSubmitHandler, false)
    el.addEventListener('dragover', onSubmitHandler, false)
    el.addEventListener('drop', onSubmitHandler, false)
    el.addEventListener('input', e => onSubmitHandler(e, true), false)
  }

  taggedUsers.value.push(user.user.username)
})

onBeforeMount(async () => {
  bread.set('Upload a new album')

  if (props.images) {
    // Parse JSON of already existing images and assign them as if they were uploaded
    const images = JSON.parse(props.images)

    if (images.length > 0) {
      for (const image of images) {
        files.values.push({
          name: image.fileName,
          size: image.sizeBytes,
          loading: false,
          key: image.key,
        })
      }
    }
  }

  addLoading('album-upload')
  await user.fetchUsers()
  await store.fetchDrafts()

  setTimeout(() => {
    delLoading('album-upload')
  }, 50)
})

/**
 * File Handling
 */

function onSubmitHandler(e: any, fromField = false) {
  e.preventDefault()
  e.stopPropagation()

  const files = fromField ? e.target.files : e.dataTransfer.files

  if (files.length > 0)
    uploadFiles(files)
}

async function uploadFiles(_files: any) {
  let i = files.values.length

  rawFileLength.value += _files.length

  for (const file of _files) {
    if (!file)
      continue

    const formData = new FormData()
    formData.append('file', file)

    await uploadFile(file, formData, clone(i))

    i++
  }
}

async function uploadFile(file: any, formData: any, index: number) {
  files.values[index] = {
    name: file.name,
    size: file.size,
    loading: true,
    key: null,
  }

  return upload('/api/images/', formData)
    .then((response: any) => {
      Object.assign(files.values[index], {
        loading: false,
        key: response.key,
      })
    })
    .catch((error) => {
      Object.assign(files.values[index], {
        loading: false,
        error,
      })
    })
}

function delImage(index: number) {
  if (files.values[index])
    files.values.splice(index, 1)
}

const rules = computed(() => ({
  title: { required, maxLength: maxLength(96) },
  description: { maxLength: maxLength(600) },
}))

const { validate, errors } = useFormValidation(album, rules, { autoclear: true })

async function submit() {
  validate().then(async () => {
    album.imageKeys = imageKeys.value

    const model = { ...album }

    // Assign properties which have to be formatted in some way or are not part of the origina form object
    Object.assign(model, {
      timeframe: {
        from: new Date(album.timeframe.from).getTime() / 1000,
        to: new Date(singleDate.value ? album.timeframe.from : album.timeframe.to).getTime() / 1000,
      },
      coverKey: album.coverKey ? album.coverKey : imageKeys.value[0],
      taggedUsers: taggedUsers.value,
    })

    const { key } = await store.addAlbum(model)

    if (key)
      albumKey.value = key
  })
}

/**
 * Tagging users
 */

function getUserImageKey(name: string): string {
  return user.users.find(item => item.username === name)?.avatarKey ?? ''
}

const userOptions = computed(() => {
  if (!user.users || user.users.length === 0)
    return null

  // Make sure you can't select yourself
  return user.users
    .map((item: User) => ({
      label: item.displayName ?? item.username,
      value: item.username,
    }))
    .filter((item: any) => item.value !== user.user.username)
})

/**
 * Drag and dropping reorder
 */
const drag_now = ref()
const drag_over = ref()

function dragStart(index: number) {
  drag_now.value = index
}

function dragOver(e: DragEvent, index: number) {
  e.preventDefault()
  drag_over.value = index
}
function dragCompare() {
  if (drag_now.value === drag_over.value) {
    drag_now.value = null
    drag_over.value = null
    return
  }

  const _temp = files.values[drag_now.value]
  files.values[drag_now.value] = files.values[drag_over.value]

  // Remove
  files.values.splice(drag_now.value, 1)

  if (drag_over.value === 0)
    files.values.unshift(_temp)
  else
    files.values.splice(drag_over.value, 0, _temp)

  nextTick(() => {
    drag_now.value = null
    drag_over.value = null
  })
}
</script>

<template>
  <div class="hi-album-upload" :class="{ 'has-drafts': drafts && !isEmpty(drafts) }">
    <LoadingSpin v-if="getLoading('album-upload')" class="dark center-page" />

    <div v-if="drafts && !isEmpty(drafts)" class="album-drafts">
      <div class="title">
        <h6>Your drafts</h6>
        <p>Manage your un-published albums</p>
      </div>

      <DraftItem v-for="item in drafts" :key="item.key" :data="item" />
    </div>

    <div class="album-upload-layout">
      <div class="album-upload-items">
        <div v-show="isLoading" class="upload-looading-bar">
          <div class="bar" :style="{ width: `${uploadPercentage}%` }" />
        </div>

        <div
          id="drop-area"
          class="album-drag-input"
          :class="{ hovering: draggingOver, empty: files.values.length === 0 }"
          @dragenter="draggingOver = true"
          @mouseleave="draggingOver = false"
        >
          <input id="draginput" name="draginput" type="file" multiple accept="image/*">
          <label for="draginput">
            <span class="material-icons">&#xe439;</span>
            <span>{{ draggingOver ? "Drop the files!" : "Cllick me / Drag files over here" }}</span>
          </label>
        </div>

        <div v-if="files.values.length > 0" class="album-upload-items-list">
          <p v-show="isLoading" class="upload-amount-indicator">
            {{ remainingProgress }} file(s) left to upload
          </p>

          <ImageUploadItem
            v-for="(item, index) in files.values"
            :key="item.name"
            :class="{ 'is-cover': item.key === album.coverKey, 'is-dragging-over': index === drag_over }"
            :data="item"
            :index="index"
            @remove="delImage"
            @set-as-cover="(key: string) => album.coverKey = key"
            @drag="dragStart(index)"
            @dragover="(e: any) => dragOver(e, index)"
            @drop="dragCompare()"
          />
        </div>
      </div>

      <div style="border-left:1px solid rgb(var(--color-border-light))">
        <div class="album-upload-metadata">
          <h3>Create album</h3>

          <InputText v-model:value="album.title" placeholder="Album name" label="Title" required :error="errors.title" />
          <InputTextarea
            v-model:value="album.description"
            placeholder="Album description"
            label="Description"
            :error="errors.description"
          />

          <h6>Event Dates</h6>
          <div class="form-date" :class="{ single: singleDate }">
            <div class="form-inputs">
              <div class="form-input">
                <input v-model="album.timeframe.from" type="date">
                <label>{{ singleDate ? "Date" : "Start" }}</label>
              </div>

              <div v-if="!singleDate" class="form-input">
                <input v-model="album.timeframe.to" type="date">
                <label>End</label>
              </div>
            </div>
            <InputCheckbox v-model:check="singleDate" label="Set date in the same day" />
          </div>

          <h6>Tagged people</h6>

          <InputSelect
            v-model:selected="taggedUsers"
            label="People"
            :options="userOptions"
            placeholder="Click to select people"
            multiple
          />

          <div v-if="taggedUsers" class="tagged-users">
            <div v-for="item in taggedUsers" :key="item" class="tagged-user" :data-title-top="user.getUsername(item)">
              <img
                class="user-image"
                :src="imageUrl(getUserImageKey(item), 'tiny')"
                :style="[`backgroundColor: rgb(${user.getUser(item, 'accentColor')})`]"
                alt=" "
                @error="(e: any) => e.target.classList.add('image-error')"
              >
            </div>
          </div>

          <InputCheckbox v-model:check="album.draft" label="Save as draft. Album will be visible only to you." />

          <Button
            v-if="albumKey"
            style="margin-top: 32px"
            class="btn-blue btn-icon"
            :to="{ name: 'AlbumDetail', params: { id: albumKey } }"
          >
            View Album
            <span class="material-icons"> &#xe941; </span>
          </Button>

          <template v-else>
            <Button
              :class="{ 'btn-disabled': files.values.length === 0 || isLoading || !album.title }"
              class="btn-icon btn-black"
              style="width: 100%; margin-bottom: 20px"
              @click="submit"
            >
              {{ album.draft ? "Save Draft" : "Save Album" }}
              <LoadingSpin v-if="isLoading" class="dark" />
            </Button>
          </template>
        </div>
      </div>
    </div>
  </div>
</template>
