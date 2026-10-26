// ==UserScript==
// @name         VRChat Launch World Desktop
// @namespace    http://tampermonkey.net/
// @version      2026-10-26
// @description  Add "Launch World (Desktop)" button to VRChat Website Launching Page
// @author       Ganlv
// @homepageURL  https://github.com/ganlvtech/vrchat-launch-desktop
// @match        https://vrchat.com/home/*
// @icon         https://vrchat.com/favicon.ico
// ==/UserScript==

(function () {
    'use strict';

    let currentId = 0;
    const onNavigate = async () => {
        const myId = ++currentId;
        for (let i = 0; ; i++) {
            if (currentId != myId) {
                return;
            }
            let found = false;
            for (const a of document.querySelectorAll('a[href*="vrchat://"]')) {
                if (!a.parentElement.querySelector('a[href*="vrchatnovr://"]')) {
                    if (a.textContent.trim() === 'Launch World') {
                        a.insertAdjacentHTML('afterend', a.outerHTML.replace('<a ', '<a style="margin-top:.5rem" ').replace(/vrchat:\/\//g, 'vrchatnovr://').replace('Launch World', 'Launch World (Desktop)'));
                        found = true;
                    }
                }
            }
            if (found) {
                break;
            }
            if (i < 50) {
                await new Promise(resolve => setTimeout(resolve, 100));
            } else {
                await new Promise(resolve => setTimeout(resolve, 2000));
            }
        }
    };
    navigation.addEventListener("navigate", onNavigate);
    onNavigate();
})();