/// Collection of system paths which we are interested about.
/// NOTE: File::open does not expand '~', so it's safer to specify the full path!
pub struct SystemAtlas<'a> {
    pub alacritty: &'a str,
    pub fontconfig: &'a str,
    pub eww_brightness: &'a str,
    pub eww_gamma: &'a str,
    pub eww_volume: &'a str,
    pub eww_workspaces: &'a str,
    pub hyprland: &'a str,
}

pub const SYSTEM_ATLAS: SystemAtlas = SystemAtlas {
    alacritty: "/home/rg/.config/alacritty/alacritty.yaml",
    fontconfig: "/home/rg/.config/fontconfig/fonts.conf",
    eww_brightness: "/tmp/eww-brightness",
    eww_gamma: "/tmp/eww-gamma",
    eww_volume: "/tmp/eww-volume",
    eww_workspaces: "/tmp/eww-workspaces",
    hyprland: "/home/rg/.config/hypr/hyprland.conf",
};
