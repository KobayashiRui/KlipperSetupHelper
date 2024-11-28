#!/bin/bash
set -e

TAG="v0.0.0"

# 設定
REPO="KobayashiRui/KlipperSetupHelper"
INSTALL_DIR="$HOME/KlipperSetupHelper"
ARCH=$(uname -m)

# ファイルURLの定義
FRONTEND_ZIP_URL="https://github.com/$REPO/releases/download/$TAG/frontend-assets.zip"
if [ "$ARCH" = "aarch64" ]; then
  BACKEND_FILE_URL="https://github.com/$REPO/releases/download/$TAG/ksh-backend-aarch64-unknown-linux-gnu"
elif [ "$ARCH" = "armv7l" ]; then
  BACKEND_FILE_URL="https://github.com/$REPO/releases/download/$TAG/ksh-backend-armv7-unknown-linux-gnueabihf"
else
  echo "Unsupported architecture: $ARCH"
  exit 1
fi

# ディレクトリ作成
echo "Creating installation directory..."
mkdir -p "$INSTALL_DIR"

# フロントエンドのダウンロードと展開
echo "Downloading and extracting frontend assets..."
curl -L $FRONTEND_ZIP_URL -o frontend-assets.zip
unzip -o frontend-assets.zip -d "$INSTALL_DIR/frontend_assets"

# バックエンドのダウンロード
echo "Downloading backend binary..."
curl -L $BACKEND_FILE_URL -o "$INSTALL_DIR/backend"
chmod +x "$INSTALL_DIR/backend"

# サービスのセットアップ
echo "Setting up systemd service in /etc/systemd/system/..."
sudo curl -L "https://github.com/$REPO/releases/download/$TAG/KlipperSetupHelper.service" -o /etc/systemd/system/KlipperSetupHelper.service
sudo systemctl enable KlipperSetupHelper.service
sudo systemctl start KlipperSetupHelper.service


echo "Installation completed! All files are in $INSTALL_DIR"