<script setup lang="ts">
import { ref } from "vue"
import { Image, imageUrl } from "../../store/album"
import { useClipboard } from "@vueuse/core"

import InputCheckbox from "../form/InputCheckbox.vue"
import Modal from "../../components/Modal.vue"
import { useToast } from "../../store/toast"
import { useUser } from "../../store/user"
import { useLoading } from "../../store/loading"

interface Props {
  image: Image
  mode: boolean
  isSelect: boolean
}

const toast = useToast()
const user = useUser()
const props = defineProps<Props>()
const emit = defineEmits<{
  (e: "select", value: Image): void
}>()
const open = ref(false)
const hover = ref(false)
const { copy } = useClipboard()

function imageClick() {
  if (props.mode) {
    // Select
    emit("select", props.image)
  } else {
    open.value = true
  }
}

function copyImage() {
  copy(imageUrl(props.image.key))
  toast.add("Image url copied to clipboard")
}

/**
 * Set as avatar / banner
 */
const loading = ref(false)

async function setAs(key: string) {
  loading.value = true
  await user.setSetting(key, props.image.key)
  loading.value = false

  toast.add(`Updated ${key === "avatarKey" ? "avatar" : "banner"} image`)
}
</script>

<template>
  <div
    class="hi-album-image"
    @mouseenter="hover = true"
    @mouseleave="hover = false"
    :class="{ 'is-selected': props.isSelect }"
  >
    <div class="all-image-checkbox" v-if="mode">
      <InputCheckbox :check="props.isSelect" @update:check="emit('select', props.image)" />
    </div>

    <div class="all-image-controls" v-show="hover && !mode">
      <button data-title-left="Go to album">
        <span class="material-icons"> &#xe89e; </span>
      </button>
      <button data-title-left="Share link" @click="copyImage">
        <span class="material-icons"> &#xe80d; </span>
      </button>
      <button data-title-left="Use as avatar" @click="setAs('avatarKey')" :class="{ 'btn-disabled': loading }">
        <span class="material-icons"> &#xe853; </span>
      </button>
      <button data-title-left="Use as banner" @click="setAs('bannerKey')" :class="{ 'btn-disabled': loading }">
        <span class="material-icons"> &#xe40b; </span>
      </button>
    </div>

    <div class="image-wrap" @click="imageClick">
      <img :src="imageUrl(props.image.key, 'medium')" alt="" />
    </div>

    <Teleport to="body" v-if="open">
      <Modal @click="open = false">
        <div class="modal-wrap modal-image">
          <div>
            <div class="all-image-controls">
              <button data-title-left="Close" @click="open = false">
                <span class="material-icons">&#xe5cd;</span>
              </button>
              <button data-title-left="Go to album">
                <span class="material-icons"> &#xe89e; </span>
              </button>
              <button data-title-left="Share link" @click="copyImage">
                <span class="material-icons"> &#xe80d; </span>
              </button>
              <button data-title-left="Use as avatar" @click="setAs('avatarKey')" :class="{ 'btn-disabled': loading }">
                <span class="material-icons"> &#xe853; </span>
              </button>
              <button data-title-left="Use as banner" @click="setAs('bannerKey')" :class="{ 'btn-disabled': loading }">
                <span class="material-icons"> &#xe40b; </span>
              </button>
            </div>
            <img :src="imageUrl(props.image.key)" alt="" />
          </div>
        </div>
      </Modal>
    </Teleport>
  </div>
</template>
