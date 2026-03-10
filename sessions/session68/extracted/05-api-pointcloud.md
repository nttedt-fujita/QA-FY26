# Python API: 点群処理 (as_dt1_api_cdc.py)

**出典**: AS-DT1 APIマニュアル FW1.00
**抽出ページ**: 66-75 (10ページ)

---

## Page 66

```
66
UI 操作に対応した、下記の関数が用意されています。
PythonAPI について
PythonAPI は3 つのファイルで構成されています。
as_dt1_api_cdc.py 点群処理API
cdc/extuart での本機との接続や点群（pcd）データ、rawdata を取得、および処理するAPI です。
class as_dt1_api_pcd()
機能
点群処理クラス
__init__(self):
機能
as_dt1_api_pcd のコンストラクターです。クラスの初期化を行います。
引数
なし
com_change_flstart
機能
測距モード変更シーケンスを実行します。
定義
com_change_flstart(self, firmware_kind=None, cdc_port="", uart_port="", uart_bitrate=0, serial_number=""):
引数
firmware_kind：AS-DT1 の動作モードを文字列で設定します。
cdc_port：AS-DT1 のUSB CDC COM ポートが接続されているデバイス名を文字列で設定します。
uart_port：AS-DT1 に接続されたUART ポートのデバイス名を文字列で設定します。
uart_bitrate：ビットレート（115200、230400、460800、921600）を文字列で設定します。
serial_number：AS-DT1 のシリアルナンバー
__init__(self, master = None, serial_port=None,bitrate=0, callback=None, ver=None, serial_number=None):
                コンストラクター、初期化、画面構築
progress_set(self, val): プログレスバー表示の更新
com_quit(self, return_code=0): Main Menu ボタンをクリックしたときの処理
com_quitfin(self): 終了処理
close_window(self): WindowClose したときの処理
_ret_aplcode(self, ret_data=None): update/verify の終了通知callback 関数
com_upd_start(self): アップデートの開始
com_vry_start(self): ベリファイの開始
com_upd_progress_do(self): アップデート、ベリファイスレッドの起動
com_prg_thr(self, event=None): アップデート、ベリファイスレッド
com_click_double(self): アップデート、ベリファイボタンをダブルクリックしたときの処理
com_stop_judge(self): アップデート、ベリファイの停止ボタン判断
com_load_uud(self): アップデートファイルの読み込み
com_reboot(self): Reboot ボタンの処理
get_file_name(self, stitle="file", types="img"): ファイル選択ダイアログ表示・ファイル名取得
com_open_com(self): AS-DT1 からのバージョン情報などの取得
ファイル名
内容
as_dt1_api_cdc.py
cdc/uart で本機の接続や点群（pcd）データを取得、および処理するクラス
as_dt1_api_uvc.py
HIST モードでヒストグラムデータを取得、および処理するクラス
as_dt1_api_update.py
update モードでファームウェアをアップデートするクラス
```

## Page 67

```
67
戻り値
現在の動作モードを文字列で返します。
接続したAS-DT1 のシリアル番号を返します。
クラス変数、cdc_status、art_status が設定されます。
com_change_flmode
機能
測距レンジ変更シーケンスを実行します。
定義
com_change_flmode(self, serial_fh=None, flmode=None):
引数
serial_fh：シリアルポートのファイルハンドル
flmode：測距レンジを設定します。
戻り値
なし
クラス変数、flmode が設定されます。
com_get_tofinfo
機能
AS-DT1 および測距センサーの情報を取得します。
com_capture、pcd、com_data_capture、make_xy_from_z、get_svm_xyz
定義
com_get_tofinfo(self, serial_fh=None):
引数
serial_fh ：シリアルポートのファイルハンドル
戻り値
(version、registory_ver、set serial_no、tofmode、sensor_serial_number)
version：現在実行中のソフトウェアバージョンが文字列で戻ります。
registory_ver：測距センサーに設定される設定値のバージョンが文字列で戻ります。
set serial_no：AS-DT1 のシリアル番号が文字列で戻ります。
tofmode：測距レンジが文字列で戻ります。
sensor_serial_number：測距モジュールの持つシリアル番号が文字列で戻ります。
クラス変数、firmware_version、setting_data_version、serial_number、flmode、module_serial_number、svm_xyz が設定されま
す。
情報取得中に測距が行われると正しくデータを取得することができません。fsync 0 で自動測距を停止させ、trgin disable で外部トリ
ガーを禁止させることで測距を行わないように設定してください。
com_get_general_info
機能
本機のflshow、sensorinfo、ver、ustatus、cstatus、diag、errinfo の情報を取得します。
問題が発生したときや、故障解析のために使用します。
定義
com_get_general_info(self, serial_fh=None):
引数
serial_fh：シリアルポートのファイルハンドル
ご注意
```

