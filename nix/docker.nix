{
  lib,
  dockerTools,
  warden,
}:
dockerTools.buildLayeredImage {
  name = "warden";
  tag = "latest";

  contents = [ dockerTools.binSh ];

  config.Cmd = [ (lib.getExe warden) ];
}
