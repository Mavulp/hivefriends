<script setup lang="ts">
import InputSelect from "../../components/form/InputSelect.vue"
import InputText from "../../components/form/InputText.vue"
import InputTextarea from "../../components/form/InputTextarea.vue"
import ImageUploadItem from "../../components/upload/ImageUploadItem.vue"
import Button from "../../components/Button.vue"
import InputCheckbox from "../../components/form/InputCheckbox.vue"
import LoadingSpin from "../../components/loading/LoadingSpin.vue"

import { computed, nextTick, onBeforeMount, reactive, ref, onMounted } from "vue"
import { useRoute, useRouter } from "vue-router"
import { Album, NewAlbum, useAlbums, imageUrl, ImageFile, Image } from "../../store/album"
import { useLoading } from "../../store/loading"
import { useUser, User } from "../../store/user"
import { useFormValidation, required } from "../../js/validation"
import { clone } from "lodash"
import { upload } from "../../js/fetch"
import { useBread } from "../../store/bread"

/**
 * Setup
 */

const { addLoading, getLoading, delLoading } = useLoading()
const route = useRoute()
const router = useRouter()
const albums = useAlbums()
const user = useUser()
const bread = useBread()

const _id = computed(() => route?.params?.id.toString() ?? null)
const IS_OK = ref(false)

onBeforeMount(async () => {
  addLoading("edit")
  bread.set("Edit an album")

  const album = await albums.fetchAlbum(_id.value)
  if (album) setupForm(album)

  delLoading("edit")
})

const rawFileLength = ref(0)

const isLoading = computed(() => files.values.some((file) => file.loading))
// const uploadProgress = computed(() => `${[...files.values].filter((item) => item.key).length} / ${rawFileLength.value}`)
const remainingProgress = computed(() => rawFileLength.value - [...files.values].filter((item) => item.key).length)
const imageKeys = computed<Array<any>>(() => files.values.map((file) => file.key).filter((item) => item))

/**
 * File Handling
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
  rawFileLength.value = _files.length

  for (const file of _files) {
    if (!file) continue

    let formData = new FormData()
    formData.append("file", file)

    await uploadFile(file, formData, clone(i))

    i++
  }
}

async function uploadFile(file: any, formData: any, index: number) {
  files.values[index] = {
    name: file.name,
    size: file.size,
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

/**
 *  Format album properties into a form
 */
const draggingOver = ref(false)
const album = reactive<NewAlbum>({} as NewAlbum)
const files = reactive<ImageFile>({ values: [] })
const singleDate = ref(false)
const key = ref()

function initializeTimeframe(ms: number) {
  let date = new Date(ms)
  let year = date.getFullYear()
  let month = ("0" + (date.getMonth() + 1)).slice(-2)
  let day = ("0" + date.getDate()).slice(-2)
  return `${year}-${month}-${day}`
}

function setupForm(_album: any) {
  key.value = _album.key

  _album.timeframe.from = initializeTimeframe(_album.timeframe.from * 1000)
  _album.timeframe.to = initializeTimeframe(_album.timeframe.to * 1000)

  if (_album.timeframe.from && _album.timeframe.to) {
    singleDate.value = _album.timeframe.from === _album.timeframe.to
  }

  // Assign image files into an array
  _album.images.map((image: Image) => {
    files.values.push({
      name: image.fileName,
      size: image.sizeBytes,
      loading: false,
      key: image.key
    })
  })

  // Delete unwanted properties from the album
  delete _album.images
  delete _album.key

  Object.assign(album, _album)
}

const rules = computed(() => ({
  title: { required }
}))

const { validate, errors } = useFormValidation(album, rules)

async function submit() {
  validate().then(async () => {
    album.imageKeys = imageKeys.value

    const model = { ...album }

    Object.assign(model, {
      timeframe: {
        from: new Date(album.timeframe.from).getTime() / 1000,
        to: new Date(singleDate.value ? album.timeframe.from : album.timeframe.to).getTime() / 1000
      },
      coverKey: album.coverKey ? album.coverKey : imageKeys.value[0]
    })

    albums.editAlbum(key.value, model)

    IS_OK.value = true
  })
}

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

/**
 * File upload
 */

function delImage(index: number) {
  if (files.values[index]) {
    files.values.splice(index, 1)
  }
}

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
  let _temp = files.values[drag_now.value]
  files.values[drag_now.value] = files.values[drag_over.value]
  files.values[drag_over.value] = _temp

  nextTick(() => {
    drag_now.value = null
    drag_over.value = null
  })
}

/**
 * Delete album
 */

async function deleteAlbum() {
  await albums.deleteAlbum(_id.value)
  router.push({ name: "UserAlbums", params: { user: user.user.username } })
}
</script>

<template>
  <div class="hi-album-upload album-edit">
    <LoadingSpin v-if="getLoading('edit')" class="center-page dark" />

    <!-- <template v-else-if="false"> no data </template> -->
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
          <p class="upload-amount-indicator" v-if="remainingProgress > 0">
            {{ remainingProgress }} file(s) left to upload
          </p>

          <ImageUploadItem
            v-for="(item, index) in files.values"
            :class="{ 'is-cover': item.key === album.coverKey, 'is-dragging-over': index === drag_over }"
            :data="item"
            :key="item.name"
            :index="index"
            @remove="delImage"
            @setAsCover="(key: string) => album.coverKey = key"
            @drag="dragStart(index)"
            @dragover="(e) => dragOver(e, index)"
            @drop="dragCompare()"
          />
        </div>
      </div>

      <div class="album-upload-metadata" v-if="!getLoading('edit')">
        <div class="title-wrap">
          <h3>Edit album</h3>
          <button @click="deleteAlbum" class="hover-bubble bubble-red">
            Delete album
            <span class="material-icons rotate" v-if="getLoading('delete-albums')">&#xe863;</span>
          </button>
        </div>

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

        <InputSelect
          label="People"
          :options="userOptions"
          v-model:selected="album.taggedUsers"
          placeholder="Click to select people"
          multiple
        />

        <div class="tagged-users" v-if="album.taggedUsers">
          <div
            class="tagged-user"
            v-for="item in album.taggedUsers"
            :data-title-top="user.getUsername(item)"
            :key="item"
          >
            <img
              class="user-image"
              :src="imageUrl(getUserImageKey(item), 'tiny')"
              :style="[`backgroundColor: rgb(${user.getUser(item, 'accentColor')})`]"
              alt=" "
              @error="(e: any) => e.target.classList.add('image-error')"
            />
          </div>
        </div>

        <InputCheckbox v-model:check="album.draft" label="Save as a draft. It won't be published" />

        <div class="buttons" style="padding-top: 16px">
          <Button
            :class="{ 'btn-disabled': files.values.length === 0 || isLoading || !album.title }"
            class="btn-icon btn-black"
            @click="submit"
          >
            {{ album.draft ? "Save Draft" : "Publish Album" }}
            <LoadingSpin class="dark" v-if="isLoading" />
          </Button>

          <Button class="btn-blue btn-icon" :to="{ name: 'AlbumDetail', params: { id: key } }">
            View Album
            <!-- <span class="material-icons"> &#xe941; </span> -->
          </Button>
        </div>
      </div>
    </div>
  </div>
</template>