## Page 68

```
68
戻り値
flshow、sensorinfo、ver、ustatus、cstatus、diag、errinfo の出力文字列
情報取得中に測距が行われると正しくデータを取得することができません。fsync 0 で自動測距を停止させ、trgin disable で外部トリ
ガーを禁止させることで測距を行わないように設定してください。
com_open_com
機能
シリアルポートをオープンし、AS-DT1 を制御するためのシリアルポートのファイルハンドルを取得します。
定義
com_open_com(self, serial_port="", bitrate=0, timeout=0.2, serial_number=""):
引数
serial_port：シリアルポートのデバイス名を文字列で設定する
bitrate：シリアルポートのビットレートを文字列で設定する
timeout：シリアルポートのタイムアウト値を秒単位の実数で設定する
serial_number：本機のシリアル番号を文字列で設定する
戻り値
serial_fh：シリアルポートのファイルハンドル
com_set_format
機能
測距結果出力形式を設定します。
定義
com_set_format(self, serial_fh=None, format="ascii"):
引数
serial_fh：シリアルポートのファイルハンドル
format：測距結果出力形式（ascii、binary、binz）を文字列で設定します。
戻り値
なし
com_get_format
機能
現在設定されている測距結果出力形式を取得します。
定義
com_get_format(self, serial_fh=None):
引数
serial_fh：シリアルポートのファイルハンドル
戻り値
format：測距データ形式（ascii、binary、binz）が文字列で返ってきます。
com_close_com
機能
AS-DT1 との通信を終了させます。
定義
com_close_com(self, serial_fh=None):
ご注意
```

## Page 69

```
69
引数
serial_fh：シリアルポートのファイルハンドル
戻り値
なし
com_send
機能
引数で指定されたコマンドを、AS-DT1 に送信します。
送信前に受信バッファーをクリアしてから送信を行います。
定義
com_send(self, serial_fh=None, command=None):
引数
serial_fh：シリアルポートのファイルハンドル
command：コマンド文字列
戻り値
なし
com_send_recv
機能
引数で指定されたコマンドを、AS-DT1 に送信します。
送信前に受信バッファーをクリアしてから送信を行います。
コマンド送信後プロンプトが帰ってくるまでデータを受信します。
定義
com_send_recv(self, serial_fh=None, command=None):
引数
serial_fh：シリアルポートのファイルハンドル
command：コマンド文字列
戻り値
受信文字列
com_recv
機能
プロンプトが来るまでデータを受信します。
定義
com_recv(self, serial_fh=None):
引数
serial_fh：シリアルポートのファイルハンドル
戻り値
受信文字列
com_single_cap
機能
t コマンドを送信します。
定義
com_single_cap(self, serial_fh=None):
```

## Page 70

```
70
引数
serial_fh：シリアルポートのファイルハンドル
戻り値
なし
com_fsync_cap
機能
fsync コマンドを送信し、繰り返し測距を開始します。
定義
com_fsync_cap(self, serial_fh=None, cycle=500):
引数
serial_fh：シリアルポートのファイルハンドル
cycle：測距繰り返し周期の少数値、小数点以下2 桁（単位：ms）
戻り値
なし
com_capture_pcd
機能
測距データをフォーマット、および測距レンジ設定に合わせて取得します。
フォーマット、および測距レンジはインスタンス変数に保存されているものを使用します。
576 点の測距点の1st ピーク、2nd ピークに対してX、Y、Z の点が出力されます。出力の576 点のデータの並びは、左上から行・列
の順で走査したものとなります。
フォーマット設定がz のみ出力される場合（ascii またはbinz）でも、svm データを使用し、x およびy 座標が計算されて出力されま
す。
フォーマット設定が1st ピークのみの場合（ascii またはbinz）は、2nd ピークには0.0 が格納されます。
com_set_mode_format でフォーマットが、com_change_mode で測距レンジが設定できます。
定義
com_capture_pcd(self, serial_fh=None):
引数
serial_fh：シリアルポートのファイルハンドル
戻り値
point_cloud 点群（pcd）データ(numpy.float32 [2][576][3]、X,Y,Z 座標（単位：m）)
com_data_capture_ser_xyz_bin
機能
AS-DT1 よりrawdata を取得します。format binary 設定時に使用します。
出力は、X、Y、Z 座標すべてを含みます。データの並びはAS-DT1 の出力順（rawdata）となっています。
この関数の出力を、make_pcd_from_raw 関数によりデータを画面左上からに並び変えることができます。
定義
com_data_capture_ser_xyz_bin(self,serial_fh=None):
引数
serial_fh：シリアルポートのファイルハンドル
戻り値
rawdata(numpy.float32 [2][576][3]、X,Y,Z 座標（単位：mm）)
```

