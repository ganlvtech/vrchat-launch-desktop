# VRChat Launch In Desktop Mode

![Human Crafted](https://img.shields.io/badge/Human%20Crafted-100%25-brightgreen)
![AI Assisted](https://img.shields.io/badge/AI%20Assisted-Yes-blue)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://choosealicense.com/licenses/mit/)

Add "Launch World (Desktop)" button to VRChat Website Launching Page (eg: https://vrchat.com/home/launch?worldId=wrld_4432ea9b-729c-46e3-8eaf-846aa0a37fdd)

## Install

1. Install [Tampermonkey for Chrome](https://chromewebstore.google.com/detail/dhdgffkkebhmkfjojejmpbldmpobfkfo) or [Tampermonkey for Edge](https://microsoftedge.microsoft.com/addons/detail/iikmkjmpaadaobahmlepeloendndfphd) in your browser.
2. Install [VRChat-Launch-World-Desktop.user.js](https://raw.githubusercontent.com/ganlvtech/vrchat-launch-desktop/main/VRChat-Launch-World-Desktop.user.js) in Tampermonkey.
3. Download `launchnovr.exe` from https://github.com/ganlvtech/vrchat-launch-desktop/releases and place it in somewhere.
4. Double click to run `launchnovr.exe`. Select `OK`.

## Uninstall

1. Remove `VRChat Launch World Desktop` in Tampermonkey options page.
2. [Uninstall `launchnovr.exe` in Windows settings](https://support.microsoft.com/en-us/windows/uninstall-or-remove-apps-and-programs-in-windows-4b55f974-2cc6-2d2b-d092-5905080eaf98).
3. Delete `launchnovr.exe` manually.
## How It Works

Register `vrchatnovr://` URL Scheme in the Registry `HKEY_CURRENT_USER\SOFTWARE\Classes\vrchatnovr`.

Use Tampermonkey userscripts to add "Launch World (Desktop)" button to VRChat Website just next to "Launch World" button and replace `vrchat://` with `vrchatnovr://`.

It will run `launchnovr.exe`, and `launchnovr.exe` will run original `launch.exe` with `--no-vr` flag.
