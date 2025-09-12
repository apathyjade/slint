; 安装程序配置
!define APP_NAME "My Slint App"
!define APP_VERSION "0.1.0"
!define APP_PUBLISHER "Your Name"
!define APP_URL "https://your-app-url.com"
!define EXE_NAME "app.exe"
!define INSTALL_DIR "$PROGRAMFILES\${APP_NAME}"
!include "MUI2.nsh"

; 配置安装程序
Name "${APP_NAME} ${APP_VERSION}"
OutFile "${APP_NAME}-setup-${APP_VERSION}.exe"
InstallDir "${INSTALL_DIR}"
Icon "resources/installer_icon.ico"
UninstallIcon "resources/uninstall_icon.ico"
; Program Files
RequestExecutionLevel admin
; 现代 UI 配置
!define MUI_ABORTWARNING
!define MUI_LICENSEPAGE_TEXT "请阅读并接受许可协议"
!define MUI_LICENSEPAGE_RADIOBUTTONS
!insertmacro MUI_PAGE_LICENSE "resources/LICENSE.txt"
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_PAGE_FINISH

!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES
!insertmacro MUI_UNPAGE_FINISH

!insertmacro MUI_LANGUAGE "SimpChinese" ; 中文界面

; 安装过程
Section "Main Program"
    ; 创建安装目录
    SetOutPath "${INSTALL_DIR}"
    ; copy exe
    File "my_slint_app.exe"
    ; File "../../target/release/*.dll"
    ; menu
    CreateDirectory "$SMPROGRAMS\${APP_NAME}"
    CreateShortcut "$SMPROGRAMS\${APP_NAME}\${APP_NAME}.lnk" "${INSTALL_DIR}\${EXE_NAME}"
    CreateShortcut "$SMPROGRAMS\${APP_NAME}\卸载.lnk" "$INSTDIR\uninstall.exe"
    
    ; 创建卸载程序
    WriteUninstaller "${INSTALL_DIR}\uninstall.exe"
SectionEnd

; 卸载过程
Section "Uninstall"
    ; 删除快捷方式
    Delete "$SMPROGRAMS\${APP_NAME}\*.*"
    RMDir "$SMPROGRAMS\${APP_NAME}"
    
    ; 删除程序文件
    Delete "${INSTALL_DIR}\${EXE_NAME}"
    Delete "${INSTALL_DIR}\uninstall.exe"
    ; Delete "${INSTALL_DIR}\*.dll"
    ; delete dir
    RMDir "${INSTALL_DIR}"
SectionEnd