pub const KOMOREBI_APPLICATIONS_CONFIG: &str = "
- name: 1Password
  identifier:
    kind: Exe
    id: Guitar Rig 7.exe
    matching_strategy: Equals
  float_identifiers:
  - kind: Exe
    id: Guitar Rig 7.exe
    matching_strategy: Equals
- name: 1Password
  identifier:
    kind: Exe
    id: 1Password.exe
    matching_strategy: Equals
  float_identifiers:
  - kind: Exe
    id: 1Password.exe
    matching_strategy: Equals
- name: Ableton Live
  identifier:
    kind: Class
    id: Ableton Live Window Class
    matching_strategy: Legacy
  float_identifiers:
  - kind: Class
    id: AbletonVstPlugClass
    matching_strategy: Legacy
  - kind: Class
    id: Vst3PlugWindow
    matching_strategy: Legacy
- name: Adobe Creative Cloud
  identifier:
    kind: Class
    id: CreativeCloudDesktopWindowClass
    matching_strategy: Legacy
  options:
  - tray_and_multi_window
- name: Adobe Photoshop
  identifier:
    kind: Class
    id: Photoshop
    matching_strategy: Legacy
- name: Adobe Premiere Pro
  identifier:
    kind: Class
    id: Premiere Pro
    matching_strategy: Legacy
  float_identifiers:
  - kind: Class
    id: DroverLord - Window Class
    matching_strategy: Equals
- name: Affinity Photo 2
  identifier:
    kind: Title
    id: Affinity Photo 2
    matching_strategy: Legacy
  options:
  - force
  float_identifiers:
  - kind: Exe
    id: Photo.exe
    matching_strategy: Equals
- name: Akiflow
  identifier:
    kind: Exe
    id: Akiflow.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
- name: Android Studio
  identifier:
    kind: Exe
    id: studio64.exe
    matching_strategy: Equals
  options:
  - object_name_change
- name: Anki
  identifier:
    kind: Exe
    id: anki.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
- name: ArmCord
  identifier:
    kind: Exe
    id: ArmCord.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
- name: AutoHotkey
  identifier:
    kind: Exe
    id: AutoHotkeyU64.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
  float_identifiers:
  - kind: Title
    id: Window Spy
    matching_strategy: StartsWith
  - kind: Exe
    id: AutoHotkeyUX.exe
    matching_strategy: Equals
- name: Beeper
  identifier:
    kind: Exe
    id: Beeper.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
- name: Bitwarden
  identifier:
    kind: Exe
    id: Bitwarden.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
- name: Blitz
  identifier:
    kind: Exe
    id: Blitz.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
- name: Bloxstrap
  identifier:
    kind: Exe
    id: Bloxstrap.exe
    matching_strategy: Equals
  float_identifiers:
  - kind: Exe
    id: Bloxstrap.exe
    matching_strategy: Equals
- name: Brave Browser
  identifier:
    kind: Exe
    id: brave.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
- name: CLion
  identifier:
    kind: Exe
    id: clion64.exe
    matching_strategy: Equals
  options:
  - object_name_change
  - tray_and_multi_window
  float_identifiers:
  - kind: Class
    id: SunAwtDialog
    matching_strategy: Equals
- name: Calculator
  identifier:
    kind: Title
    id: Calculator
    matching_strategy: Equals
  float_identifiers:
  - kind: Title
    id: Calculator
    matching_strategy: Equals
- name: Citrix Receiver
  identifier:
    kind: Exe
    id: SelfService.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
  float_identifiers:
  - kind: Exe
    id: SelfService.exe
    matching_strategy: Equals
- name: Clash Verge
  identifier:
    kind: Exe
    id: Clash Verge.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
- name: Clementine
  identifier:
    kind: Exe
    id: clementine.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
- name: CopyQ
  identifier:
    kind: Exe
    id: copyq.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
- name: Credential Manager UI Host
  identifier:
    kind: Exe
    id: CredentialUIBroker.exe
    matching_strategy: Equals
  float_identifiers:
  - kind: Exe
    id: CredentialUIBroker.exe
    matching_strategy: Equals
