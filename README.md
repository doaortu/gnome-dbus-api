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
    - [ ] Get night light status
    - [ ] Set night light status
    - [ ] Get night light temperature
    - [ ] Set night light temperature

## Interfaces

- net.hadess.PowerProfiles: power profiles (power save, balanced, performance)
- org.bluez: bluetooth devices, devices stats
- org.freedesktop.NetworkManager: network manager, wifi, connections
- org.freedesktop.UDisks2: disks, partitions, filesystems
- org.freedesktop.UPower: power management, battery, external devices battery, battery degradation, etc...
- org.a11y.Bus: accessibility bus
- org.freedesktop.FileManager1: file manager (nautilus)
- org.gnome.SettingsDaemon.\*: settings daemon

## Devtools

- [D-Spy](https://flathub.org/apps/org.gnome.dspy): search for and inspect D-Bus services  
  ![D-Spy example](d-spy.png)

## Dbus sources

- [What is DBUS? Use cases](https://www.baeldung.com/linux/dbus)
- [DBUS data types](https://www.alteeve.com/w/List_of_DBus_data_types)
- [CLI DBUS man pages](https://sarata.com/manpages/dbus-send.1.html)
- [DBUS Gnome shell interfaces](https://gitlab.gnome.org/GNOME/gnome-shell/-/tree/92d3c6e051958b31151bf9538205a71cab6f70d7/data/dbus-interfaces)

## Rust bindings

- [zbus](https://dbus2.github.io/zbus/introduction.html) a beatifully designed abstraction over D-Bus
- [zvariant](https://docs.rs/zvariant/latest/zvariant/) a crate for working with D-Bus types
