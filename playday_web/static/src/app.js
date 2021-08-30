'use strict';

import './app.css';
import { createApp } from 'vue';
import * as dayjs from 'dayjs';
import VueApp from './vue/App.vue';
import relativeTime from 'dayjs/plugin/relativeTime';
import utc from 'dayjs/plugin/utc';

dayjs.extend(relativeTime);
dayjs.extend(utc);
function App() {

    const initPage = (pageName) => {
        if (pageName === 'home') {
            // const elem_btnShowModal = document.getElementById('btnShowModalAddGame');
            // elem_btnShowModal.addEventListener('click', fnToggleModal);
            const homeApp = createApp(VueApp);
            homeApp.config.globalProperties.$filters = {
                epochToHuman: (value) => {
                    if (!value) return '';
                    const dtTimestamp = dayjs.unix(value);
                    return dtTimestamp.format('DD-MMM-YYYY');
                },
                epochToRelative: (value) => {
                    if (!value) return '';

                    const dtTimestamp = dayjs.unix(value);
                    return dtTimestamp.fromNow();
                },
                iso8601ToRelative: (value) => {
                    if (!value) return '';

                    const dtTimestamp = dayjs.utc(value);
                    return dtTimestamp.fromNow();
                }
            };

            homeApp.mount('#app');
        }
    };

    this.init = (pageName) => {
        initPage(pageName);
        return true;
    }
};
(() => {
    const app = new App();
    app.init('home');
})();