- name: Cron
  identifier:
    kind: Exe
    id: Cron.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
- name: DS4Windows
  identifier:
    kind: Exe
    id: DS4Windows.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
- name: Delphi applications
  identifier:
    kind: Class
    id: TApplication
    matching_strategy: Legacy
  float_identifiers:
  - kind: Class
    id: TApplication
    matching_strategy: Legacy
  - kind: Class
    id: TWizardForm
    matching_strategy: Legacy
- name: Discord
  identifier:
    kind: Exe
    id: Discord.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
  - layered
- name: Discord Bot Client
  identifier:
    kind: Exe
    id: DiscordBotClient.exe
    matching_strategy: Equals
- name: DiscordCanary
  identifier:
    kind: Exe
    id: DiscordCanary.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
- name: DiscordDevelopment
  identifier:
    kind: Exe
    id: DiscordDevelopment.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
- name: DiscordPTB
  identifier:
    kind: Exe
    id: DiscordPTB.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
- name: Docker Desktop
  identifier:
    kind: Exe
    id: Docker Desktop.exe
    matching_strategy: Equals
- name: Dropbox
  identifier:
    kind: Exe
    id: Dropbox.exe
    matching_strategy: Equals
  float_identifiers:
  - kind: Exe
    id: Dropbox.exe
    matching_strategy: Equals
- name: EA Desktop Client
  identifier:
    kind: Exe
    id: EADesktop.exe
    matching_strategy: Equals
- name: Eagle
  identifier:
    kind: Exe
    id: Eagle.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
- name: ElectronMail
  identifier:
    kind: Exe
    id: ElectronMail.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
- name: Element
  identifier:
    kind: Exe
    id: Element.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
- name: Elephicon
  identifier:
    kind: Exe
    id: Elephicon.exe
    matching_strategy: Equals
  float_identifiers:
  - kind: Exe
    id: Elephicon.exe
    matching_strategy: Equals
- name: ElevenClock
  identifier:
    kind: Exe
    id: ElevenClock.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
- name: Elgato Camera Hub
  identifier:
    kind: Exe
    id: Camera Hub.exe
    matching_strategy: Equals
  float_identifiers:
  - kind: Exe
    id: Camera Hub.exe
    matching_strategy: Equals
- name: Elgato Control Center
  identifier:
    kind: Exe
    id: ControlCenter.exe
    matching_strategy: Equals
  float_identifiers:
  - kind: Exe
    id: ControlCenter.exe
    matching_strategy: Equals
- name: Elgato Wave Link
  identifier:
    kind: Exe
    id: WaveLink.exe
    matching_strategy: Equals
  float_identifiers:
  - kind: Exe
    id: WaveLink.exe
    matching_strategy: Equals
- name: Epic Games Launcher
  identifier:
    kind: Exe
    id: EpicGamesLauncher.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
- name: Everything
  identifier:
    kind: Class
    id: EVERYTHING
    matching_strategy: Legacy
  options:
  - tray_and_multi_window
- name: Everything1.5a
  identifier:
    kind: Class
    id: EVERYTHING_(1.5a)
    matching_strategy: Legacy
  options:
  - force
  - tray_and_multi_window
- name: FFMetrics
  identifier:
    kind: Exe
    id: FFMetrics.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
- name: Figma
  identifier:
    kind: Exe
    id: Figma.exe
    matching_strategy: Equals
- name: Files
  identifier:
    kind: Exe
    id: Files.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
- name: Flow Launcher
  identifier:
    kind: Exe
    id: Flow.Launcher.exe
    matching_strategy: Equals
- name: GOG Galaxy
  identifier:
    kind: Exe
    id: GalaxyClient.exe
    matching_strategy: Equals
  options:
  - force
  - tray_and_multi_window
  float_identifiers:
  - kind: Class
    id: Chrome_RenderWidgetHostHWND
    matching_strategy: Legacy
- name: GitHub Credential Manager
  identifier:
    kind: Exe
    id: git-credential-manager.exe
    matching_strategy: Equals
  float_identifiers:
  - kind: Exe
    id: git-credential-manager.exe
    matching_strategy: Equals
