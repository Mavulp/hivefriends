<script setup lang="ts">
import InputSelect from "../../components/form/InputSelect.vue"
import InputText from "../../components/form/InputText.vue"
import InputTextarea from "../../components/form/InputTextarea.vue"
import ImageUploadItem from "../../components/upload/ImageUploadItem.vue"
import Button from "../../components/Button.vue"
import LoadingSpin from "../../components/loading/LoadingSpin.vue"
import InputCheckbox from "../../components/form/InputCheckbox.vue"

import { onBeforeUnmount, onMounted, reactive, ref, computed, onBeforeMount } from "vue"
import { upload } from "../../js/fetch"
import { useFormValidation, required } from "../../js/validation"
import { useAlbums, NewAlbum, imageUrl } from "../../store/album"
import { clone } from "lodash"
import { useUser, User } from "../../store/user"
import { useLoading } from "../../store/loading"

const store = useAlbums()
const user = useUser()
const { addLoading, delLoading, getLoading } = useLoading()

/**
 * Interface & setup
 */

interface File {
  values: Array<{
    name: string
    type: string
    size: number
    loading: boolean
    error?: string
    key: string | null
  }>
}

const files = reactive<File>({ values: [] })
const album = reactive<NewAlbum>({
  title: "",
  description: "",
  locations: [],
  timeframe: {
    from: 0,
    to: 0
  },
  imageKeys: [],
  taggedUsers: [],
  coverKey: ""
})

// const album = reactive<NewAlbum>({} as NewAlbum)

// If album was successfuly generated, this will get populated
const albumKey = ref()

// TODO: Compute global loading progress by
// checking loading states of all files and add a loading bar / percentagle somewhere in the upload
// TODO: add multiple locations

const draggingOver = ref(false)
const singleDate = ref(false)

const isLoading = computed(() => files.values.some((file) => file.loading))
// const loadingProgress = computed(() => [...files.values].filter((item) => !item.key).length)
const imageKeys = computed<Array<any>>(() => files.values.map((file) => file.key))

/**
 * Lifecycle
 */

onMounted(() => {
  const el = document.getElementById("drop-area")

  if (el) {
    el.addEventListener("dragenter", onSubmitHandler, false)
    el.addEventListener("dragleave", onSubmitHandler, false)
    el.addEventListener("dragover", onSubmitHandler, false)
    el.addEventListener("drop", onSubmitHandler, false)
    el.addEventListener("input", (e) => onSubmitHandler(e, true), false)
  }
})

onBeforeMount(async () => {
  addLoading("users")
  await user.fetchUsers()
  delLoading("users")
})

/**
 * File Handling
 */

function onSubmitHandler(e: any, fromField: boolean = false) {
  e.preventDefault()
  e.stopPropagation()

  let files = fromField ? e.target.files : e.dataTransfer.files

  if (files.length > 0) {
    uploadFiles(files)
  }
}

async function uploadFiles(_files: any) {
  let i = files.values.length

  for (const file of _files) {
    if (!file) continue

    let formData = new FormData()
    formData.append("file", file)

    uploadFile(file, formData, clone(i))

    i++
  }
}

async function uploadFile(file: any, formData: any, index: number) {
  files.values[index] = {
    name: file.name,
    size: file.size,
    type: file.type,
    loading: true,
    key: null
  }

  return upload("/api/images/", formData)
    .then((response: any) => {
      Object.assign(files.values[index], {
        loading: false,
        key: response.key
      })
    })
    .catch((error) => {
      Object.assign(files.values[index], {
        loading: false,
        error
      })
    })
}

function delImage(index: number) {
  if (files.values[index]) {
    files.values.splice(index, 1)
  }
}

const rules = computed(() => ({
  title: { required }
}))

const { validate, errors } = useFormValidation(album, rules)

