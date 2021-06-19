'use strict';

import './app.css';
import { createApp } from 'vue';
import * as dayjs from 'dayjs';
import reject from 'lodash/reject';
import lodashFind from 'lodash/find';

function App() {

    const fnToggleModal = (event) => {
        const body = document.querySelector('body');
        const modal = document.getElementById('dvModalAddGame');
        modal.classList.toggle('hidden');
        modal.classList.toggle('pointer-events-none');
        body.classList.toggle('modal-active');
    }

    const fnAttachEventListener = (pageName) => {
        if (pageName === 'home') {
            const elem_btnShowModal = document.getElementById('btnShowModalAddGame');
            elem_btnShowModal.addEventListener('click', fnToggleModal);
        }
    };

    const initSearchApp = () => {
        const app = createApp({
            data() {
                return {
                    keyword: '',
                    isSearching: false,
                    isSaving: false,
                    games: [],
                    selectedGames: []
                }
            },
            methods: {
                closeModal() {
                    fnToggleModal();
                    this.keyword = '';
                    this.isSearching = false;
                    this.isSaving = false;
                    this.games = [];
                    this.selectedGames = [];
                },
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
                        return alert('Halla bol!');
                    }

                    this.games = await response.json();
                },
                isGameSelected(gameId) {
                    const thisGame = lodashFind(this.selectedGames, (game) => { return game.id === gameId });

                    if (thisGame) {
                        return true;
                    } else {
                        return false;
                    }
                },
                selectGame(igdbGame) {
                    if (!this.isGameSelected(igdbGame.id)) {
                        this.selectedGames.push(igdbGame);
                    }
                },
                nukeSelf(gameId) {
                    this.selectedGames = reject(this.selectedGames, (game) => { return game.id === gameId });
                },
                async addToWishlist() {
                    if (this.selectedGames.length === 0) {
                        return;
                    }
                    this.isSaving = true;
                    const response = await fetch('/api/wishlist', {
                        method: 'POST',
                        headers: {
                            'Content-Type': 'application/json;charset=utf-8'
                        },
                        body: JSON.stringify(this.selectedGames)
                    });
                    this.isSaving = false;
                    if (response.status !== 200) {
                        return alert('Udi baba!');
                    }

                    this.closeModal();
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
