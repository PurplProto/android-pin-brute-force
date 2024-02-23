# Android Pin Brute Force (APBF)

This app is was written to brute force the pin of an Android device. This is risky and could cause the target device to completely lockdown and thus requiring a factory reset making the user data irrecoverable, so heed the following warnings:

- ⚠️ This is known to not work on Android devices after v10 ⚠️
- This has been tested on only a single target device (Samsung S8) and host device (Google Pixel 3XL)
- You use this project and it's content at your own risk
- No warranty, help or support is implied as per the [MIT LICENSE](./LICENSE).
- This the first app I've built in Rust, therefore expect bugs/issues (feel free to create an issue, although I can't guarantee I can fix it myself)

⚠️ The app is unable to detect a successful pin entry, so you will need to keep an check on the process. ⚠️

## Prerequisites

The following should be achievable by yourself already:

- A spare Android device
  - Running a kernel with HID support
  - With a full chroot install of Nethunter
  - Has Root access
- A locked Android device

## Usage

Grab the built binary and push it to your device in your favourite way, i.e. `adb push apbf ~/apbf` or if you have the Nethunter ssh deamon running `rsync -P apbf root@nethunterip:~/`.
The binary should be in the Nethunter chroot and must be executable.

```
A tool to brute force the PIN of an Android device.

Usage: apbf [OPTIONS] [COMMAND]

Commands:
  start   Starts brute force attack
  resume  Resumes brute force attack
  help    Print this message or the help of the given subcommand(s)

Options:
  -c, --cool-down <COOL_DOWN>
          List of cool down periods between pin attempts. Go format and count seperated by a colon i.e. -c 15s:3 -c 10m:3 -c 30m:-1 Omitting the the count or using -1 sets the cool down period until the end of the pin list
  -k, --keyboard-device <KEYBOARD_DEVICE>
          <Optional> keyboard device file to use. Defaults to: /dev/hidg0
  -m, --mouse-device <MOUSE_DEVICE>
          <Optional> mouse device file to use. Defaults to: /dev/hidg1
  -p, --pin-size <PIN_SIZE>
          <Optional> Size of the pin to brute force. Defaults to 4. Currently supports 4 and 6
  -v, --verbose...
          <Optional> Turn debugging information on. Can be passed up to 2 times for more verbosity
  -l, --log-file-path <LOG_FILE_PATH>
          <Optional> Logfile path. If exists, appends to the file
  -h, --help
          Print help
  -V, --version
          Print version
```

### Example

This will attempt 4 digit pins every 15 seconds 4 times, then every minute 4 times, followed by every 10 minutes just 2 times and finally will try a pin every 30 minutes until all remaining pins have been tried.

```bash
./apbf -v -c 15s:4 -c 1m:4 -c 10m:2 -c 30m:-1
```

## Build the app yourself

This has only been tested on WSL using Ubuntu 22.04.3 LTS.

Due to some odd dynamic linker issues in the Nethunter chroot while testing, the app is statically complied so no external dependencies are required at runtime (with the exception of the x86_64 build).

### Linux

1. Install Rust: [https://www.rust-lang.org/tools/install](rustup)
2. Install the Android [https://developer.android.com/tools/sdkmanager](sdkmanager)
3. Install NDK using Android sdkmanager: `sdkmanager "platforms;android-33" "ndk;25.2.9519653"`
4. Export `ANDROID_NDK_HOME` with the path of the NDK install folder, you can find more [guidance here](https://github.com/bbqsrc/cargo-ndk). I suggest adding this variable to your `.bashrc`.
5. Clone this repo
6. Open a shell and cd into the cloned repo
7. Execute `cargo ndk -t arm64-v8a -p 33 build --release`
    - Or for a statically linked binary do `RUSTFLAGS="-C target-feature=+crt-static" cargo ndk -t arm64-v8a -p 33 build --release` instead
8. Locate the built executable at `target/aarch64-linux-android/release/android-pin-brute-force`

## Downloads
I had troubles getting the dynamically linked builds running in the Nethunter chroot environment, therefore all builds are built statically linked appart from x86_64, I could not get this building statically linked, therefore it is a dynamically linked binary.

You can find the built binaries [on the releases page](https://github.com/PurplProto/android-pin-brute-force/releases/latest).

## Attributions

This project was inspired by these projects:

- [urbanadventurer/Android-PIN-Bruteforce](https://github.com/urbanadventurer/Android-PIN-Bruteforce)
- [byt3bl33d3r/duckhunter](https://github.com/byt3bl33d3r/duckhunter)

This project has uses content from these projects:

- [Linux USB HID gadget driver](https://docs.kernel.org/usb/gadget_hid.html)
- [danielmiessler/SecLists](https://github.com/danielmiessler/SecLists)
- [bbqsrc/cargo-ndk](https://github.com/bbqsrc/cargo-ndk)
- [zenito9970/countdown-rs](https://github.com/zenito9970/countdown-rs)
