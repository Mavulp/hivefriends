<script setup lang="ts">
import Button from "../Button.vue"
import LoadingSpin from "../loading/LoadingSpin.vue"
import InputCheckbox from "../form/InputCheckbox.vue"
import InputSelect from "../form/InputSelect.vue"
import InputText from "../form/InputText.vue"
import InputTextarea from "../form/InputTextarea.vue"
import UploadSettingsImage from "./modules/UploadSettingsImage.vue"

import { computed, onBeforeMount, reactive } from "vue"
import { useLoading } from "../../store/loading"
import { useUser } from "../../store/user"
import { useFormValidation, minLength, required, sameAs } from "../../js/validation"

const { getLoading, addLoading, delLoading } = useLoading()
const user = useUser()

onBeforeMount(() => {
  user.fetchSettings()
})

const themeOptions = [
  { label: "Light Theme", value: "light-theme" },
  // { label: "Dark Theme; High Contrast", value: "dark-contrast" },
  { label: "Dark Theme", value: "dark-normal" }
]

const _private = computed<boolean>({
  get: () => user.settings.private,
  set: (value: boolean) => user.setSetting("private", value)
})

const _theme = computed<string>({
  get: () => user.settings.colorTheme ?? "light-theme",
  set(value: string) {
    user.setSetting("colorTheme", value)
    const r = document.querySelector(":root")
    if (r) {
      r.removeAttribute("class")
      r.classList.add(value)
    }
  }
})

/**
 * User information validation
 */
const userForm = reactive({
  displayName: user.user.displayName,
  bio: user.user.bio
})

const infoRules = computed(() => ({
  displayName: { minLength: minLength(3) }
}))

// const { errors, validate, status } = useFormValidation(userForm, rules, { autoclear: true })
const infoValidation = useFormValidation(userForm, infoRules, { autoclear: true })

async function submitUserInfo() {
  addLoading("user-form")

  infoValidation
    .validate()
    .then(() => {
      user.setSetting("displayName", userForm.displayName)
      user.setSetting("bio", userForm.bio)
    })
    .finally(() => {
      delLoading("user-form")
    })
}

/**
 * User password validation
 */

const passForm = reactive({
  old: "",
  new1: "",
  new2: ""
})

const passIsEmpty = computed(
  () => passForm.old.length === 0 || passForm.new1.length === 0 || passForm.new2.length === 0
)

const passRules = computed(() => ({
  old: {
    required
  },
  new1: {
    required,
    minLenght: minLength(8)
  },
  new2: {
    required,
    sameAs: sameAs(passForm.new1)
  }
}))

const passValidation = useFormValidation(passForm, passRules, { autoclear: true })

async function savePassword() {
  addLoading("password")

  passValidation
    .validate()
    .then(() => {
      user.changePassword({
        old: passForm.old,
        new: passForm.new2
      })
    })
    .finally(() => {
      delLoading("password")
    })
}
</script>

<template>
  <div class="hi-user-page">
    <LoadingSpin class="center-page" v-if="getLoading('settings')" />

    <div class="hi-user-settings-items">
      <h1>Settings</h1>

      <ul class="user-settings-list">
        <li>
          <h5>Your info</h5>
          <form @submit.prevent="submitUserInfo">
            <input id="username" style="display: none" type="text" name="fakeusernameremembered" />
            <input id="password" style="display: none" type="password" name="fakepasswordremembered" />

            <InputText
              v-model:value="userForm.displayName"
              placeholder="Your display name"
              label="Display name"
              :error="infoValidation.errors.displayName"
            />
            <InputTextarea
              label="Bio"
              v-model:value="userForm.bio"
              placeholder="Something about you. It can be anything"
              :value="user.user.bio"
            />
            <Button
              class="btn-black"
              type="submit"
              :class="{ 'btn-disabled': getLoading('user-form') || infoValidation.status.anyError }"
              >Save</Button
            >
          </form>
        </li>
        <li>
          <h5>Avatar image</h5>
          <p>Will be used on your profile or if someone tags you in an album / photo</p>
          <UploadSettingsImage field="avatarKey" key="one" />
        </li>
        <li>
          <h5>Banner image</h5>
          <p>Visible on your profile</p>
          <UploadSettingsImage field="bannerKey" key="two" />
        </li>

        <li>
          <h5>Application</h5>
          <!-- <InputCheckbox
            label="Set yourself as private. You won't be mentioned in albums & your albums won't be publicly available"
            v-model:check="_private"
          /> -->
        </li>
        <li>
          <InputSelect
            style="width: 356px"
            v-model:selected="_theme"
            :options="themeOptions"
            label="Theme"
            placeholder="Select a theme"
            canclear
          />
        </li>
        <li>
          <h5>Password</h5>

          <form @submit.prevent="savePassword">
            <InputText
              :error="passValidation.errors.old"
              v-model:value="passForm.old"
              label="Old password"
              placeholder="Your current password"
            />
            <InputText
              :error="passValidation.errors.new1"
              v-model:value="passForm.new1"
              label="New password"
              placeholder="Your new password"
            />
            <InputText
              :error="passValidation.errors.new2"
              v-model:value="passForm.new2"
              label="Confirm password"
              placeholder="Confirm your new password"
            />
            <Button
              class="btn-black"
              type="submit"
              :class="{ 'btn-disabled': getLoading('password') || passValidation.status.anyError || passIsEmpty }"
              >Save</Button
            >
          </form>
        </li>
      </ul>
    </div>
  </div>
</template>
