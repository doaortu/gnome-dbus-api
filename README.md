# Rust dbus gnome shell API

A friendly API for interacting with gnome shell, freedesktop and other D-Bus services that are available on **Ubuntu**.

## Features

- [x] Power management
  - [x] Power off
  - [x] Reboot
  - [x] Suspend
- [x] Locales
  - [x] Get x11 layout
- [x] Gnome extensions
  - [x] Get extensions
  - [x] Enable extension
  - [x] Disable extension
  - [x] Uninstall extension
- [x] Gnome shell screenshot
  - [x] Pick color
- [x] Settings

  - [x] Night light
    - [x] Get night light status
    - [x] Set night light status
    - [x] Get night light temperature
    - [x] Set night light temperature

- [ ] Gsettings Dconf (https://crates.io/crates/dconf_rs/0.3.0)
  - [ ] ![image](https://github.com/JulianKominovic/gnome-dbus-api/assets/70329467/a8acb0e3-8759-4dea-9b28-0dfabcb0709e)
  - [ ] org.gnome.desktop.peripherals.keyboard delay 500 (initial key repeat delay)
  - [ ] org.gnome.desktop.peripherals.keyboard repeat-interval 30 (initial key repeat delay)
  - [ ] org.gnome.desktop.peripherals.mouse natural-scroll true
  - [ ] org.gnome.desktop.peripherals.touchpad tap-to-click true
  - [ ] org.gnome.desktop.peripherals.touchpad two-finger-scrolling-enabled true
  - [ ] org.gnome.desktop.calendar show-weekdate
  - [ ] org.gnome.desktop.interface clock-format
  - [ ] org.gnome.desktop.interface clock-show-date true
  - [ ] org.gnome.desktop.interface clock-show-seconds false
  - [ ] org.gnome.desktop.interface clock-show-weekday true
  - [ ] org.gnome.desktop.interface color-scheme 'prefer-light'
  - [ ] org.gnome.desktop.interface cursor-blink true
  - [ ] org.gnome.desktop.interface cursor-blink-time 1200
  - [ ] org.gnome.desktop.interface cursor-size 24
  - [ ] org.gnome.desktop.interface cursor-blink-timeout 10
  - [ ] org.gnome.desktop.interface enable-hot-corners false
  - [ ] org.gnome.desktop.interface locate-pointer false (with ctrl key)
  - [ ] org.gnome.desktop.interface overlay-scrolling true
  - [ ] org.gnome.desktop.interface show-battery-percentage true
  - [ ] org.gnome.desktop.privacy disable-camera false
  - [ ] org.gnome.desktop.privacy disable-microphone false
  - [ ] org.gnome.desktop.privacy disable-sound-output false
  - [ ] org.gnome.desktop.privacy hide-identity false
  - [ ] org.gnome.desktop.screensaver picture-uri 'file:///home/julian/...'
  - [ ] org.gnome.desktop.background show-desktop-icons true
  - [ ] org.gnome.desktop.background picture-uri-dark 'file:///home/julian/Pictures/Wallpapers/image.webp'
  - [ ] org.gnome.shell development-tools true
  - [ ] org.gnome.shell disable-user-extensions false
  - [ ] org.gnome.desktop.a11y always-show-text-caret false
  - [ ] org.gnome.desktop.a11y always-show-universal-access-status false
  - [ ] org.gnome.desktop.a11y.applications screen-keyboard-enabled false
  - [ ] org.gnome.desktop.a11y.applications screen-magnifier-enabled false
  - [ ] org.gnome.desktop.a11y.applications screen-reader-enabled false
  - [ ] **org.gnome.shell.extensions.dash-to-dock > XYZ**
  - [ ] org.gnome.mutter center-new-windows
  - [ ] org.gnome.gnome-session auto-save-session false (restore open apps on login)

## Interfaces

- [ ] org.freedesktop.UPower: (https://crates.io/crates/upower_dbus)
  - [ ] is_on_battery
  - [ ] is_lid_closed
  - [ ] enumerate_devices
  - [ ] get_display_device
  - [ ] device, battery, external_device
    - [ ] type (important to identify external devices) (https://upower.freedesktop.org/docs/Device.html)
    - [ ] state (https://upower.freedesktop.org/docs/Device.html)
    - [ ] technology (https://upower.freedesktop.org/docs/Device.html)
    - [ ] get_percentage
    - [ ] time_to_empty (seconds) (0 if unknown)
    - [ ] time_to_full (seconds) (0 if unknown)
    - [ ] capacity (battery life)
    - [ ] energy_full (Wh) (actual max charge)
    - [ ] energy_full_design (Wh) (factory max charge)
    - [ ] energy_rate (W) (current power draw)
    - [ ] temperature (K)
    - [ ] model
    - [ ] vendor
    - [ ] voltage
- [ ] net.hadess.PowerProfiles: power profiles (power save, balanced, performance)
  - [ ] active_profile (read/write)
  - [ ] PerformanceInhibited (read) (reason for performance being inhibited)
  - [ ] PerformanceDegraded (read) (reason for performance being degraded)
- [ ] org.a11y.Bus: accessibility bus

  - [ ] isEnabled (read/write)
  - [ ] screenReaderEnabled (read/write)

- org.bluez: bluetooth devices, devices stats
- org.freedesktop.NetworkManager: network manager, wifi, connections
- org.freedesktop.UDisks2: disks, partitions, filesystems
- org.freedesktop.FileManager1: file manager (nautilus)
- org.gnome.SettingsDaemon.\*: settings daemon
  - [ ] org.gnome.SettingsDaemon.Power:
    - [ ] keyboard
      - [ ] brightness (r/w)
      - [ ] brightness-step-up
      - [ ] brightness-step-down
      - [ ] brightness-toggle
    - [ ] screen
      - [ ] brightness (r/w)
      - [ ] brightness-step-up
      - [ ] brightness-step-down
      - [ ] brightness-toggle

## Devtools

- [D-Spy](https://flathub.org/apps/org.gnome.dspy): search for and inspect D-Bus services  
  ![D-Spy example](d-spy.png)

## Dbus sources

- [What is DBUS? Use cases](https://www.baeldung.com/linux/dbus)
- [DBUS data types](https://www.alteeve.com/w/List_of_DBus_data_types)
- [CLI DBUS man pages](https://sarata.com/manpages/dbus-send.1.html)
- [DBUS Gnome shell interfaces](https://gitlab.gnome.org/GNOME/gnome-shell/-/tree/92d3c6e051958b31151bf9538205a71cab6f70d7/data/dbus-interfaces)

## GSettings

[Gnome mutter schema](https://github.com/GNOME/mutter/blob/main/data/org.gnome.mutter.gschema.xml.in)

## Rust bindings

- [zbus](https://dbus2.github.io/zbus/introduction.html) a beatifully designed abstraction over D-Bus
- [zvariant](https://docs.rs/zvariant/latest/zvariant/) a crate for working with D-Bus types
