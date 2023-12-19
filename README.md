Drop down terminal inspired by Tilda.

(GTK3 flavor: [hkoosha/zoha](https://github.com/hkoosha/zoha))

```
$ zoha -h
A drop down terminal inspired by Tilda

Usage: zoha [OPTIONS]

Options:
  -c, --cfg-file CFG_FILE      Override location of config file
  -k, --keypress-grabber       Disable listening on dbus for keypress
      --list-key-grabber-keys  List keys accepted by keypress grabber
  -s, --signal                 Signal Zoha to toggle visibility and exit
      --list-monitors          List monitors and exit
  -q, --quiet                  Do not print hints
      --dry-run                Sanitize configuration, print any errors and exit
  -h, --help                   Print help
  -V, --version                Print version
```

# What is Zoha?

Zoha is a terminal emulator based on GTK's VTE. Unlike others however, it can be pulled down from
top of the screen with a click of a button.

# Hotkey to Toggle Visibility

Zoha connects to dbus, and listens for signals to toggle its visibility. A dbus signal can be
broadcast by issuing the following command:


```bash
zoha -s
```

Assign this command to a keyboard shortcut through your window manager, and you're all set!

### Hack to Use Zoha Without DBus

Zoha also comes with a hack, that listens to key presses of the keyboard and toggles visibility
if the configured shortcuts are pressed. This is a hack though, and more info can be found by
launching zoha with the hack:

```bash
zoha -k
```

# Installing

Generate and launch the binary by following commands, you need to have Rust installed:

```bash
cargo build -r
./target/release/zoha

# And to send the DBus signal:
./target/release/zoha -s
```

You can install to `/usr/bin/` by:

```bash
sudo make install
```

Don't forget to assign `zoha -s` to your desired shortcut in your window manager's shortcuts config.

# Configuration

Zoha has a set of configuration all with default values. The default value can be overridden in the
config file at `$HOME/.zoha.toml` or by launching zoha and naming an alternative config file:

```bash
zoha -c 'alt/path/to/config/file.toml'
```

If the configuration file fails to parse, zoha falls back to default values so that you can launch
the terminal anyway and inspect the config file.

The configuration format is TOML and looks like this:

```toml
[process]
# nothing here yet.

[keys]
quit = "Shift-Ctrl-Alt-q"
tab_add = "Shift-Ctrl-t"
tab_close = "Shift-Ctrl-w"

[display]
# start_hidden = true
tab_mode = "Auto"

[color]
pallet = "SolarizedDark"
bg = "rgba(0,0,0,0.8)"
```

# Configuration Values

### [process]

| Config          | Default        | Type   | Notes                                                                                                                        |
|-----------------|----------------|--------|------------------------------------------------------------------------------------------------------------------------------|
| application\_id | io.koosha.zoha | string | The GTK application id, not much point in changing this.                                                                     |
| command         | *varies*       | string | The shell to use inside the terminal. Must be path of an executable, defaults to which ever is found first: bash, zsh, fish. |

### [font]

| Config | Default | Type   | Notes                                                |
|--------|---------|--------|------------------------------------------------------|
| font   | font    | string | Name of font to use, must be a Pango parsable value. |
| size   | 16      | u8     | Font size.                                           |

### [color]

| Config    | Default                  | Type        | Notes                                                                                                             |
|-----------|--------------------------|-------------|-------------------------------------------------------------------------------------------------------------------|
| bg        | rgba(0, 0, 0, 0.1)       | RGBA string | Background color, must be parsable as GTK RGBA.                                                                   |
| fg        | rgba(255, 255, 255, 0.1) | RGBA string | Foreground color, must be parsable as GTK RGBA.                                                                   |
| cursor    | rgba(0, 0 ,0, 0.1)       | RGBA string | Cursor color, must be parsable as GTK RGBA.                                                                       |
| pallet    | Tango                    | enum        | Default color pallet; possible values: Tango, Zenburn, Linux, XTerm, RXVT, SolarizedLight, SolarizedDark, Snazzy. |
| color\_00 | *Empty*                  | RGBA string | Override of color 0 in the pallet, must be parsable as GTK RGBA.                                                  |
| color\_01 | *Empty*                  | RGBA string | Override of color 1 in the pallet, must be parsable as GTK RGBA.                                                  |
| color\_02 | *Empty*                  | RGBA string | Override of color 2 in the pallet, must be parsable as GTK RGBA.                                                  |
| color\_03 | *Empty*                  | RGBA string | Override of color 3 in the pallet, must be parsable as GTK RGBA.                                                  |
| color\_04 | *Empty*                  | RGBA string | Override of color 4 in the pallet, must be parsable as GTK RGBA.                                                  |
| color\_05 | *Empty*                  | RGBA string | Override of color 5 in the pallet, must be parsable as GTK RGBA.                                                  |
| color\_06 | *Empty*                  | RGBA string | Override of color 6 in the pallet, must be parsable as GTK RGBA.                                                  |
| color\_07 | *Empty*                  | RGBA string | Override of color 7 in the pallet, must be parsable as GTK RGBA.                                                  |
| color\_08 | *Empty*                  | RGBA string | Override of color 8 in the pallet, must be parsable as GTK RGBA.                                                  |
| color\_09 | *Empty*                  | RGBA string | Override of color 9 in the pallet, must be parsable as GTK RGBA.                                                  |
| color\_10 | *Empty*                  | RGBA string | Override of color 10 in the pallet, must be parsable as GTK RGBA.                                                 |
| color\_11 | *Empty*                  | RGBA string | Override of color 11 in the pallet, must be parsable as GTK RGBA.                                                 |
| color\_12 | *Empty*                  | RGBA string | Override of color 12 in the pallet, must be parsable as GTK RGBA.                                                 |
| color\_13 | *Empty*                  | RGBA string | Override of color 13 in the pallet, must be parsable as GTK RGBA.                                                 |
| color\_14 | *Empty*                  | RGBA string | Override of color 14 in the pallet, must be parsable as GTK RGBA.                                                 |
| color\_15 | *Empty*                  | RGBA string | Override of color 15 in the pallet, must be parsable as GTK RGBA.                                                 |


### [display]

| Config                   | Default | Type   | Notes                                                                                                             |
|--------------------------|---------|--------|-------------------------------------------------------------------------------------------------------------------|
| monitor                  | *Empty* | string | Monitor to display Zoha on, you can get a list of available monitors by `zoha --list-monitors`.                   |
| title                    | Zoha    | string | Application's title.                                                                                              |
| width                    | *Empty* | u32    | Height of Zoha's window in pixels.                                                                                |
| height                   | *Empty* | u32    | Height of Zoha's window in pixels.                                                                                |
| width\_percentage        | 100     | u8     | Percentage of monitors width to use as Zoha's window width. Overrides `width` property if specified.              |
| width\_percentage        | 100     | u8     | Percentage of monitors height to use as Zoha's window width. Overrides `height` property if specified.            |
| start_hidden             | false   | bool   | Hide Zoha on initial startup.                                                                                     |
| skip_taskbar             | true    | bool   | Do not show Zoha in taskbar.                                                                                      |
| always_on_top            | true    | bool   | Always on top of other windows.                                                                                   |
| sticky                   | true    | bool   | Always on all desktops.                                                                                           |
| fullscreen               | true    | bool   | Fullscreen.                                                                                                       |
| tab_scroll_wrap          | true    | bool   | When scrolling tabs with the configured shortcut, wrap when reaching the end / beginning.                         |
| tab_mode                 | Auto    | enum   | When to show tabs. "Auto" will show tabs if there are more than one tab; possible values: Auto, Never, Always.    |
| tab_position             | Top     | enum   | Where to show tabs; possible values: Left, Right, Top, Bottom.                                                    |
| tab_expand               | false   | bool   | Expand the tab bar to full window's width.                                                                        |
| tab_title_num_characters | 25      | i8     | Number of characters to show in tab. Negative values trims the title from right, positive values trims from left. |
| scrollbar_position       | Hidden  | enum   | Possible values: Left, Right, Hidden.                                                                             |

### [key]

| Config              | Default              | Type   | Notes                                                                                  |
|---------------------|----------------------|--------|----------------------------------------------------------------------------------------|
| copy                | Ctrl-Shift-C         | string | Copy shortcut; valid GTK accelerator specification.                                    |
| paste               | Ctrl-Shift-V         | string | Paste shortcut; valid GTK accelerator specification.                                   |
| quit                | Ctrl-Shift-Alt-q     | string | App exit shortcut; valid GTK accelerator specification.                                |
| transparency_toggle | F12                  | string | Temporarily toggle transparency; valid GTK accelerator specification.                  |
| tab_add             | Ctrl-Shift-T         | string | Add a new tab next to current tab; valid GTK accelerator specification.                |
| tab_close           | Ctrl-Shift-W         | string | Close current tab; valid GTK accelerator specification.                                |
| tab_move_backward   | Ctrl-Shift-Page_Up   | string | Move the active tab forward among all tabs; valid GTK accelerator specification.       |
| tab_move_forward    | Ctrl-Shift-Page_Down | string | Move the active tab backward among all tabs; valid GTK accelerator specification.      |
| tab_goto_next       | Ctrl-Page_Down       | string | Goto next tab; valid GTK accelerator specification.                                    |
| tab_move_previous   | Ctrl-Page_Down       | string | Goto previous tab; valid GTK accelerator specification.                                |
| tab_goto_last       | Ctrl-Alt-9           | string | Goto last tab; valid GTK accelerator specification.                                    |
| tab_goto_01         | Ctrl-Alt-1           | string | Goto tab number 1; valid GTK accelerator specification.                                |
| tab_goto_02         | Ctrl-Alt-2           | string | Goto tab number 2; valid GTK accelerator specification.                                |
| tab_goto_03         | Ctrl-Alt-3           | string | Goto tab number 3; valid GTK accelerator specification.                                |
| tab_goto_04         | Ctrl-Alt-4           | string | Goto tab number 4; valid GTK accelerator specification.                                |
| tab_goto_05         | Ctrl-Alt-5           | string | Goto tab number 5; valid GTK accelerator specification.                                |
| tab_goto_06         | Ctrl-Alt-6           | string | Goto tab number 6; valid GTK accelerator specification.                                |
| tab_goto_07         | Ctrl-Alt-7           | string | Goto tab number 7; valid GTK accelerator specification.                                |
| tab_goto_08         | Ctrl-Alt-8           | string | Goto tab number 8; valid GTK accelerator specification.                                |
| font_size_inc       | Ctrl-Alt-Equal       | string | Temporarily increase font size on all tabs; valid GTK accelerator specification.       |
| font_size_dec       | Ctrl-Alt-Equal       | string | Temporarily decrease font size on all tabs; valid GTK accelerator specification.       |
| font_size_reset     | Ctrl-Alt-Equal       | string | Reset font size on all tabs to the initial value; valid GTK accelerator specification. |

### [terminal]

| Config               | Default             | Type   | Notes                                                                                                           |
|----------------------|---------------------|--------|-----------------------------------------------------------------------------------------------------------------|
| allow_hyperlink      | true                | bool   | Refer to GTK VTE for more information.                                                                          |
| allow_bold           | true                | bool   | Refer to GTK VTE for more information.                                                                          |
| audible_bell         | false               | bool   | Refer to GTK VTE for more information.                                                                          |
| cursor_blink         | false               | bool   | Refer to GTK VTE for more information.                                                                          |
| cursor_shape         | Block               | enum   | Refer to GTK VTE for more information; possible values: Block, IBeam, Underline.                                |
| scroll_on_output     | false               | bool   | Refer to GTK VTE for more information.                                                                          |
| scroll_on_keystroke  | true                | bool   | Refer to GTK VTE for more information.                                                                          |
| mouse_auto_hide      | false               | bool   | Refer to GTK VTE for more information.                                                                          |
| scrollback_lines     | -1                  | i64    | Refer to GTK VTE for more information.                                                                          |
| backspace_binding    | Auto                | enum   | Refer to GTK VTE for more information; possible values: Auto, AsciiBackspace, AsciiDelete, DeleteSequence, Tty. |
| delete_binding       | Auto                | enum   | Refer to GTK VTE for more information; possible values: Auto, AsciiBackspace, AsciiDelete, DeleteSequence, Tty. |
| word_char_exceptions | -A-Za-z0-9,./?%&#:_ | string | Refer to GTK VTE for more information.                                                                          |

### [behavior]

| Config                 | Default         | Type | Notes                                                                                                            |
|------------------------|-----------------|------|------------------------------------------------------------------------------------------------------------------|
| terminal_exit_behavior | ExitTerminal    | enum | What happens when a VTE terminal exits; possible values: ExitTerminal; TODO: RestartCommand, DropToDefaultShell. |
| last_tab_exit_behavior | RestartTerminal | enum | What happens when last tab exits or is closed; possible values: RestartTerminal, RestartTerminalAndHide, Exit.   |

### [hack]

| Config | Default | Type                       | Notes                                                                                |
|--------|---------|----------------------------|--------------------------------------------------------------------------------------|
| toggle | F1      | string list (of key-codes) | Key-code sequence to toggle visibility. Zoha must run with `-k` flag to take effect. |

Note: Run Zoha with `--list-key-grabber-keys` to print a list of keys accepted by the toggle config above.
Please note: the key sequence specification **IS DIFFERENT** from shortcut specification above.
The shortcuts take GTK accelerator specification, but `toggle` takes Rust's device_query crate
specific keys.

Example value for `toggle`:
```toml
toggle = ["LControl", "LShift", "F1"]
```

# TODO

- Test multi monitor setup.
- Config TUI.
- ~~Steal~~ borrow a proper key-grabber from Tomboy Notes (and Tilda).
- (Maybe) direct config modification of supported window manager / supported version for adding
  toggle shortcut.
- Show a dialog on config error.
- Custom gtk CSS.
- Extra config: New tab's add position.
- Extra config: Show scrollbar on mouse.
- Config: terminal_exit_behavior.
- Config: prompt_on_exit.
- Config: working_dir.
- Update VTE in the original crate (vte-rs) and deprecate ours (zoha-vte-rs).
- Fix regex in vte-rs (or zoha-vte-rs).

# Troubleshooting

### Zoha immediately hides after toggling visibility, or doesn't show at all

Did you run Zoha as `zoha -k` and also configured a window manager shortcut? This will cause Zoha
to receive 2 signals and immediately hides after showing up.

### My config does not take effect

Launch Zoha in dry-run mode and observe the output. Is there any configuration errors?

```bash
zoha --dry-run
# OR
zoha --dry-run -k
```
