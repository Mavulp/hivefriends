<script setup lang="ts">
import { computed, onBeforeMount, onMounted, reactive, ref } from 'vue'
import InputTextarea from '../form/InputTextarea.vue'
import InputText from '../form/InputText.vue'
import InputSelect from '../form/InputSelect.vue'
import LoadingSpin from '../loading/LoadingSpin.vue'
import Button from '../Button.vue'
import { useLoading } from '../../store/loading'
import { useUser } from '../../store/user'
import { minLength, required, sameAs, useFormValidation } from '../../js/validation'
import { HEX_to_RGB, TEXT_CONTRAST } from '../../js/utils'
import { useToast } from '../../store/toast'
import countries from '../../js/countries'
import { useBread } from '../../store/bread'
import { useThresholdScroll } from '../../js/_composables'
import UploadSettingsImage from './modules/UploadSettingsImage.vue'

const { getLoading, addLoading, delLoading } = useLoading()
const bread = useBread()
const toast = useToast()
const user = useUser()

const themeOptions = [
  { label: 'Light Theme', value: 'light-theme' },
  // { label: "Dark Theme; High Contrast", value: "dark-contrast" },
  { label: 'Dark Theme', value: 'dark-normal' },
]

const _theme = computed<string>({
  get: () => user.settings.colorTheme ?? 'light-theme',
  set: (value: string) => {
    user.setSetting('colorTheme', value)
    const r = document.querySelector(':root')
    if (r) {
      r.removeAttribute('class')
      r.classList.add(value)
    }
  },
})

const _accent = computed<string>({
  get: () => user.settings.accentColor ?? '229,125,35',
  set: (value: string) => {
    const [r, g, b] = HEX_to_RGB(value)

    user.setSetting('accentColor', `${r},${g},${b}`)
    document.documentElement.style.setProperty('--color-highlight', `${r},${g},${b}`)
  },
})

const buttonColor = computed(() => {
  const [r, g, b] = _accent.value.split(',')
  return TEXT_CONTRAST(Number(r), Number(g), Number(b))
})
/**
 * User information validation
 */
const userForm = reactive({
  displayName: '',
  bio: '',
  country: user.user.country,
})

onBeforeMount(async () => {
  await user.fetchSettings()

  userForm.displayName = user.getUsername(user.user.username)
  userForm.bio = user.user.bio

  bread.set('Your settings')
})

const infoRules = computed(() => ({
  displayName: { minLength: minLength(3) },
}))

// const { errors, validate, status } = useFormValidation(userForm, rules, { autoclear: true })
const infoValidation = useFormValidation(userForm, infoRules, { autoclear: true })

async function submitUserInfo() {
  addLoading('user-form')

  infoValidation
    .validate()
    .then(() => {
      Promise.all([
        user.setSetting('displayName', userForm.displayName),
        user.setSetting('bio', userForm.bio),
        user.setSetting('country', userForm.country),
      ])
        .then(() => toast.add('Successfully updated user information', 'success'))
        .catch(() => toast.add('Error updating user information', 'error'))
    })
    .finally(() => {
      delLoading('user-form')
    })
}

const countryOptions = computed(() => {
  return Object.entries(countries).map(([code, country]) => {
    return {
      value: code,
      label: country.name,
    }
  })
})

/**
 * User password validation
 */

const passForm = reactive({
  old: '',
  new1: '',
  new2: '',
})

const passIsEmpty = computed(
  () => passForm.old.length === 0 || passForm.new1.length === 0 || passForm.new2.length === 0,
)

const passRules = computed(() => ({
  old: {
    required,
  },
  new1: {
    required,
    minLenght: minLength(8),
  },
  new2: {
    required,
    sameAs: sameAs(passForm.new1),
  },
}))

const passValidation = useFormValidation(passForm, passRules, { autoclear: true })

async function savePassword() {
  addLoading('password')

  passValidation
    .validate()
    .then(() => {
      user.changePassword({
        old: passForm.old,
        new: passForm.new2,
      })
    })
    .finally(() => {
      delLoading('password')
    })
}

/**
 * Scrolling
 */

function scrollTo(selector: string) {
  const el = document.querySelector<HTMLElement>(selector)

  if (el && el.parentElement) {
    window.scrollTo({
      top: el.parentElement.offsetTop - 64,
      behavior: 'smooth',
    })
  }
}

const active = ref<string>('info')
onMounted(() => {
  const el = document.querySelectorAll<HTMLElement>('h5[id]')

  if (el) {
    window.addEventListener('scroll', () => {
      el.forEach((section) => {
        const sectionTop = section.parentElement?.offsetTop ?? 0
        const sectionHeight = section.parentElement?.clientHeight ?? 0

        if (window.scrollY >= sectionTop - sectionHeight / 2) {
          const id = section.getAttribute('id')

          if (id)
            active.value = id
        }
      })
    })
  }
})

