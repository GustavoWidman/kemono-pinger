{ self, ... }:

{
  flake.nixosModules.default =
    {
      config,
      lib,
      pkgs,
      ...
    }:
    let
      cfg = config.services.kemono-pinger;

      toml = pkgs.formats.toml { };

      configFile =
        if cfg.config == null then
          null
        else if lib.isPath cfg.config || lib.isString cfg.config then
          cfg.config
        else
          toml.generate "config.toml" cfg.config;
    in
    {
      options.services.kemono-pinger = {
        enable = lib.mkEnableOption "enable kemono-pinger service";

        package = lib.mkOption {
          type = lib.types.package;
          default = self.packages.${pkgs.system}.default;
          description = "kemono-pinger package to use";
        };

        config = lib.mkOption {
          type =
            with lib.types;
            oneOf [
              str
              path
              toml.type
              null
            ];
          description = "configuration settings for kemono-pinger. Also accepts paths (string or path type) to a config file.";
        };
      };

      config = lib.mkIf cfg.enable {
        systemd.services.kemono-pinger = {
          description = "Kemono Pinger Service";
          wantedBy = [ "multi-user.target" ];
          after = [ "network.target" ];

          serviceConfig = {
            Type = "simple";
            ExecStart = "${cfg.package}/bin/kemono-pinger --config ${lib.escapeShellArg configFile}";
            Restart = "on-failure";
            RestartSec = "5s";

            NoNewPrivileges = true;
            PrivateTmp = true;
            ProtectSystem = "strict";
            ProtectHome = true;
          };
        };
      };
    };

}
