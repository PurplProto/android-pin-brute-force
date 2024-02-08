# Android Pin Brute Force (APBF)

This app is was written to brute force the pin of an Android device. This is risky and could cause the target device to completely lockdown and thus requiring a factory reset making the user data irrecoverable, so heed the following warnings:

- This is known to not work on Android devices after v10
- This has been tested on only a single device (Samsung S8)
- You use this project and it's content at your own risk
- No warranty, help or support is implied as per the [LICENSE](./LICENSE).

## Prerequisites

The following should be achievable by yourself already:

- A spare Android device
  - Running a kernel with HID support
  - A full chroot install of Nethunter
  - Root access
- A locked Android device

## Install

TODO

## Build the app yourself

This has only been tested on WSL using Ubuntu 22.04.3 LTS.

### Linux

1. Install Rust: [https://www.rust-lang.org/tools/install](rustup)
2. Install the Android [https://developer.android.com/tools/sdkmanager](sdkmanager)
3. Install NDK using Android sdkmanager: `sdkmanager "platforms;android-33" "ndk;25.2.9519653"`
4. Export `ANDROID_NDK_HOME` with the path of the NDK folder, you can find more [guidance here](https://github.com/bbqsrc/cargo-ndk). I suggest adding this variable to your `.bashrc`.
5. Clone this repo
6. Open a shell and cd into the cloned repo
7. Execute `cargo ndk -t arm64-v8a -p 33 build --release`
8. Locate the built executable at `target/aarch64-linux-android/release/android-pin-brute-force`

## Attributions

This project was inspired by these projects:

- [urbanadventurer/Android-PIN-Bruteforce](https://github.com/urbanadventurer/Android-PIN-Bruteforce)
- [byt3bl33d3r/duckhunter](https://github.com/byt3bl33d3r/duckhunter)

This project has uses content from these projects:

- [Linux USB HID gadget driver](https://docs.kernel.org/usb/gadget_hid.html)
- [danielmiessler/SecLists](https://github.com/danielmiessler/SecLists)
- [bbqsrc/cargo-ndk](https://github.com/bbqsrc/cargo-ndk)
- [zenito9970/countdown-rs](https://github.com/zenito9970/countdown-rs)
