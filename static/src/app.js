'use strict';

import './app.css';
import { createApp } from 'vue';
import * as dayjs from 'dayjs';
import reject from 'lodash/reject';

function App() {

    const fnSaveSyncFlow = async (event) => { }

    const fnToggleModal = (event) => {
        const body = document.querySelector('body');
        const modal = document.getElementById('dvModalAddGame');
        modal.classList.toggle('hidden');
        modal.classList.toggle('pointer-events-none');
        body.classList.toggle('modal-active');
    }

    const fnAttachEventListener = (pageName) => {
        if (pageName === 'home') {
            const elem_btnSaveSyncFlow = document.getElementById('btnSaveSyncFlow');
            const elem_btnShowModal = document.getElementById('btnShowModalAddGame');
            const elem_btnCloseModal = document.querySelector('.modal-close');

            elem_btnSaveSyncFlow.addEventListener('click', fnSaveSyncFlow);
            elem_btnShowModal.addEventListener('click', fnToggleModal);
            elem_btnCloseModal.addEventListener('click', fnToggleModal);
        }
    };

    const initSearchApp = () => {
        const app = createApp({
            data() {
                return {
                    keyword: '',
                    isSearching: false,
                    games: [],
                    selectedGames: []
                }
            },
            methods: {
                async search(event) {
                    const searchKeyword = this.keyword;
                    if (!searchKeyword) {
                        return;
                    }

                    this.isSearching = true;
                    const response = await fetch(`/api/search?keyword=${searchKeyword}`, {
                        method: 'GET',
                        headers: {
                            'Content-Type': 'application/json;charset=utf-8'
                        }
                    });
                    this.isSearching = false;

                    if (response.status !== 200) {
                        alert('Halla bol!');
                    }

                    this.games = await response.json();
                },
                selectGame(gameId, gameName) {
                    this.selectedGames.push({
                        id: gameId,
                        name: gameName
                    });
                },
                nukeSelf(gameId) {
                    this.selectedGames = reject(this.selectedGames, (game) => { return game.id === gameId });
                }
            }
        });
        app.config.globalProperties.$filters = {
            epochToHuman: (value) => {
                if (!value) return ''
                const dtTimestamp = dayjs.unix(value);
                return dtTimestamp.format('DD-MMM-YYYY');
            }
        };
        const vm = app.mount('#searchGamesApp');
    }

    this.init = (pageName) => {
        fnAttachEventListener(pageName);

        initSearchApp();
        return true;
    }
};
(() => {
    const app = new App();
    app.init('home');
})();
