self:
{
  config,
  lib,
  pkgs,
  ...
}:

let
  cfg = config.services.bitgateway;
in
{
  options.services.bitgateway = {
    enable = lib.mkEnableOption "BITGATEWAY desktop client";

    package = lib.mkOption {
      type = lib.types.package;
      default = self.packages.${pkgs.stdenv.hostPlatform.system}.default;
      description = "The BITGATEWAY package to use.";
    };

    autoStart = lib.mkOption {
      type = lib.types.bool;
      default = false;
      description = "Start BITGATEWAY automatically with the graphical session.";
    };

    silentStart = lib.mkOption {
      type = lib.types.bool;
      default = false;
      description = "Start BITGATEWAY with its window hidden and only the tray icon visible.";
    };
  };

  config = lib.mkMerge [
    (lib.mkIf cfg.enable {
      environment.systemPackages = [ cfg.package ];
    })

    (lib.mkIf cfg.autoStart {
      assertions = [
        {
          assertion = cfg.enable;
          message = "services.bitgateway.autoStart requires services.bitgateway.enable";
        }
      ];

      systemd.user.services.bitgateway = {
        description = "BIT SRUN gateway client";
        after = [ "graphical-session.target" ];
        partOf = [ "graphical-session.target" ];
        wantedBy = [ "graphical-session.target" ];

        serviceConfig = {
          ExecStart = lib.escapeShellArgs (
            [ (lib.getExe cfg.package) ] ++ lib.optional cfg.silentStart "--silent"
          );
          Restart = "on-failure";
          RestartSec = 3;
        };
      };
    })
  ];
}