- name: GitHub Desktop
  identifier:
    kind: Exe
    id: GitHubDesktop.exe
    matching_strategy: Equals
- name: GoPro Webcam
  identifier:
    kind: Class
    id: GoPro Webcam
    matching_strategy: Legacy
  options:
  - tray_and_multi_window
- name: Godot Manager
  identifier:
    kind: Exe
    id: GodotManager.exe
    matching_strategy: Equals
  options:
  - force
  - object_name_change
- name: Golden Dict
  identifier:
    kind: Exe
    id: GoldenDict.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
- name: Google Chrome
  identifier:
    kind: Exe
    id: chrome.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
- name: Google Drive
  identifier:
    kind: Exe
    id: GoogleDriveFS.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
  float_identifiers:
  - kind: Exe
    id: GoogleDriveFS.exe
    matching_strategy: Equals
- name: Honeyview
  identifier:
    kind: Class
    id: HoneyviewClassX
    matching_strategy: Legacy
- name: Houdoku
  identifier:
    kind: Exe
    id: Houdoku.exe
    matching_strategy: Equals
- name: IntelliJ IDEA
  identifier:
    kind: Exe
    id: idea64.exe
    matching_strategy: Equals
  options:
  - object_name_change
  - tray_and_multi_window
  float_identifiers:
  - kind: Class
    id: SunAwtDialog
    matching_strategy: Equals
- name: Itch.io
  identifier:
    kind: Exe
    id: itch.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
- name: KOOK
  identifier:
    kind: Exe
    id: KOOK.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
- name: Keyviz
  identifier:
    kind: Exe
    id: keyviz.exe
    matching_strategy: Equals
  float_identifiers:
  - kind: Exe
    id: keyviz.exe
    matching_strategy: Equals
- name: Kleopatra
  identifier:
    kind: Exe
    id: kleopatra.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
- name: Kotatogram
  identifier:
    kind: Exe
    id: Kotatogram.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
- name: LocalSend
  identifier:
    kind: Exe
    id: localsend_app.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
- name: Logi Bolt
  identifier:
    kind: Exe
    id: LogiBolt.exe
    matching_strategy: Equals
  float_identifiers:
  - kind: Exe
    id: LogiBolt.exe
    matching_strategy: Equals
- name: LogiTune
  identifier:
    kind: Exe
    id: LogiTune.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
  float_identifiers:
  - kind: Exe
    id: LogiTune.exe
    matching_strategy: Equals
- name: Logitech G HUB
  identifier:
    kind: Exe
    id: lghub.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
- name: Logitech Options
  identifier:
    kind: Exe
    id: LogiOptionsUI.exe
    matching_strategy: Equals
  float_identifiers:
  - kind: Exe
    id: LogiOptionsUI.exe
    matching_strategy: Equals
- name: Mailspring
  identifier:
    kind: Exe
    id: mailspring.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
- name: ManicTime
  identifier:
    kind: Exe
    id: ManicTimeClient.exe
    matching_strategy: Equals
  options:
  - force
  - object_name_change
  - tray_and_multi_window
- name: ManyCam
  identifier:
    kind: Exe
    id: ManyCam.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
- name: Mattermost
  identifier:
    kind: Exe
    id: Mattermost.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
- name: Mica For Everyone
  identifier:
    kind: Exe
    id: MicaForEveryone.exe
    matching_strategy: Equals
- name: Microsoft Active Accessibility
  identifier:
    kind: Class
    id: ''
    matching_strategy: Legacy
  float_identifiers:
  - kind: Class
    id: '#32770'
    matching_strategy: Legacy
- name: Microsoft Excel
  identifier:
    kind: Exe
    id: EXCEL.EXE
    matching_strategy: Equals
  options:
  - layered
  float_identifiers:
  - kind: Class
    id: _WwB
    matching_strategy: Legacy
- name: Microsoft Outlook
  identifier:
    kind: Exe
    id: OUTLOOK.EXE
    matching_strategy: Equals
  options:
  - layered
  - tray_and_multi_window
  float_identifiers:
  - kind: Class
    id: _WwB
    matching_strategy: Legacy
