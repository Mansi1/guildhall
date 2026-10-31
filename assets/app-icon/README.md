# Guildhall app icon

The app icon is a columned hall in cream on a copper square: original Guildhall artwork, MIT OR
Apache-2.0 like the code. Every file in this directory is rendered by `packaging/icons.sh` from
the two SVG sources below.

## Files

- `guildhall.svg`: the source of the app icon; also the hicolor scalable icon.
- `guildhall-mark-black.svg`: the hall alone in black, the source of the macOS menu-bar template.
- `guildhall-1024.png`: 1024 px render, with extra transparent margin for Apple's icon grid.
- `guildhall.icns`: macOS bundle icon (`CFBundleIconFile`).
- `guildhall.ico`: Windows icon, 16–256 px, embedded in the `.exe` by
  `apps/guildhall/build.rs` and used by the MSI.
- `hicolor/<size>/apps/io.github.mansi1.guildhall.png` (16–512) and
  `hicolor/scalable/apps/io.github.mansi1.guildhall.svg`: Linux icon theme.
- `tray-mark-black-44.png` (macOS template, 22 pt @2x) and `tray-64.png` (Windows, Linux,
  FreeBSD): the tray / menu-bar icon.

## Regenerate

Edit the SVG sources, then run `packaging/icons.sh` on any OS. It needs `resvg`; the `.ico` and
`.icns` are packed by `cargo xtask ico` / `cargo xtask icns`. Never edit the renders by hand.
