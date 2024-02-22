# Nexus Steam Integration

Some people love to track their time played through Steam, this is quite easy for those who downloaded
the game through Steam, but for those who use the standalone client, they might want to track their time as well.

This little addon is heavily influenced by the original Addon Loader version here https://github.com/vimodev/GW2SteamRunner

I wanted to bring something similar to the [Nexus](https://raidcore.gg/Nexus) framework

### Installation

> Install directly from the Nexus Addons Library or if it's not available, install locally below

1. Download the latest `nexus_steam_integration.dll` from the Releases page
2. Drop the `nexus_steam_integration.dll` into your `<Guild Wars 2>/addons/` folder
    1. Make sure the `nexus_steam_integration.dll` is unblocked on your machine, by right-clicking the `.dll` file and clicking Properties
    2. If it's blocked at the bottom of the page if you see a "Unblock" checkbox, you must check this and save.

We will also require the `steam_api64.dll` files to actually tell Steam we're playing Guild Wars 2

1. Download the latest `steam_api64.dll` and `steam_api64.lib`
2. Drop these two files into the **root** of your Guild Wars 2 folder
> Note: This is NOT the `addons/` folder but one level up