- name: Microsoft PC Manager
  identifier:
    kind: Exe
    id: MSPCManager.exe
    matching_strategy: Equals
  float_identifiers:
  - kind: Exe
    id: MSPCManager.exe
    matching_strategy: Equals
- name: Microsoft PowerPoint
  identifier:
    kind: Exe
    id: POWERPNT.EXE
    matching_strategy: Equals
  options:
  - layered
  float_identifiers:
  - kind: Class
    id: _WwB
    matching_strategy: Legacy
- name: Microsoft SQL Server Management Studio
  identifier:
    kind: Exe
    id: Ssms.exe
    matching_strategy: Equals
- name: Microsoft Teams
  identifier:
    kind: Class
    id: TeamsWebView
    matching_strategy: Legacy
  options:
  - tray_and_multi_window
- name: Microsoft Teams classic
  identifier:
    kind: Exe
    id: Teams.exe
    matching_strategy: Equals
  float_identifiers:
  - kind: Title
    id: Microsoft Teams Notification
    matching_strategy: Legacy
  - kind: Title
    id: Microsoft Teams Call
    matching_strategy: Legacy
- name: Microsoft Word
  identifier:
    kind: Exe
    id: WINWORD.EXE
    matching_strategy: Equals
  options:
  - layered
  float_identifiers:
  - kind: Class
    id: _WwB
    matching_strategy: Legacy
- name: Modern Flyouts
  identifier:
    kind: Exe
    id: ModernFlyoutsHost.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
- name: Morgen
  identifier:
    kind: Exe
    id: Morgen.exe
    matching_strategy: Equals
- name: Mozilla Firefox
  identifier:
    kind: Exe
    id: firefox.exe
    matching_strategy: Equals
  options:
  - object_name_change
  - tray_and_multi_window
  float_identifiers:
  - kind: Class
    id: MozillaTaskbarPreviewClass
    matching_strategy: Legacy
- name: MuseScore
  identifier:
    kind: Exe
    id: MuseScore.exe
    matching_strategy: Equals
- name: NVIDIA GeForce Experience
  identifier:
    kind: Exe
    id: NVIDIA GeForce Experience.exe
    matching_strategy: Equals
- name: NZXT CAM
  identifier:
    kind: Exe
    id: NZXT CAM.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
- name: NetEase Cloud Music
  identifier:
    kind: Exe
    id: cloudmusic.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
- name: NiceHash Miner
  identifier:
    kind: Exe
    id: nhm_app.exe
    matching_strategy: Equals
  options:
  - force
- name: NohBoard
  identifier:
    kind: Exe
    id: NohBoard.exe
    matching_strategy: Equals
  float_identifiers:
  - kind: Exe
    id: NohBoard.exe
    matching_strategy: Equals
- name: Notion Enhanced
  identifier:
    kind: Exe
    id: Notion Enhanced.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
- name: OBS Studio (32-bit)
  identifier:
    kind: Exe
    id: obs32.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
- name: OBS Studio (64-bit)
  identifier:
    kind: Exe
    id: obs64.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
- name: ONLYOFFICE Editors
  identifier:
    kind: Class
    id: DocEditorsWindowClass
    matching_strategy: Legacy
  options:
  - tray_and_multi_window
- name: Obsidian
  identifier:
    kind: Exe
    id: Obsidian.exe
    matching_strategy: Equals
- name: OneDrive
  identifier:
    kind: Exe
    id: OneDrive.exe
    matching_strategy: Equals
  float_identifiers:
  - kind: Class
    id: OneDriveReactNativeWin32WindowClass
    matching_strategy: Legacy
- name: OneQuick
  identifier:
    kind: Exe
    id: OneQuick.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
- name: OpenRGB
  identifier:
    kind: Exe
    id: OpenRGB.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
- name: Paradox Launcher
  identifier:
    kind: Exe
    id: Paradox Launcher.exe
    matching_strategy: Equals
  float_identifiers:
  - kind: Exe
    id: Paradox Launcher.exe
    matching_strategy: Equals
- name: Passware Kit Forensic
  identifier:
    kind: Exe
    id: PasswareKitForensic.exe
    matching_strategy: Equals
