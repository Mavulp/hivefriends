<script setup lang="ts">
import InputText from "../../components/form/InputText.vue"
import Button from "../../components/Button.vue"
import LoadingSpin from "../../components/loading/LoadingSpin.vue"

import { ref, reactive, computed, nextTick, onBeforeMount } from "vue"
import { getRanMinMax } from "../../js/utils"
import { useUser } from "../../store/user"
import { useFormValidation, required, minLength, asyncValidation } from "../../js/validation"
import { useRouter } from "vue-router"
import { useBread } from "../../store/bread"
import { useLoading } from "../../store/loading"

const router = useRouter()
const bread = useBread()
const auth = useUser()
const { getLoading } = useLoading()

type SignInForm = {
  username: string
  password: string
}

const form = reactive<SignInForm>({ username: "", password: "" })

const placeholders = [
  "dolanspaghetti",
  "tmtupomegranate",
  "apjriehat",
  "stormtrooper11",
  "clobatenjoyer",
  "quoteshater",
  "zealseal",
  "horse smeller",
  "horselinman",
  "cheessyman",
  "_your_name_ (intense)",
  "Slifblinger",
  "gaming bed",
  "sergeiy",
  "checkmate"
]
const placeholder = ref(placeholders[getRanMinMax(0, placeholders.length - 1)])

async function submit() {
  validate()
    .then(async () => {
      // Submit
      if (form.username && form.password) {
        await auth.signIn(form)
        reset()

        nextTick(() => {
          router.push({ name: "Home" })
        })
      }
    })
    .catch((errors) => {
      // console.log("test", errors)
    })
}

const rules = computed(() => ({
  username: {
    required
  },
  password: {
    required,
    minLength: minLength(3)
  }
}))

// Setup validation
const { errors, validate, reset } = useFormValidation(form, rules, { autoclear: true })

onBeforeMount(async () => {
  const r = document.querySelector(":root")
  if (r) {
    r.classList.remove("dark-normal")
  }

  const token = localStorage.getItem("bearer_token")
  const key = localStorage.getItem("user")

  if (token && key) {
    await auth.fetchUser(key)
    router.push({ name: "Home" })
  } else {
    bread.set("Welcome to hi!friends")
  }
})
</script>

<template>
  <div class="route-login">
    <div class="route-login-split has-block">
      <div class="block">Bunch of friends all around the world cherishing moments spent together.</div>
    </div>

    <div class="route-login-split has-form">
      <form @submit.prevent="submit" class="form-wrap">
        <div class="loading-overlay" v-if="getLoading('login')">
          <LoadingSpin class="dark center-parent" />
        </div>

        <img src="/Sharp2.png" alt=" " />
        <InputText :error="errors.username" v-model:value="form.username" label="Username" :placeholder="placeholder" />
        <InputText
          :error="errors.password"
          v-model:value="form.password"
          label="Password"
          type="password"
          placeholder="***************"
        />

        <Button class="btn-login btn-black" type="submit">
          <span>Log In</span>

          <span class="material-icons"> &#xea77; </span>
        </Button>
      </form>

      <p class="copyright">
        <span class="material-icons"> &#xe86f; </span>
        Made by <a target="_blank" href="https://github.com/mavulp">Mavulp</a> in {{ new Date().getFullYear() }}
      </p>

      <!-- <p class="copy-text">Copyright {{ new Date().getFullYear() }} Mavulp (hivecom devs)</p> -->
    </div>
  </div>
</template>
