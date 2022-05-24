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
  timeframe: {
    from: 0,
    to: 0
  },
  imageKeys: [],
  taggedUsers: [],
  coverKey: ""
})

// If album was successfuly generated, this will get populated
const albumKey = ref()

const draggingOver = ref(false)
const singleDate = ref(false)

const isLoading = computed(() => files.values.some((file) => file.loading))
// const loadingPercent = computed(() =>
//   Math.ceil(([...files.values].filter((item) => !item.key).length / files.values.length) * 100)
// )
const uploadProgress = computed(() => `${[...files.values].filter((item) => item.key).length} / ${files.values.length}`)
const imageKeys = computed<Array<any>>(() => files.values.map((file) => file.key).filter((item) => item))

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

  taggedUsers.value.push(user.getUsername())
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
    album.imageKeys = imageKeys.value

    const model = { ...album }

    // Assign properties which have to be formatted in some way or are not part of the origina form object
    Object.assign(model, {
      timeframe: {
        from: new Date(album.timeframe.from).getTime() / 1000,
        to: new Date(singleDate.value ? album.timeframe.from : album.timeframe.to).getTime() / 1000
      },
      coverKey: album.coverKey ? album.coverKey : imageKeys.value[0],
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

const taggedUsers = ref<Array<string>>([])

function getUserImageKey(name: string): string {
  return user.users.find((item) => item.username === name)?.avatarKey ?? ""
}

const userOptions = computed(() => {
  if (!user.users || user.users.length === 0) return null

  // Make sure you can't select yourself
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

        <div class="album-upload-items-list" v-if="files.values.length > 0">
          <ImageUploadItem
            v-for="(item, index) in files.values"
            :class="{ 'is-cover': item.key === album.coverKey }"
            :data="item"
            :key="item.name"
            :index="index"
            @remove="delImage"
            @setAsCover="(key: string) => album.coverKey = key"
          />
        </div>
      </div>

      <div class="album-upload-metadata">
        <h3>Create an album</h3>

        <InputText v-model:value="album.title" placeholder="Album name" label="Title" required :error="errors.title" />
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

        <span class="loading" v-if="getLoading('users')"> Loading... </span>

        <template v-else>
          <InputSelect
            label="People"
            :options="userOptions"
            v-model:selected="taggedUsers"
            placeholder="Click to select people"
            multiple
          />

          <div class="tagged-users" v-if="taggedUsers">
            <div class="tagged-user" v-for="item in taggedUsers" :data-title-top="user.getUsername(item)" :key="item">
              <img
                class="user-image"
                :src="imageUrl(getUserImageKey(item), 'tiny')"
                :style="[`backgroundColor: rgb(${user.getUser(item, 'accentColor')})`]"
                alt=" "
                @error="(e: any) => e.target.classList.add('image-error')"
              />
            </div>
          </div>
        </template>

        <Button class="btn-blue mt-20 btn-icon" v-if="albumKey" :to="{ name: 'AlbumDetail', params: { id: albumKey } }">
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
            Save Album
            <LoadingSpin class="dark" v-if="isLoading" />
          </Button>

          <p v-if="isLoading">{{ uploadProgress }} photos uploaded</p>
        </template>
      </div>
    </div>
  </div>
</template>