- name: Playnite
  identifier:
    kind: Exe
    id: Playnite.DesktopApp.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
  float_identifiers:
  - kind: Exe
    id: Playnite.FullscreenApp.exe
    matching_strategy: Equals
- name: Plexamp
  identifier:
    kind: Exe
    id: Plexamp.exe
    matching_strategy: Equals
- name: Postman
  identifier:
    kind: Exe
    id: Postman.exe
    matching_strategy: Equals
- name: PowerToys
  identifier:
    kind: Exe
    id: PowerToys.exe
    matching_strategy: Equals
  float_identifiers:
  - kind: Exe
    id: PowerToys.ColorPickerUI.exe
    matching_strategy: Equals
  - kind: Exe
    id: PowerToys.CropAndLock.exe
    matching_strategy: Equals
  - kind: Exe
    id: PowerToys.ImageResizer.exe
    matching_strategy: Equals
  - kind: Exe
    id: PowerToys.Peek.UI.exe
    matching_strategy: Equals
  - kind: Exe
    id: PowerToys.PowerLauncher.exe
    matching_strategy: Equals
  - kind: Exe
    id: PowerToys.PowerAccent.exe
    matching_strategy: Equals
- name: Process Hacker
  identifier:
    kind: Exe
    id: ProcessHacker.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
  float_identifiers:
  - kind: Exe
    id: ProcessHacker.exe
    matching_strategy: Equals
- name: ProtonDrive
  identifier:
    kind: Exe
    id: ProtonDrive.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
- name: ProtonVPN
  identifier:
    kind: Exe
    id: ProtonVPN.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
- name: PyCharm
  identifier:
    kind: Exe
    id: pycharm64.exe
    matching_strategy: Equals
  options:
  - object_name_change
  - tray_and_multi_window
  float_identifiers:
  - kind: Class
    id: SunAwtDialog
    matching_strategy: Equals
- name: QQ
  identifier:
    kind: Exe
    id: QQ.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
  float_identifiers:
  - kind: Title
    id: 图片查看器
    matching_strategy: Legacy
  - kind: Title
    id: 群聊的聊天记录
    matching_strategy: Legacy
  - kind: Title
    id: 语音通话
    matching_strategy: Legacy
- name: QtScrcpy
  identifier:
    kind: Exe
    id: QtScrcpy.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
- name: QuickLook
  identifier:
    kind: Exe
    id: QuickLook.exe
    matching_strategy: Equals
  float_identifiers:
  - kind: Exe
    id: QuickLook.exe
    matching_strategy: Equals
- name: RepoZ
  identifier:
    kind: Exe
    id: RepoZ.exe
    matching_strategy: Equals
  float_identifiers:
  - kind: Exe
    id: RepoZ.exe
    matching_strategy: Equals
- name: Rider
  identifier:
    kind: Exe
    id: rider64.exe
    matching_strategy: Equals
  options:
  - object_name_change
  - tray_and_multi_window
  float_identifiers:
  - kind: Class
    id: SunAwtDialog
    matching_strategy: Equals
  - kind: Title
    id: PopupMessageWindow
    matching_strategy: Legacy
- name: Roblox FPS Unlocker
  identifier:
    kind: Exe
    id: rbxfpsunlocker.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
- name: RoundedTB
  identifier:
    kind: Exe
    id: RoundedTB.exe
    matching_strategy: Equals
  float_identifiers:
  - kind: Exe
    id: RoundedTB.exe
    matching_strategy: Equals
- name: RoundedTB
  identifier:
    kind: Exe
    id: RoundedTB.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
- name: RustRover
  identifier:
    kind: Exe
    id: rustrover64.exe
    matching_strategy: Equals
  options:
  - object_name_change
  - tray_and_multi_window
  float_identifiers:
  - kind: Class
    id: SunAwtDialog
    matching_strategy: Equals
- name: Sandboxie Plus
  identifier:
    kind: Exe
    id: SandMan.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
- name: ShareX
  identifier:
    kind: Exe
    id: ShareX.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
