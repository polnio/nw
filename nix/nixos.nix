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

  packages = self.packages.${pkgs.system};
  cfg = config.programs.nw;
in
{
  options.programs.nw = {
    enable = mkEnableOption "nw";

    package = mkOption {
      type = types.package;
      default = if cfg.withUi then packages.with-ui else packages.default;
      defaultText = literalExpression "inputs.nw.packages.${pkgs.system}.default";
      description = "The package to use.";
    };

    withUi = mkOption {
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
    environment.systemPackages = [ cfg.package ];
    environment.etc."xdg/nw/config.toml" = mkIf (cfg.settings != { }) {
      source = tomlFormat.generate "nw-config" cfg.settings;
    };
  };
}
