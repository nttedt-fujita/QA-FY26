# Python API: アップデート・デバイス検索

**出典**: AS-DT1 APIマニュアル FW1.00
**抽出ページ**: 75-78 (4ページ)

---

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

## Page 76

```
76
update
機能
AS-DT1 のファームウェアをアップデートします。
書き込みが可能なファイルは、拡張子が.f.bin のファイルのみです。
書き込みが終了するとコンストラクターで設定されたcallback 関数が呼ばれます。
処理の結果は、callback 関数の戻り値および、クラス変数returncode で返されます。
定義
update(self, file=None):
引数
file：書き込むファームウェアファイル名（文字列）
戻り値
なし
verify
機能
AS-DT1 のファームウェアのベリファイを行います。
ベリファイが可能なファイルは、拡張子が.f.bin のファイルのみです。
ベリファイが終了するとコンストラクターで設定されたcallback 関数が呼ばれます。
処理の結果は、callback 関数の戻り値および、クラス変数returncode で返されます。
定義
verify(self, file=None):
引数
file：本機のファームウェアと比較を行うファームウェアファイル名（文字列）
戻り値
なし
get_progress
機能
ファームウェアの書き込みやベリファイの進捗状況を取得します。
定義
get_progress(self):
引数
なし
戻り値
進捗状態（文字列）
書き込み中、またはベリファイ中のバンク番号とオフセットアドレスを文字列で返します。
set_stop
機能
ファームウェアの書き込み、またはベリファイを強制終了させます。
定義
set_stop(self):
引数
なし
```

## Page 77

```
77
戻り値
なし
test code
このAPI ソースコードには、テストコードが含まれており、このソースコードを下記の手順で動作させることで、ファームウェアの
アップデート、ベリファイ強制終了、ベリファイが行われます。
起動方法
使用するシステムに応じて、py またはpython を使用して起動させてください。
引数
comport：COM port 名を設定してください。
bitrate：UART 接続時はビットレートを設定してください。USB の場合は0 を指定してください。
filename：アップデートパッケージファイル名
動作
このプログラムを起動すると指定されたCOM ポートが開き、指定されたファイルを使って、接続されている機器のファームウェア
アップデート、ベリファイ強制終了、ベリファイが行われます。
as_dt1_api_enumeration.py（AS-DT1 デバイス検索API）
接続されているAS-DT1 を検索しデバイス名を取得するAPI です。
接続されたすべてのAS-DT1 のデバイス名を列挙する関数と、AS-DT1 のシリアルナンバーからデバイス名を検索する関数が用意さ
れています。
class as_dt1_api_enumeration
機能
デバイス検索クラス
__init__(self):
機能
as_dt1_ api_enumeration のコンストラクターです。クラスの初期化を行います。
引数
なし
戻り値
なし
get_uvc_list
機能
接続されているAS-DT1 のuvc デバイスを検索しOpenCV で使用できるUVC デバイス番号、およびUVC デバイスのシリアル番号
を返します。
定義
get_uvc_list(self):
引数
なし
戻り値
[' ', '1:AS-DT1_1000001', '2:AS-DT1_1000002']
UVC デバイス番号、デバイス名(AS-DT1)、AS-DT1 のシリアル番号が含まれた文字列の配列
as-dt1_api_update.py comport bitrate filename
```

## Page 78

```
78
get_uart_list
機能
接続されているAS-DT1、およびAS-DT1 以外のUART ポートのリストを返します。
定義
get_uart_list(self):
引数
なし
戻り値
([' ', 'COM12 AS-DT1_1000001', 'COM23 AS-DT1_1000002'], [' ', 'COM14 USB serial device (COM14)'])
AS-DT1 のシリアルポート名（Linux の場合はデバイスパス）、デバイス名、シリアル番号が含まれた文字列の配列
AS-DT1 以外のシリアルポート名（Linux の場合はデバイスパス）、デバイス名が含まれた文字列の配列
get_port_list
機能
接続されているAS-DT1、およびAS-DT1 以外のUVC/UART ポートのリストを返します。
定義
get_port_list(self):
引数
なし
戻り値
([' ', 'COM10 AS-DT1_1000001', 'COM13 AS-DT1_1000002'], 
[' ', '1:AS-DT1_1000001', '2:AS-DT1_1000002'], 
[' ', 'COM14 USB serial device (COM14)'])
AS-DT1 のシリアルポート名（Linux の場合はデバイスパス）、デバイス名、シリアル番号が含まれた文字列の配列
AS-DT1 のUVC デバイス番号、デバイス名、シリアル番号が含まれた文字列の配列
AS-DT1 以外のシリアルポート名（Linux の場合はデバイスパス）、デバイス名が含まれた文字列の配列
get_port_from_serial_number
機能
シリアル番号指定で、シリアルポート名やUVC のデバイス番号を取得します。
定義
get_port_from_serial_number(self, serial_number=None):
引数
serial_number：AS-DT1 のシリアル番号
戻り値
('COM10', '1')
AS-DT1 のシリアルポート名（デバイスパス）の文字列
AS-DT1 のUVC デバイス番号の文字列
as_dt1_api_hist.py ヒストグラムモードAPI
class as_dt1_api_hist()
機能
ヒストグラムデータ取得クラス
```