## Page 71

```
71
com_data_capture_ser_ascii
機能
AS-DT1 よりrawdata を取得します。format ascii 設定時に使用します。
出力は、1st Peak のZ 座標のみで、X、Y は0.0 mm、データの並びはAS-DT1 の出力順となっています。
この出力を、make_xy_from_z 関数により、X、Y 座標を生成することで、通常のrawdata を得ることができます。
定義
com_data_capture_ser_ascii(self,serial_fh=None):
引数
serial_fh：シリアルポートのファイルハンドル
戻り値
rawdata(numpy.float32 [2][576][3]、X,Y,Z 座標（単位：mm）、z のみ)
com_data_capture_ser_z_bin
機能
AS-DT1 よりrawdata を取得します。format binz 設定時に使用します。
出力は、1st Peak のZ 座標のみで、X、Y は0.0 mm、データの並びはAS-DT1 の出力順となっています。
この出力を、make_xy_from_z 関数により、X、Y 座標を生成することで、通常のrawdata を得ることができます。
定義
com_data_capture_ser_z_bin(self,serial_fh=None):
引数
serial_fh：シリアルポートのファイルハンドル
戻り値
rawdata(numpy.float32 [2][576][3]、X,Y,Z 座標（単位：mm）、z のみ、1st Peak のみ)
rotation_xyz
機能
点群データを指定された角度回転させます。入力の点群データに回転行列をX、Y、Z の順で適用することで、演算を行います。
定義
rotation_xyz(self, point_cloud=None, theta_x=0, theta_y=0, theta_z=0):
引数
point_cloud 点群データ(numpy.float32 [2][576][3]、X,Y,Z 座標（単位：m）)
theta_x：x 回転角度（単位：度）
theta_y：y 回転角度（単位：度）
theta_z：z 回転角度（単位：度）
戻り値
点群(pcd) データ(numpy.float32 [2][576][3]、X,Y,Z 座標（単位：m）)
make_pcd_from_raw
機能
以下のrawdata を使用してpoint_cloud 点群データ(numpy.float32 [2][576][3]、X,Y,Z 座標（単位：m）) を作成します。
• com_data_capture_ser_xyz_bin 関数が出力するrawdata
• com_data_capture_ser_ascii 関数やcom_data_capture_ser_z_bin 関数の出力を用いて、make_xy_from_z 関数によりX、Y 座標
を生成することで得られるrawdata
定義
make_pcd_from_raw(self, rawdata=None):
```

## Page 72

```
72
引数
rawdata(numpy.float32 [2][576][3]、X,Y,Z 座標（単位：mm）)
戻り値
点群(pcd) データ(numpy.float32 [2][576][3]、X,Y,Z 座標（単位：m）)
make_xy_from_z
機能
format ascii やbinz で取得したrawdata のX、Y 座標をZ 座標から作成します。
定義
make_xy_from_z(self, pcd_raw_z=None):
引数
rawdata(numpy.float32 [2][576][3]、X,Y,Z 座標（単位：mm）、z のみ)
戻り値
rawdata(numpy.float32 [2][576][3]、X,Y,Z 座標（単位：mm）)
getMp
機能
距離データの配列順（左上から行・列の順で走査したもの）の測距点番号で、SPAD センサー上の位置、および測距点のサイズ情報
を取得します。
定義
getMp(self,mp_num=None):
引数
mp_num：測距点番号
戻り値
位置とサイズ情報 ([x,y,w,h] mpxy address x,y and macro pixel size w,h)
get_mpxydef
機能
測距点のセンサー上の位置、および測距点のサイズ情報をAS-DT1 から取得します。
定義
get_mpxydef(self, serial_fh):
引数
serial_fh：シリアルポートのファイルハンドル
戻り値
位置とサイズ情報 ([576][4]、x,y,w,h)
get_svm_xyz
機能
測距点のsvm 情報をAS-DT1 から取得します。
定義
get_svm_xyz(self, serial_fh):
引数
serial_fh： シリアルポートのファイルハンドル
```

## Page 73

