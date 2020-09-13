initSidebarItems({"mod":[["account","Request access to the current logged user information such as the id, name or their avatar uri.  # Examples"],["background","Request running an application in the background. # Examples"],["camera","Check if a camera is available, request access to it and open a pipewire remote stream. # Examples"],["device","Request access to specific devices such as camera, speakers or microphone. # Examples"],["email","Compose an email. # Examples"],["file_chooser","Open/save file(s) chooser.  # Examples"],["game_mode","Enable/disable/query the status of Game Mode. ```no_run use ashpd::desktop::game_mode::{GameModeProxy, GameModeStatus}; use zbus::{self, fdo::Result};"],["inhibit","Inhibit the session from being restarted or the user from logging out."],["location","Query the user's GPS location. # Examples"],["memory_monitor","Monitor memory level. # Examples"],["network_monitor","Check the status of the network on a user's machine. # Examples"],["notification","Send/withdraw notifications.  # Examples"],["open_uri","Open a file or a directory.  # Examples"],["print","Print a docucment. # Examples"],["remote_desktop","Start a remote desktop session and interact with it."],["screencast","Start a screencast session and get the pipewire remote of it. # Examples"],["screenshot","Take a screenshot or pick a color.  # Examples"],["secret","Retrieve a per-application secret used to encrypt confedential data inside the sandbox. # Examples"],["settings","Read & listen to system settings changes. ```no_run use ashpd::desktop::settings::SettingsProxy; use zbus::{self, fdo::Result};"],["trash","Move a file to the trash. # Examples"],["wallpaper","Set a wallpaper on lockscreen, background or both. # Examples"]]});