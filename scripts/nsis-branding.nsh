; Tauri sets MUI_ICON from installerIcon but leaves MUI_UNICON at NSIS's default.
; This include runs before MUI pages are expanded. Use the same approved ICO.
!define MUI_UNICON "${__FILEDIR__}\..\src-tauri\icons\icon.ico"
