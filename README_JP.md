# wclock 🌍

美しいリアルタイム世界時計のCLIツールです。主要都市の時刻をエレガントなカードスタイルまたはシンプルなリスト形式で表示します。

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)

## 機能

- 🎨 **2つの表示モード**
  - **カードモード**: 国旗付きのエレガントな4列カードレイアウト
  - **シンプルモード**: クイックリファレンス用のコンパクトなリストビュー
- 🔄 **リアルタイム更新**: 毎秒ライブで時計を更新
- 🌐 **カスタマイズ可能**: 設定ファイルで独自の都市リストを設定
- 🚀 **高速＆軽量**: パフォーマンスのためにRustで構築
- 🎯 **スマートなデフォルト**: 20の主要都市をデフォルトで含む
- 🛡️ **堅牢性**: 無効なタイムゾーンを適切に処理

## インストール

### ソースから

1. [rustup.rs](https://rustup.rs/)からRustツールチェーンをインストール

2. クローンしてインストール:
```bash
git clone https://github.com/yourusername/world-clock-cli.git
cd world-clock-cli
cargo install --path .
```

### Cargoを使用

```bash
cargo install world-clock-cli
```

## 使用方法

### カードモード（デフォルト）

美しいカードレイアウトで時刻を表示:

```bash
wclock
```

### シンプルモード

コンパクトなリストで時刻を表示:

```bash
wclock -s
```

### 終了

どちらのモードでも `Ctrl+C` を押すと終了します。

## 設定

`wclock`は以下の場所（順番）で設定ファイルを探します:

1. `./config.toml` (カレントディレクトリ)
2. `~/.wclock/config.toml` (ユーザーホームディレクトリ)

### 設定例

`~/.wclock/config.toml`を作成:

```toml
# wclock 設定ファイル
# 表示するタイムゾーンのリスト (IANA タイムゾーン形式)

timezones = [
    # アジア
    "Asia/Tokyo",
    "Asia/Seoul",
    "Asia/Shanghai",
    "Asia/Hong_Kong",
    "Asia/Singapore",
    "Asia/Dubai",
    
    # ヨーロッパ
    "Europe/London",
    "Europe/Paris",
    "Europe/Berlin",
    "Europe/Moscow",
    
    # アメリカ大陸
    "America/New_York",
    "America/Los_Angeles",
    "America/Chicago",
    "America/Toronto",
    "America/Sao_Paulo",
    "America/Mexico_City",
    
    # オセアニア
    "Australia/Sydney",
    "Pacific/Auckland",
    
    # アフリカ
    "Africa/Cairo",
    "Africa/Johannesburg",
]
```

### サポートされているタイムゾーン

任意の有効な[IANAタイムゾーン識別子](https://en.wikipedia.org/wiki/List_of_tz_database_time_zones)が使用できます。例:
- `America/New_York`
- `Europe/London`
- `Asia/Tokyo`
- `Australia/Sydney`

### 国旗

このツールには、IANAタイムゾーンデータベースから**550以上の都市**に対する包括的な国旗サポートが含まれています。国旗マッピングは保守性を高めるため地理的地域ごとに整理されています：

- **アフリカ**: 54都市
- **アメリカ大陸**: 168都市
- **アジア**: 99都市
- **オーストラリア**: 23都市
- **ヨーロッパ**: 64都市
- **太平洋**: 44都市
- **その他の地域**: 100以上の都市（大西洋、インド洋、南極など）

特定の国旗マッピングがない都市は🌐地球アイコンを表示します。

## デフォルトの都市

設定ファイルが見つからない場合、wclockは以下の都市を表示します:
- Tokyo, Seoul, Singapore, Dubai (アジア)
- Moscow, London, Paris (ヨーロッパ)
- New York, Los Angeles, Chicago, Toronto (アメリカ大陸)
- Sydney, Auckland (オセアニア)

## ソースからのビルド

```bash
# デバッグビルド
cargo build

# リリースビルド（最適化）
cargo build --release

# テストの実行
cargo test

# ローカルにインストール
cargo install --path .
```

## プロジェクト構成

```
world-clock-cli/
├── src/
│   ├── main.rs         # エントリーポイントとCLI引数解析
│   ├── config.rs       # 設定ファイルの読み込み
│   ├── display.rs      # 表示ロジック（カード＆シンプルモード）
│   ├── timezone.rs     # タイムゾーンデータ構造
│   └── flags/          # 国旗マッピング（地域別に整理）
│       ├── mod.rs      # フラグモジュール統合
│       ├── africa.rs   # アフリカの都市
│       ├── america.rs  # アメリカ大陸の都市
│       ├── asia.rs     # アジアの都市
│       ├── australia.rs # オーストラリアの都市
│       ├── europe.rs   # ヨーロッパの都市
│       ├── pacific.rs  # 太平洋の都市
│       └── other.rs    # その他の地域
├── config.toml         # 設定例
├── Cargo.toml          # Rust依存関係
├── LICENSE             # MITライセンス
└── README.md           # このファイル
```

## コントリビューション

コントリビューションを歓迎します！お気軽にプルリクエストを送信してください。

### 新しい都市の国旗を追加

国旗マッピングは`src/flags/`ディレクトリ内で地理的地域ごとに整理されています。国旗を追加または更新するには：

1. 適切な地域ファイルを見つけます（例：アジアの都市の場合は`src/flags/asia.rs`）
2. `get_flag()`関数内で都市エントリを追加または変更します：

```rust
pub fn get_flag(city_name: &str) -> Option<&'static str> {
    match city_name {
        "あなたの都市" => Some("🇾🇨"),  // あなたの国の国旗
        // ... 既存のエントリー ...
        _ => None,
    }
}
```

3. `src/flags/mod.rs`のメイン`get_country_flag()`関数が自動的にすべての地域を検索します。

## ライセンス

このプロジェクトはMITライセンスの下でライセンスされています - 詳細は[LICENSE](LICENSE)ファイルを参照してください。

## 謝辞

- タイムゾーン処理には[chrono](https://github.com/chronotope/chrono)を使用
- ターミナルUIには[crossterm](https://github.com/crossterm-rs/crossterm)を使用
- 国旗はUnicode絵文字セットから

---

**❤️とRustで作られました**
