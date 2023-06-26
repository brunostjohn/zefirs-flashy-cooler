# ![Logo](https://raw.githubusercontent.com/brunostjohn/zefirs-flashy-cooler/main/static/images/favicon-32x32.png) Zefir's Flashy Cooler

## This branch is a transition to Tauri and Rust away from Electron. This app is still being worked on, it'll take time before this is fully cooked. Thank you for your patience.

This is a desktop app that controls LCD CPU coolers instead of your manufacturer's software. Most of the app is just Electron living its best life but the device handling code is written in Rust. This and an RGB control app (like [SignalRGB](https://signalrgb.com/)) will use literally less CPU than your manufacturers app while looking cooler. Oh and also my cat's in it. So there's that.

In case of any issues, there's a Discord server [here](https://discord.gg/KXmnwA6zWK) where I'm available to help.

Currently supported coolers:

- Corsair's Capellix LCD (revision 1)
- Corsair's Capellix LCD (reviison 2)
- Thermaltake's Ultra LCD

![Screenshot 1](https://raw.githubusercontent.com/brunostjohn/zefirs-flashy-cooler/main/static/images/screenshot.png)
![Screenshot 2](https://raw.githubusercontent.com/brunostjohn/zefirs-flashy-cooler/main/static/images/screenshot2.png)

This is designed explicitly for use with Windows.
Will it run on Linux or macOS? Maybe. Probably not. Linux support is something I frequently think about. I actually might make that a thing once I get all the basic functionality down.

## Installation:

Download the installer from the releases tab and enjoy. There are some bugs to iron out so please submit on issue when you notice one. LibreHardwareMonitor is required for this to be able to use any system sensor input (CPU, GPU etc.).

## Building:

To build this app from source, you'll need to give it some TLC.

1. Have a Rust toolchain up and running.
2. Have Visual Studio set up for C# and .NET AOT.
3. Have Ultralight's statically linked libs downloaded.

### How to build

1. Copy over all the Ultralight static libs to `/src-tauri/static-libs`.
2. Compile LibreHardwareMonitorNative with `dotnet publish /p:NativeLib=Static /p:SelfContained=true -r win-x64 -c release` and also copy it to `/src-tauri/static-libs`.
3. You might be missing some libs in your PATH. Check `/src-tauri/build.rs` and change directories to be applicable to your environment.
4. If `System.IO.Compression.Native.Aot.lib` throws linker conflicts for you, run `lib /remove:libs-native\System.IO.Compression.Native\CMakeFiles\System.IO.Compression.Native.Aot.dir\D\_\a_work\1\s\src\native\external\brotli\dec\decode.c.obj /LIBPATH:[WHEREEVER YOU HAVE THE REPO ON YOUR MACHINE]zefirs-flashy-cooler\src-tauri\static-libs .\System.IO.Compression.Native.Aot.lib`. These symbols are still covered by a different library and everything still functions correctly.
5. Run `cargo tauri dev` in a cmd window with admin privileges. You should see that app start up.

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