const { passed, scroll } = useThresholdScroll(292)
</script>

<template>
  <div class="hi-settings-page">
    <LoadingSpin v-if="getLoading('settings')" class="center-page" />

    <div>
      <div class="settings-nav">
        <button :class="{ active: active === 'info' }" class="nav-item hover-bubble" @click="scrollTo('#info')">
          User Information
        </button>
        <button :class="{ active: active === 'avatars' }" class="nav-item hover-bubble" @click="scrollTo('#avatars')">
          Avatar & Banner
        </button>
        <button :class="{ active: active === 'visuals' }" class="nav-item hover-bubble" @click="scrollTo('#visuals')">
          Visuals
        </button>
        <button :class="{ active: active === 'passwords' }" class="nav-item hover-bubble" @click="scrollTo('#passwords')">
          Password
        </button>

        <div class="gap" />

        <button :class="{ visible: passed }" class="hover-bubble go-up" data-title-bottom="Scroll Up" @click="scroll">
          <span class="material-icons"> &#xe5d8; </span>
        </button>
      </div>
    </div>

    <div class="hi-user-settings-items">
      <div class="title-wrap">
        <h1>Settings</h1>
        <button class="button btn-black size-normal">
          <span class="material-icons">&#xe9ba;</span>
          Log out
        </button>
      </div>

      <ul class="user-settings-list">
        <li>
          <h5 id="info">
            Your info
          </h5>
          <!-- <form @submit.prevent="submitUserInfo"> -->
          <input id="username" style="display: none" type="text" name="fakeusernameremembered">
          <input id="password" style="display: none" type="password" name="fakepasswordremembered">

          <InputText
            v-model:value="userForm.displayName"
            placeholder="Your display name"
            label="Display name"
            :error="infoValidation.errors.displayName"
          />

          <InputTextarea
            v-model:value="userForm.bio"
            label="Bio"
            placeholder="Something about you. It can be anything"
          />

          <InputSelect
            v-model:selected="userForm.country"
            label="Country"
            placeholder="Select where you're from"
            :options="countryOptions"
          />

          <Button
            class="btn-black"
            :class="{ 'btn-disabled': getLoading('user-form') || infoValidation.status.anyError }"
            @click="submitUserInfo"
          >
            Save
          </Button>
          <!-- </form> -->
        </li>
        <li>
          <h5 id="avatars">
            Avatar image
          </h5>
          <p>Will be used on your profile or if someone tags you in an album / photo</p>
          <UploadSettingsImage key="one" field="avatarKey" />
        </li>
        <li>
          <h5>Banner image</h5>
          <p>Visible on your profile</p>
          <UploadSettingsImage key="two" field="bannerKey" />
        </li>
        <li>
          <h5 id="visuals">
            Color theme
          </h5>
          <InputSelect
            v-model:selected="_theme"
            :options="themeOptions"
            label="Theme"
            placeholder="Select a theme"
            cantclear
          />

          <h5>Accent Color</h5>
          <p>Personalize your profile and site by changing the accent color</p>

          <div class="accent-color">
            <input
              id="color"
              :value="_accent"
              type="color"
              name="color"
              @change="(e: any) => _accent = e.target.value"
            >
            <label for="color" :style="{ color: buttonColor }"> Change</label>

            <div class="divider" />

            <div class="example-block" />
            <div class="example-block text color">
              <span class="material-icons">&#xe40a;</span>
            </div>
            <div class="example-block text">
              Example
            </div>
            <div class="example-block text color">
              Example
            </div>
          </div>
        </li>
        <li>
          <h5 id="passwords">
            Password
          </h5>

          <form @submit.prevent="savePassword">
            <input id="username" style="display: none" type="text" name="fakeusernameremembered">
            <input id="password" style="display: none" type="password" name="fakepasswordremembered">
            <InputText
              v-model:value="passForm.old"
              type="password"
              :error="passValidation.errors.old"
              label="Old password"
              placeholder="Your current password"
            />
            <InputText
              v-model:value="passForm.new1"
              type="password"
              :error="passValidation.errors.new1"
              label="New password"
              placeholder="Your new password"
            />
            <InputText
              v-model:value="passForm.new2"
              type="password"
              :error="passValidation.errors.new2"
              label="Confirm password"
              placeholder="Confirm your new password"
            />
            <Button
              class="btn-black"
              type="submit"
              :class="{ 'btn-disabled': getLoading('password') || passValidation.status.anyError || passIsEmpty }"
            >
              Save
            </Button>
          </form>
        </li>
      </ul>
    </div>
  </div>
</template>
