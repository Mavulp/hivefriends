<script setup lang="ts">
import InputText from "../../components/form/InputText.vue"
import Button from "../../components/Button.vue"
import { ref, reactive, computed, nextTick, onBeforeMount } from "vue"
import { getRanMinMax } from "../../js/utils"
import { useUser } from "../../store/user"
import { useFormValidation, required, minLength, asyncValidation } from "../../js/validation"
import { useRouter } from "vue-router"
import { useBread } from "../../store/bread"

const router = useRouter()
const bread = useBread()
const auth = useUser()

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
  "zealseal"
]
const placeholder = ref(placeholders[getRanMinMax(0, 5)])

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
      console.log("test", errors)
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
    bread.set("Please sign-in to hi!friends")
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
        <InputText :error="errors.username" v-model:value="form.username" label="Username" :placeholder="placeholder" />
        <InputText
          :error="errors.password"
          v-model:value="form.password"
          label="Password"
          type="password"
          placeholder="***************"
        />

        <Button class="btn-login btn-black" type="submit" size="56px">
          <span>Sign In</span>
        </Button>
      </form>

      <p class="copy-text">Copyright {{ new Date().getFullYear() }} Mavulp (hivecom devs)</p>
    </div>
  </div>
</template>
