# Пакет barricade-bar.
#   nix-build                       # сборка без flake
#   pkgs.callPackage ./default.nix {}  # из своей конфигурации
{
  pkgs ? import <nixpkgs> { },
  lib ? pkgs.lib,
  rustPlatform ? pkgs.rustPlatform,
}:

rustPlatform.buildRustPackage {
  pname = "barricade-wallpaper";
  version = (lib.importTOML ./Cargo.toml).package.version;

  src = lib.fileset.toSource {
    root = ./.;
    fileset = lib.fileset.unions [
      ./Cargo.toml
      ./Cargo.lock
      ./src
    ];
  };

  cargoLock.lockFile = ./Cargo.lock;

  nativeBuildInputs = with pkgs; [
    pkg-config
    wrapGAppsHook3
  ];

  buildInputs = with pkgs; [
    gtk3
    glib
    cairo
    pango
    gdk-pixbuf
    gtk-layer-shell
    dbus
    networkmanager
    libpulseaudio
    adwaita-icon-theme # символические иконки freedesktop
  ];

  # wpctl нужен виджету audio во время работы
  preFixup = ''
    gappsWrapperArgs+=(--prefix PATH : ${lib.makeBinPath [ pkgs.wireplumber ]})
  '';

  meta = {
    description = "обои для тайлинг композиторов на GTK3";
    homepage = "https://github.com/yarok-k/barricade-wallpaper";
    license = lib.licenses.gpl3Plus;
    mainProgram = "barricade-wallpaper";
    platforms = lib.platforms.linux;
  };
}
