<script setup lang="ts">
import { ref, reactive, computed, onBeforeMount, onMounted } from "vue"
import { getImageChunks } from "../../js/_composables"
import { useBread } from "../../store/bread"

// const open = ref(false)
const bread = useBread()

onBeforeMount(() => {
  bread.set("All images list")
})

const chunks = computed(() =>
  getImageChunks([
    // {
    //   key: "TsQTZopMQYu5huvkuVn1pw",
    //   fileName: "black-mesa.jpg",
    //   sizeBytes: 440410,
    //   uploader: "Jokler",
    //   uploadedAt: 1654906142
    // },
    // {
    //   key: "8vrqbq-JRFSxgIdsBwTUJQ",
    //   fileName: "asteroid-station.png",
    //   sizeBytes: 10145583,
    //   uploader: "Jokler",
    //   uploadedAt: 1654907150
    // },
    // {
    //   key: "FvK9jmF-SXWyiB2QyaT1jQ",
    //   fileName: "elisabeth-nagurnaya-necromancer.jpg",
    //   sizeBytes: 2183323,
    //   uploader: "Jokler",
    //   uploadedAt: 1654907266
    // },
    // {
    //   key: "pf5txMllQaiJ-FtZNeC0Pg",
    //   fileName: "artur-sadlos-to-sh300-ooh-as-05i.jpg",
    //   sizeBytes: 714894,
    //   uploader: "Jokler",
    //   uploadedAt: 1654907330
    // },
    // {
    //   key: "RP2riPdlQteYrVt1FTkSmQ",
    //   fileName: "cloudy_days.png",
    //   sizeBytes: 2555734,
    //   uploader: "Jokler",
    //   uploadedAt: 1654907341
    // }
  ])
)

// onMounted(() => {
//     const el = document.getElementById("drop-area")

//   if (el) {
//     el.addEventListener("dragenter", onSubmitHandler, false)
//     el.addEventListener("dragleave", onSubmitHandler, false)
//     el.addEventListener("dragover", onSubmitHandler, false)
//     el.addEventListener("drop", onSubmitHandler, false)
//     el.addEventListener("input", (e) => onSubmitHandler(e, true), false)
//   }
// })

// function onSubmitHandler(e: any, fromField: boolean = false) {
//   e.preventDefault()
//   e.stopPropagation()

//   let files = fromField ? e.target.files : e.dataTransfer.files

//   if (files.length > 0) {
//     uploadFiles(files)
//   }
// }

// async function uploadFiles(_files: any) {
//   let i = files.values.length
//   rawFileLength.value = _files.length + imageKeys.value.length

//   for (const file of _files) {
//     if (!file) continue

//     let formData = new FormData()
//     formData.append("file", file)

//     await uploadFile(file, formData, clone(i))

//     i++
//   }
// }

// async function uploadFile(file: any, formData: any, index: number) {
//   files.values[index] = {
//     name: file.name,
//     size: file.size,
//     loading: true,
//     key: null
//   }

//   return upload("/api/images/", formData)
//     .then((response: any) => {
//       Object.assign(files.values[index], {
//         loading: false,
//         key: response.key
//       })
//     })
//     .catch((error) => {
//       Object.assign(files.values[index], {
//         loading: false,
//         error
//       })
//     })
// }
</script>

<template>
  <div class="hi-image-list">
    <div class="hi-image-list-upload">
      <h1>Upload Images</h1>
      <p>
        This is not the recommended way to use the site, but if you want to upload singular images not tied to an album,
        you can do so here. The main use of this should be to store and share standalone images.
      </p>

      <!-- <div
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
        </div> -->
    </div>

    <div class="hi-album-images">
      <div class="hi-album-image-col" v-for="chunk in chunks" :key="chunk.length">
        <!-- <ImageListitem v-for="image in chunk" :key="image.key" :image="image" :album-key="album.key" /> -->

        <!-- <img src="" alt=""> -->
        <pre>
          {{ chunk }}
        </pre>
      </div>
    </div>
  </div>
</template>
