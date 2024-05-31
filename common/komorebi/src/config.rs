pub const KOMOREBI_CONFIG: &str = r###"
{
  "$schema": "https://raw.githubusercontent.com/LGUG2Z/komorebi/v0.1.25/schema.json",
"app_specific_configuration_path": "$APPLICATIONS_CONFIG_PATH",
"window_hiding_behaviour": "Cloak",
"cross_monitor_move_behaviour": "Insert",
"default_workspace_padding": 0,
"default_container_padding": 0,
"border_width": 1,
"border_offset": 0,
"active_window_border": true,
"mouse_follows_focus": false,
"active_window_border_colours": {
"single": "#ff0000",
"stack": "#00a542",
"monocle": "#ff3399"
},
"stackbar": {
"height": 4,
"mode": "Never",
"tabs": {
"width": 300,
"focused_text": "#00a542",
"unfocused_text": "#b3b3b3",
"background": "#141414"
}
},
"monitors": [
{
"workspaces": [
{
"name": "I",
"layout": "BSP"
},
{
"name": "II",
"layout": "VerticalStack"
},
{
"name": "III",
"layout": "HorizontalStack"
},
{
"name": "IV",
"layout": "UltrawideVerticalStack"
},
{
"name": "V",
"layout": "Rows"
},
{
"name": "VI",
"layout": "Grid"
},
{
"name": "VII",
"layout": "RightMainVerticalStack"
}
]
}
]
}"###;