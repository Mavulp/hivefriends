import { createPinia } from "pinia"
import { createApp } from "vue"
import App from "./App.vue"
import router from "./router"

const app = createApp(App).use(createPinia()).use(router)

// Init icons
// import PhotoLibrary from "vue-material-design-icons/PhotoLibraryIcon.vue"
// import Settings from "vue-material-design-icons/SettingsIcon.vue"
// import Logout from "vue-material-design-icons/LogoutIcon.vue"

// app.component("icon-photo-library", PhotoLibrary)
// app.component("icon-settings", Settings)
// app.component("icon-logout", Logout)

app.mount("#app")
