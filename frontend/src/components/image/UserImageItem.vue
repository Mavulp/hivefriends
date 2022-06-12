<script setup lang="ts">
import { ref, reactive, computed } from "vue"
import { Image, imageUrl } from "../../store/album"

import InputCheckbox from "../form/InputCheckbox.vue"
import Modal from "../../components/Modal.vue"

interface Props {
  image: Image
  mode: boolean
  isSelect: boolean
}

const props = defineProps<Props>()
const emit = defineEmits<{
  (e: "select", value: Image): void
}>()
const open = ref(false)
const hover = ref(false)

function imageClick() {
  if (props.mode) {
    // Select
    emit("select", props.image)
  } else {
    open.value = true
  }
}
</script>

<template>
  <div class="hi-album-image" @mouseenter="hover = true" @mouseleave="hover = false">
    <div class="all-image-checkbox" v-if="mode">
      <InputCheckbox :check="isSelect" @update:check="emit('select', props.image)" />
    </div>

    <div class="all-image-controls" v-show="hover && !mode">
      <button data-title-left="Go to album">
        <span class="material-icons"> &#xe89e; </span>
      </button>
      <button data-title-left="Share link">
        <span class="material-icons"> &#xe80d; </span>
      </button>
      <button data-title-left="Delete">
        <span class="material-icons"> &#xe872; </span>
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
              <button data-title-left="Close">
                <span class="material-icons">&#xe5cd;</span>
              </button>
              <button data-title-left="Go to album">
                <span class="material-icons"> &#xe89e; </span>
              </button>
              <button data-title-left="Share link">
                <span class="material-icons"> &#xe80d; </span>
              </button>
              <button data-title-left="Delete">
                <span class="material-icons"> &#xe872; </span>
              </button>
            </div>
            <img :src="imageUrl(props.image.key)" alt="" />
          </div>
        </div>
      </Modal>
    </Teleport>
  </div>
</template>
