---
allowed-tools: Read, Bash
argument-hint: [install]
description: Setup reqvire environment
model: claude-sonnet-4-5-20250929
---
  
### Installing reqvire

Detect the operating system and install reqvire accordingly:

1. First check the platform (look at the `<env>` context for "Platform:" info)
2. Run the appropriate installation commands for that platform

#### Linux x86_64
```bash
mkdir -p ~/.local/bin
curl -fsSL -o /tmp/reqvire.tar.gz https://github.com/Reqvire/reqvire/releases/download/v0.9.0/reqvire-linux-x86_64.tar.gz
tar -xzf /tmp/reqvire.tar.gz -C ~/.local/bin
mv ~/.local/bin/reqvire-linux-x86_64 ~/.local/bin/reqvire
chmod +x ~/.local/bin/reqvire

Mac Silicon (ARM64)

mkdir -p ~/.local/bin
curl -fsSL -o /tmp/reqvire.tar.gz https://github.com/Reqvire/reqvire/releases/download/v0.9.0/reqvire-darwin-aarch64.tar.gz
tar -xzf /tmp/reqvire.tar.gz -C ~/.local/bin
mv ~/.local/bin/reqvire-darwin-aarch64 ~/.local/bin/reqvire
chmod +x ~/.local/bin/reqvire

Mac Intel (x86_64)

mkdir -p ~/.local/bin
curl -fsSL -o /tmp/reqvire.tar.gz https://github.com/Reqvire/reqvire/releases/download/v0.9.0/reqvire-darwin-x86_64.tar.gz
tar -xzf /tmp/reqvire.tar.gz -C ~/.local/bin
mv ~/.local/bin/reqvire-darwin-x86_64 ~/.local/bin/reqvire
chmod +x ~/.local/bin/reqvire

Windows (PowerShell)

New-Item -ItemType Directory -Force -Path "$env:USERPROFILE\.local\bin"
Invoke-WebRequest -Uri "https://github.com/Reqvire/reqvire/releases/download/v0.9.0/reqvire-windows-x86_64.zip" -OutFile "$env:TEMP\reqvire.zip"
Expand-Archive -Path "$env:TEMP\reqvire.zip" -DestinationPath "$env:USERPROFILE\.local\bin" -Force
Rename-Item "$env:USERPROFILE\.local\bin\reqvire-windows-x86_64.exe" "reqvire.exe"