- name: Sideloadly
  identifier:
    kind: Exe
    id: sideloadly.exe
    matching_strategy: Equals
  float_identifiers:
  - kind: Exe
    id: sideloadly.exe
    matching_strategy: Equals
- name: Signal
  identifier:
    kind: Exe
    id: Signal.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
- name: SiriKali
  identifier:
    kind: Exe
    id: sirikali.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
- name: Slack
  identifier:
    kind: Exe
    id: Slack.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
  float_identifiers:
  - kind: Class
    id: Chrome_RenderWidgetHostHWND
    matching_strategy: Legacy
- name: Slack
  identifier:
    kind: Exe
    id: slack.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
  float_identifiers:
  - kind: Class
    id: Chrome_RenderWidgetHostHWND
    matching_strategy: Legacy
- name: Smart Install Maker
  identifier:
    kind: Exe
    id: SIM.exe
    matching_strategy: Equals
  float_identifiers:
  - kind: Class
    id: obj_App
    matching_strategy: Legacy
  - kind: Class
    id: obj_Form
    matching_strategy: Legacy
- name: SnippingTool
  identifier:
    kind: Exe
    id: SnippingTool.exe
    matching_strategy: Equals
  float_identifiers:
  - kind: Exe
    id: SnippingTool.exe
    matching_strategy: Equals
- name: SoulseekQt
  identifier:
    kind: Exe
    id: SoulseekQt.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
- name: Spotify
  identifier:
    kind: Exe
    id: Spotify.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
- name: Steam
  identifier:
    kind: Class
    id: vguiPopupWindow
    matching_strategy: Legacy
- name: Steam Beta
  identifier:
    kind: Class
    id: SDL_app
    matching_strategy: Legacy
  options:
  - tray_and_multi_window
  float_identifiers:
  - kind: Title
    id: notificationtoasts_
    matching_strategy: Legacy
- name: Stremio
  identifier:
    kind: Exe
    id: stremio.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
- name: System Informer
  identifier:
    kind: Exe
    id: SystemInformer.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
  float_identifiers:
  - kind: Exe
    id: SystemInformer.exe
    matching_strategy: Equals
- name: SystemSettings
  identifier:
    kind: Exe
    id: SystemSettings.exe
    matching_strategy: Equals
  float_identifiers:
  - kind: Class
    id: Shell_Dialog
    matching_strategy: Legacy
- name: Task Manager
  identifier:
    kind: Exe
    id: Taskmgr.exe
    matching_strategy: Equals
  float_identifiers:
  - kind: Class
    id: TaskManagerWindow
    matching_strategy: Legacy
- name: Telegram
  identifier:
    kind: Exe
    id: Telegram.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
- name: TickTick
  identifier:
    kind: Exe
    id: TickTick.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
- name: Total Commander
  identifier:
    kind: Exe
    id: TotalCMD64.exe
    matching_strategy: Equals
  float_identifiers:
  - kind: Class
    id: TDLG2FILEACTIONMIN
    matching_strategy: Equals
- name: TouchCursor
  identifier:
    kind: Exe
    id: tcconfig.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
  float_identifiers:
  - kind: Exe
    id: tcconfig.exe
    matching_strategy: Equals
- name: TranslucentTB
  identifier:
    kind: Exe
    id: TranslucentTB.exe
    matching_strategy: Equals
  float_identifiers:
  - kind: Exe
    id: TranslucentTB.exe
    matching_strategy: Equals
- name: TranslucentTB
  identifier:
    kind: Exe
    id: TranslucentTB.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
- name: Unity Hub
  identifier:
    kind: Exe
    id: Unity Hub.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
- name: Unreal Editor
  identifier:
    kind: Exe
    id: UnrealEditor.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
- name: VMware Horizon Client
  identifier:
    kind: Exe
    id: vmware-view.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
- name: VRCX
  identifier:
    kind: Exe
    id: VRCX.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
- name: Visual Studio
  identifier:
    kind: Exe
    id: devenv.exe
    matching_strategy: Equals
  options:
  - object_name_change
- name: Visual Studio Code
  identifier:
    kind: Exe
    id: Code.exe
    matching_strategy: Equals
