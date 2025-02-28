Edit files at

 `/usr/share/xdg-desktop-portal/portals/mechanix.portal`

```rust
[portal]
DBusName=org.mechanix.services.FileChooser
Interfaces=org.freedesktop.impl.portal.FileChooser
UseIn=gnome;sway;xfce;
```

 `/usr/share/xdg-desktop-portal/portals/portal.conf`

```rust
[preferred]
org.freedesktop.impl.portal.FileChooser=mechanix
default=gnome;gtk;
```

this command can be used to check your config out:

```rust
$ /usr/libexec/xdg-desktop-portal -v
```

```
`echo $XDG_CURRENT_DESKTOP`
```
`echo $XDG_CURRENT_DESKTOP`

`journalctl -xe | grep dbus`  for debugging dbus

`systemctl --user restart xdg-desktop-portal` restart xdg portals

command to check whether backend is working or not
```busctl --user introspect org.mechanix.services.FileChooser /org/mechanix/services/FileChooser```


this command apparently works to check the bus

```
gdbus call --session \
    --dest org.mechanix.services.FileChooser \
    --object-path /org/mechanix/services/FileChooser \
    --method org.freedesktop.impl.portal.FileChooser.OpenFile \
    "/org/mechanix/services/FileChooser" \
    "com.example.app" \
    "" \
    "Open a file" \
    "{}"

```
