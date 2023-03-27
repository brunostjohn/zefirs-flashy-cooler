# ![Logo](https://raw.githubusercontent.com/brunostjohn/zefirs-flashy-cooler/main/assets/images/favicon-32x32.png) Zefir's Flashy Cooler

This is a desktop app that controls LCD CPU coolers instead of your manufacturer's software. Most of the app is just Electron living its best life but the device handling code is written in Rust. This and an RGB control app (like [SignalRGB](https://signalrgb.com/)) will use literally less CPU than your manufacturers app while looking cooler. Oh and also my cat's in it. So there's that.

In case of any issues, there's a Discord server [here](https://discord.gg/KXmnwA6zWK) where I'm available to help.

Currently supported coolers:
- Corsair's Capellix LCD (revision 1)
- Corsair's Capellix LCD (reviison 2)
- Thermaltake's Ultra LCD

![Screenshot 1](https://raw.githubusercontent.com/brunostjohn/zefirs-flashy-cooler/main/assets/images/screenshot.png)
![Screenshot 2](https://raw.githubusercontent.com/brunostjohn/zefirs-flashy-cooler/main/assets/images/screenshot2.png)

This is designed explicitly for use with Windows.
Will it run on Linux or macOS? Maybe. Probably not. Linux support is something I frequently think about. I actually might make that a thing once I get all the basic functionality down.

## Installation:

Download the installer from the releases tab and enjoy. There are some bugs to iron out so please submit on issue when you notice one. LibreHardwareMonitor is required for this to be able to use any system sensor input (CPU, GPU etc.).

## Building:

To build this app from source, you will need an up-to-date Rust toolchain in your PATH. Once that requirement is satisfied, open a terminal in this project's directory and run `npm i`. This will download all dependencies. After that, build the native modules with `npx electron-rebuild`. Now, onto the build options/scripts.

- `npm start` will compile all native device drivers with target debug and start the app in debug mode.
- `npm run start-release` will compile all native device drivers with target release and start the app in debug mode.
- `npm run make` will compile all native device drivers with target release and create an installable app bundle.
- `npm run build-debug` will compile all native device drivers with target debug.
- `npm run build` is a shorthand for the command above.
- `npm run build-release` will compile all native device drivers with target release.
- `npm start-no-recompile` will start the app in debug mode.

## I want to make my own theme.

That's great, the [wiki on themes](https://github.com/brunostjohn/zefirs-flashy-cooler/wiki/Themes) is a great starting point.

## What it does and what it doesn't do.

- [x] Display a static image to the LCD.
- [x] Have theme support.
- [x] Read data from system sensors.
- [x] Display that data.
- [x] Have a GUI.
- [x] Have external theme support.
- [ ] A bunch of other stuff I'll probably come up with while developing this.

### Last words

I'm literally learning JavaScript, NodeJS, and all the other technologies I'm using here while writing this. If there's anything you feel can be improved, feel free to submit a PR.

Massive thanks to TheDordo from the SignalRGB team for working on REing the display's USB protocol with me.
