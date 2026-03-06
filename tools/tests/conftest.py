"""テスト設定

tools/incoming_inspection/ モジュールをインポート可能にする
"""

import sys
from pathlib import Path

# プロジェクトルートをパスに追加
project_root = Path(__file__).parent.parent.parent
sys.path.insert(0, str(project_root))

# tools/incoming_inspection/ を直接インポート可能にする
tools_path = Path(__file__).parent.parent / "incoming_inspection"
sys.path.insert(0, str(tools_path))
