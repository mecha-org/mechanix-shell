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
