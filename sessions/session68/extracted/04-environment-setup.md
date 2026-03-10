# 環境設定（Windows/Linux/Jetson/Raspberry Pi）

**出典**: AS-DT1 APIマニュアル FW1.00
**抽出ページ**: 61-63 (3ページ)

---

## Page 61

```
61
Python サンプルプログラムとAPI
AS-DT1 を使用するためのPython サンプルプログラムとPython API について説明します。
サンプルプログラムでは、AS-DT1 の基本的な動作を行うことができるプログラムの作成方法を知ることができます。
また、Python API を使用することで、AS-DT1 のコマンドを使用することなくAS-DT1 の設定や測距を行うことができるようになり
ます。
サンプルプログラムのディレクトリー構成
サンプルプログラムのディレクトリー構成は下記のとおりです。
フォルダー名／ファイル名
説明
README.txt
最初にお読みください。
application/
Python サンプルプログラムとAPI の関連ファイルが格納されています。
AS-DT1_sample_application.exe
Windows 実行ファイルに変換したアプリケーションです。
LICENSE
ライセンス条項を記載したファイルです。
README.txt
ライセンスに関する注意事項を記載したファイルです。
3rd-party-software/
利用している第三者ソフトウェアのライセンス条項が記載されたファイルやそのライセンスに従って開示義
務のあるソースコードが格納されています。
how_to_build/
Windows 実行ファイルに変換する方法を記載した文書が格納されています。
src/
ソースコードが格納されています。
documents
API マニュアル（本書）が格納されています。
AS-DT1_API_MANUAL_*_EN.pdf
API マニュアル（本書）の英語版です。
AS-DT1_API_MANUAL_*_JP.pdf
API マニュアル（本書）の日本語版です。
firmware
デバイスファームウェアの関連ファイルが格納されています。
AS-DT1_uud_V1.00.f.bin.zip
デバイスファームウェアのバイナリーファイルです。
README.txt
ライセンスに関する注意事項を記載したファイルです。
3rd-party-software-license/
利用している第三者ソフトウェアのライセンス条項が記載されたファイルが格納されています。
EULA/
デバイスファームウェアの使用許諾書が格納されています。
SDK_for_AS-DT1_Ver1.00/
README.txt
application/
AS-DT1_sample_application.exe
LICENSE
README.txt
3rd-party-software/
how_to_build/
src/
documents
AS-DT1_API_MANUAL_Ver.1.00_EN.pdf
AS-DT1_API_MANUAL_Ver.1.00_JP.pdf
Ĭrmware
AS-DT1_uud_V1.00.f.bin.zip
README.txt
3rd-party-software-license/
EULA/
```

## Page 62

```
62
API およびサンプルプログラムのソースコードのディレクトリー構成は下記のとおりです。
環境設定
Python サンプルプログラムおよびPython API を使用するために、事前にインストールが必要なソフトウェアやライブラリーについ
て説明します。
インストールの方法は、各ソフトウェアやOS の手順に従ってください。
Windows 11での環境設定
• OS Windows 11 64 bit
• Python 3.13.9（python.org よりダウンロードしてインストールしてください。）
• 以下の追加ライブラリー
numpy
pyserial
pygame
pygetwindow
pywin32
opencv-python
アプリケーションプログラムフォルダーにpip で使用できるrequirements_win32.txt があります。下記のコマンドでインストールで
きます。
pip install -r requirements_win32.txt
PC用Linuxでの環境設定
• OS Ubuntu 24.04
• Python 3.12.3-1 ubuntu0.8
• 以下の追加ライブラリー
python3-numpy
python3-serial
python3-opencv
python3-pygame
python3-xlib
python3-tk
v4l-utils
python はOS のインストール時にインストールされたものを使用します。
追加ライブラリーは、下記のコマンドでインストールしてください。
sudo apt install python3-numpy python3-serial python3-pil python3-opencv python3-pygame python3-xlib python3-tk v4l-utils 
サンプルプログラムを実行するには、以下コマンドによる実行するユーザーへの権限付与が必要です。
sudo gpasswd -a <username> video
sudo gpasswd -a <username> dialout
src
as_dt1_cdc.py
as_dt1_inten.py
as_dt1_main.py
as_dt1_update.py
requirements_win32.txt
API
        as_dt1_api_enumeration.py
        as_dt1_api_hist.py
        as_dt1_api_pcd.py
        as_dt1_api_update.py
        mdf_dll.dll
```

## Page 63

```
63
Jetsonでの環境設定
• Jetson Orin AGX JetPack 6.2.1
• Python 3.10.12
• 以下の追加ライブラリー
python3-serial
python3-pygame
python3-opencv-python
v4l-utils
python はOS のインストール時にインストールされたものを使用します。
追加ライブラリーは、下記のコマンドでインストールしてください。
sudo apt install python3-serial python3-pygame python3-opencv v4l-utils
サンプルプログラムを実行するには、以下コマンドによる実行するユーザーへの権限付与が必要です。
sudo gpasswd -a <username> video
sudo gpasswd -a <username> dialout
Raspberry PIでの環境設定
• OS Raspberry Pi OS 1 Oct 2025（64 bit または32 bit）
• Python 3.13.5
• 以下の追加ライブラリー
python3-opencv
python はOS のインストール時にインストールされたものを使用します。
追加ライブラリーは、下記のコマンドでインストールを行ってください。
sudo apt install python3-opencv
サンプルプログラムを実行するには、以下コマンドによる実行するユーザーへの権限付与が必要です。
sudo gpasswd -a <username> video
sudo gpasswd -a <username> dialout
サンプルプログラムについて
サンプルプログラムの構成と、動作の概要について説明します。
サンプルプログラムは、以下のファイルで構成されています。
as_dt1_main.pyファイル
class MainMenu(tk.Frame):
MainMenu(tk.Frame) メインメニュー画面の描画と、ボタンをクリックしたときの処理を行っています。tkinter を継承しており、TK
によるGUI 画面を作成します。
MainMenu クラスのTK から継承したメンバー関数mainloop を呼び出すことで、GUI が開始されます。
ファイル名
内容
as_dt1_main.py
メインメニュー画面
as_dt1_cdc.py
CDC/EXTUART 測距モード
as_dt1_inten.py
INTEN モード
as_dt1_update.py
ファームウェアップデートモード
```
