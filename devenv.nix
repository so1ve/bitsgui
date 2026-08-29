{ lib, pkgs, ... }:

let
  desktopLibraries = with pkgs; [
    glib-networking
    gtk3
    libayatana-appindicator
    openssl
    webkitgtk_4_1
    xdotool
  ];
in
{
  languages.rust = {
    enable = true;
    toolchainFile = ./rust-toolchain.toml;
  };

  packages =
    with pkgs;
    [
      actionlint
      nixfmt-tree
      pkg-config
      tombi
    ]
    ++ desktopLibraries;

  env.LD_LIBRARY_PATH = lib.makeLibraryPath desktopLibraries;
  env.GIO_EXTRA_MODULES = "${pkgs.glib-networking}/lib/gio/modules";
  env.XDG_DATA_DIRS = lib.makeSearchPath "share" [
    pkgs.gsettings-desktop-schemas
    pkgs.gtk3
  ];
}
