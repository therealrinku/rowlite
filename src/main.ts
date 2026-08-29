import { createApp } from "vue";
import PrimeVue from 'primevue/config';
import Aura from '@primeuix/themes/aura';

import App from "./App.vue";
import './styles.css';

const app = createApp(App).mount("#app");
app.use(PrimeVue, {
    theme: {
        preset: Aura
    }
});