```
73
戻り値
svm 情報(numpy.float32 [576][2]、X/Z、Y/Z)
firmware_kind
型
str（文字列）
意味
現在の測距モードが文字列で保存されています。
AS-DT1 と接続されていないときは""（空文字列）となります。
接続されていて、動作モードが取得されると下記のような文字列が入ります。
"CDC"、"Uart"、"Inten"、"Update"、"Hist"
firmware_version
型
str（文字列）
意味
現在のAS-DT1 のファームウェアバージョンが文字列で保存されています。
AS-DT1 と接続されていないときは""（空文字列）となります。
接続されていて、取得されると下記のような文字列が入ります。
"CDC Version 1.00"
setting_data_version
型
str（文字列）
意味
現在のAS-DT1 のレジスター設定情報のバージョンが文字列で保存されています。
AS-DT1 と接続されていないときは""（空文字列）となります。
接続されていて、取得されると下記のような文字列が入ります。
"Version 1.00 REG 1.0.0"
serial_number
型
str（文字列）
意味
現在のAS-DT1 のシリアル番号が文字列で保存されています。
AS-DT1 と接続されていないときは""（空文字列）となります。
接続されていて、取得されると下記のような文字列が入ります。
"1000001"
module_serial_number
型
str（文字列）
意味
現在のAS-DT1 の測距モジュールシリアル番号が文字列で保存されています。
AS-DT1 と接続されていないときは""（空文字列）となります。
接続されていて、取得されると下記のような文字列が入ります。
"FFFFF4100001X0000"
```

## Page 74

```
74
cdc_status
型
str（文字列）
意味
現在のAS-DT1 のUSB CDC 接続状態が保存されています。
AS-DT1 と接続されていないときは""（空文字列）となります。
接続されていて、取得されると下記のような文字列が入ります。
"USB SS"、"USB HS"、"USB FS"、"USB Disconnect"、"USB Disable"
uart_status
型
str（文字列）
意味
現在のAS-DT1 のレジストリバージョンが文字列で保存されています。
AS-DT1 と接続されていないときは""（空文字列）となります。
接続されていて、取得されると下記のような文字列が入ります。
"UART 115200bps"、"UART 230400"、"UART 460800"、"UART 921600"、"UART Disconnect"、"UART Disable"
return_status
型
str（文字列）
意味
関数呼び出しのエラー状態が保存されます。
一度文字列が設定されるとNone には戻らないため、エラー処理を行った後にNone を代入してください。
flmode
型
str（文字列）
意味
現在のAS-DT1 の測距レンジが文字列で保存されています。
AS-DT1 と接続されていないときは""（空文字列）となります。
接続されていて、取得されると下記のような文字列が入ります。
"30mstd"、"30m15f"、"30m30f"、"20m"、"40m"
cdc_uart_format
型
str（文字列）
意味
現在のAS-DT1 の測距レンジが文字列で保存されています。
AS-DT1 と接続されていないときは""（空文字列）となります。
接続されていて、取得されると下記のような文字列が入ります。
"ascii"、"binary"、"binz"
timestamp
型
str（文字列）
```

## Page 75

```
75
意味
最後に取得した測距データのタイムスタンプが保存されます。
測距を行っていない場合は""（空文字列）となります。
取得されると下記のような文字列が入ります。
"102456"
svm_xyz
型
numpy.float32 [576][2]
意味
AS-DT1 より取得したsvm 情報（測距結果のz から、x、y 座標を計算するテーブル）が保存されます。
com_get_tofinfo 関数を呼び出すまでは、0.0 が入っています。
test code
このAPI ソースコードには、テストコードが含まれており、このソースコードを下記の手順で動作させることで、診断情報の表示と
1 回の測距が実施されます。
起動方法
使用するシステムに応じて、py またはpython を使用して起動させてください。
引数
comport：COM port 名を設定してください。
動作
このプログラムを起動すると指定されたCOM ポートが開き、本機のセンサー情報、動作情報を取得・表示してからascii モードでt
コマンドを発行し、1 回測距を行いその結果の一部を表示します。
さらにその後、fsync コマンドにて測距、取得した点群の一部とそれをrotation で回転した結果の一部を表示します。
as_dt1_api_update.py アップデートAPI
ファームウェアの書き込み、およびベリファイを行うAPI です。
class as_dt1_api_update():
機能
ファームウェアップデートクラス
__init
機能
as_dt1_api_update のコンストラクターです。クラスの初期化を行います。
定義
__init__(self, serial_port="", bitrate=0, callback=None, serial_number=""):
引数
serial_port：シリアルポートのデバイス名を文字列で設定します。
bitrate：シリアルポートのビットレートを文字列で設定します。
callback：callback 関数を設定します。
serial_number：本機のシリアル番号を文字列で設定します。
戻り値
なし
as-dt1_api_pcd.py comport
```
