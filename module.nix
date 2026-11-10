# Модуль NixOS: programs.barricade-bar.enable = true;
{ config, lib, pkgs, ... }:

let
  cfg = config.programs.barricade-wallpeper;
in
{
  options.programs.barricade-wallpaper = {
    enable = lib.mkEnableOption "barricade-bar, a status bar for niri";

    package = lib.mkOption {
      type = lib.types.package;
      default = pkgs.callPackage ./default.nix { };
      defaultText = lib.literalExpression "pkgs.callPackage ./default.nix { }";
      description = "Пакет barricade-bar.";
    };
  };

  config = lib.mkIf cfg.enable {
    environment.systemPackages = [ cfg.package ];
    systemd.user.services.barricade-wallpaper = lib.mkIf cfg.autostart {
      description = "Barricade Bar";
      wantedBy = [ "graphical-session.target" ];
      partOf = [ "graphical-session.target" ];
      after = [ "graphical-session.target" ];
      serviceConfig = {
        ExecStart = lib.getExe cfg.package;
        Restart = "on-failure";
        RestartSec = 2;
      };
    };
  };
}
