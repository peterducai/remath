   1 dism.exe /online /enable-feature /featurename:Microsoft-Windows-Subsystem-Linux /all /norestart
   2 dism.exe /online /enable-feature /featurename:VirtualMachinePlatform /all /norestart
   3 wsl --set-default-version 2