- name: Visual Studio Code - Insiders
  identifier:
    kind: Exe
    id: Code - Insiders.exe
    matching_strategy: Equals
- name: Voice.ai
  identifier:
    kind: Exe
    id: VoiceAI.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
- name: WebStorm
  identifier:
    kind: Exe
    id: webstorm64.exe
    matching_strategy: Equals
  options:
  - object_name_change
  - tray_and_multi_window
  float_identifiers:
  - kind: Class
    id: SunAwtDialog
    matching_strategy: Equals
- name: WebTorrent Desktop
  identifier:
    kind: Exe
    id: WebTorrent.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
- name: WinZip (32-bit)
  identifier:
    kind: Exe
    id: winzip32.exe
    matching_strategy: Equals
  float_identifiers:
  - kind: Exe
    id: winzip32.exe
    matching_strategy: Equals
- name: WinZip (64-bit)
  identifier:
    kind: Exe
    id: winzip64.exe
    matching_strategy: Equals
  float_identifiers:
  - kind: Exe
    id: winzip64.exe
    matching_strategy: Equals
- name: Windows Console (conhost.exe)
  identifier:
    kind: Class
    id: ConsoleWindowClass
    matching_strategy: Equals
  options:
  - force
- name: Windows Explorer
  identifier:
    kind: Exe
    id: explorer.exe
    matching_strategy: Equals
  float_identifiers:
  - kind: Class
    id: OperationStatusWindow
    matching_strategy: Legacy
  - kind: Title
    id: Control Panel
    matching_strategy: Legacy
- name: Windows Installer
  identifier:
    kind: Exe
    id: msiexec.exe
    matching_strategy: Equals
  float_identifiers:
  - kind: Exe
    id: msiexec.exe
    matching_strategy: Equals
- name: Windows Subsystem for Android
  identifier:
    kind: Exe
    id: WsaClient.exe
    matching_strategy: Equals
  float_identifiers:
  - kind: Class
    id: android(splash)
    matching_strategy: Legacy
- name: Windows Update Standalone Installer
  identifier:
    kind: Exe
    id: wusa.exe
    matching_strategy: Equals
  float_identifiers:
  - kind: Exe
    id: wusa.exe
    matching_strategy: Equals
- name: WingetUI
  identifier:
    kind: Exe
    id: WingetUI.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
- name: WingetUI
  identifier:
    kind: Exe
    id: wingetui.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
- name: Wox
  identifier:
    kind: Exe
    id: Wox.exe
    matching_strategy: Equals
  float_identifiers:
  - kind: Title
    id: Hotkey sink
    matching_strategy: Legacy
- name: XAMPP Control Panel
  identifier:
    kind: Exe
    id: xampp-control.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
- name: Zoom
  identifier:
    kind: Exe
    id: Zoom.exe
    matching_strategy: Equals
  float_identifiers:
  - kind: Exe
    id: Zoom.exe
    matching_strategy: Equals
- name: mpv
  identifier:
    kind: Class
    id: mpv
    matching_strategy: Legacy
  options:
  - object_name_change
- name: mpv.net
  identifier:
    kind: Exe
    id: mpvnet.exe
    matching_strategy: Equals
  options:
  - object_name_change
- name: paint.net
  identifier:
    kind: Exe
    id: paintdotnet.exe
    matching_strategy: Equals
  float_identifiers:
  - kind: Exe
    id: paintdotnet.exe
    matching_strategy: Equals
- name: pinentry
  identifier:
    kind: Exe
    id: pinentry.exe
    matching_strategy: Equals
  float_identifiers:
  - kind: Exe
    id: pinentry.exe
    matching_strategy: Equals
- name: qBittorrent
  identifier:
    kind: Exe
    id: qbittorrent.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
- name: todoist
  identifier:
    kind: Exe
    id: Todoist.exe
    matching_strategy: Equals
- name: ueli
  identifier:
    kind: Exe
    id: ueli.exe
    matching_strategy: Equals
  options:
  - tray_and_multi_window
  float_identifiers:
  - kind: Exe
    id: ueli.exe
    matching_strategy: Equals
";