async function submit() {
  validate().then(async () => {
    // Iterate over all active images, add them to imageKeys array
    album.imageKeys = imageKeys.value

    const model = { ...album }

    Object.assign(model, {
      locations: album.locations?.join(","),
      timeframe: {
        from: new Date(album.timeframe.from).getTime() / 1000,
        to: new Date(singleDate.value ? album.timeframe.from : album.timeframe.to).getTime() / 1000
      },
      // TODO: implement
      // coverKey: album.coverKey ?? files.values[0].key
      coverKey: album.coverKey ?? files.values[0].key,
      taggedUsers: taggedUsers.value
    })

    const { key } = await store.addAlbum(model)

    if (key) {
      albumKey.value = key
    }
  })
}

/**
 * Tagging users
 */

const taggedUsers = ref([])
// const users

function getUserImageKey(name: string): string {
  return user.users.find((item) => item.username === name)?.avatarKey ?? ""
}

const userOptions = computed(() => {
  if (!user.users || user.users.length === 0) return null

  return user.users.map((item: User) => ({
    label: item.displayName ?? item.username,
    value: item.username
  }))
})
</script>

<template>
  <div class="hi-album-upload">
    <div class="album-upload-layout">
      <div class="album-upload-items">
        <div
          class="album-drag-input"
          id="drop-area"
          @dragenter="draggingOver = true"
          @mouseleave="draggingOver = false"
          :class="{ hovering: draggingOver, empty: files.values.length === 0 }"
        >
          <input id="draginput" name="draginput" type="file" multiple accept="image/*" />
          <label for="draginput">
            <span class="material-icons">&#xe439;</span>
            <span>{{ draggingOver ? "Drop the files!" : "Cllick me / Drag files over here" }}</span>
          </label>
        </div>

        <div class="album-upload-items-list">
          <template v-if="files.values.length > 0">
            <ImageUploadItem
              v-for="(item, index) in [...files.values].reverse()"
              :class="{ 'is-cover': item.key === album.coverKey }"
              :data="item"
              :key="item.name"
              :index="index"
              @remove="delImage"
              @setAsCover="(key: string) => album.coverKey = key"
            />
          </template>
        </div>
      </div>

      <div class="album-upload-metadata">
        <h3>Create an album</h3>

        <InputText v-model:value="album.title" placeholder="Album name" label="Title*" :error="errors.title" />
        <InputTextarea v-model:value="album.description" placeholder="Album description" label="Description" />

        <h6>Event Dates</h6>
        <div class="form-date" :class="{ single: singleDate }">
          <div class="form-inputs">
            <div class="form-input">
              <input type="date" v-model="album.timeframe.from" />
              <label>{{ singleDate ? "Date" : "Start" }}</label>
            </div>

            <div class="form-input" v-if="!singleDate">
              <input type="date" v-model="album.timeframe.to" />
              <label>End</label>
            </div>
          </div>
          <InputCheckbox v-model:check="singleDate" label="Set date in the same day" />
        </div>

        <h6>Tagged people</h6>

        <p style="margin-bottom: 24px">Select people who you wish to tag in this album</p>

        <span v-if="getLoading('users')"> loading </span>

        <InputSelect
          v-else
          label="People"
          :options="userOptions"
          v-model:selected="taggedUsers"
          placeholder="Click to select people"
          multiple
        />

        <div class="tagged-users" v-if="taggedUsers">
          <div class="tagged-user" v-for="item in taggedUsers" :data-title-top="user.getUsername(item)" :key="item">
            <img
              :src="imageUrl(getUserImageKey(item), 'tiny')"
              :style="[`backgroundColor: rgb(${user.getUser(item, 'accentColor')})`]"
            />
          </div>
          <!-- List users with their profile pictures, name in a tooltip -->
        </div>

        <Button
          :class="{ 'btn-disabled': files.values.length === 0 || isLoading }"
          class="btn-icon btn-black"
          style="width: 100%"
          @click="submit"
        >
          Upload
          <LoadingSpin class="dark" v-if="isLoading" />
        </Button>

        <Button class="btn-blue mt-20 btn-icon" v-if="albumKey" :to="{ name: 'AlbumDetail', params: { id: albumKey } }">
          View Album
          <span class="material-icons"> &#xe941; </span>
        </Button>
      </div>
    </div>
  </div>
</template>
