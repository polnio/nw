self:
{
  lib,
  pkgs,
  config,
  ...
}:
let
  inherit (lib) types;
  inherit (lib.modules) mkIf;
  inherit (lib.options) mkOption mkEnableOption literalExpression;

  tomlFormat = pkgs.formats.toml { };

  package = self.packages.${pkgs.system}.default;
  cfg = config.programs.nw;
in
{
  options.programs.nw = {
    enable = mkEnableOption "nw";

    package = mkOption {
      type = types.package;
      default = package.override { withUI = cfg.withUI; };
      defaultText = literalExpression "inputs.nw.packages.${pkgs.system}.default";
      description = "The package to use.";
    };

    withUI = mkOption {
      type = types.bool;
      default = false;
      description = "Whether to include the UI.";
    };

    settings = mkOption {
      type = types.attrs;
      default = { };
      description = "The settings to use.";
    };
  };

  config = mkIf cfg.enable {
    home.packages = [ cfg.package ];
    xdg.configFile."nw/config.toml" = mkIf (cfg.settings != { }) {
      source = tomlFormat.generate "nw-config" cfg.settings;
    };
  };
}
