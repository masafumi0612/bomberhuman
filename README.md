# BomberHuman

Rustで実装したボンバーマン風マルチプレイヤーブラウザゲーム。RustのゲームロジックをWebAssemblyにコンパイルし、CanvasAPIで描画します。最大4人同時プレイ対応。

**GitHub Pages でプレイできます:** https://masafumi0612.github.io/bomberhuman/

## 操作方法

| プレイヤー | 移動 | 爆弾設置 |
|---|---|---|
| P1 | 矢印キー | Space |
| P2 | WASD | X |
| P3 | TFGH | B |
| P4 | IJKL | O |

ゲームパッド接続時はアナログスティックで移動、ボタン0で爆弾設置。

## デプロイ方法

### 1. ローカル開発サーバー (localhost)

開発時はWebpackの開発サーバーを使用します。

**前提条件:**
- [Rust](https://www.rust-lang.org/tools/install)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- [Node.js / npm](https://nodejs.org/)

```bash
# 1. RustをWebAssemblyにコンパイル
wasm-pack build

# 2. npm依存パッケージのインストール (初回のみ)
cd www
npm install

# 3. 開発サーバー起動
npm start
```

ブラウザで `http://localhost:8080` を開くとゲームが起動します。
ソースコードを変更すると自動でリロードされます。

---

### 2. WASMスタティックビルド (本番用)

Webpackで静的ファイルをビルドし、任意のWebサーバーで配信できます。

```bash
# 1. RustをWebAssemblyにコンパイル
wasm-pack build

# 2. 静的ファイルをビルド (www/dist/ に出力)
cd www
npm install
npm run build
```

`www/dist/` フォルダに以下が生成されます:

```
www/dist/
  index.html
  bootstrap.js      # バンドル済みJS
  *.module.wasm     # WASMバイナリ
  image/            # ゲームスプライト
```

この `dist/` フォルダを任意のWebサーバーのドキュメントルートに配置するだけで動作します。

> **注意:** WASMファイルは `application/wasm` MIMEタイプで配信される必要があります。主要なWebサーバー (nginx, Apache, GitHub Pages など) はデフォルトで対応しています。

---

### 3. GitHub Pages へのデプロイ

リポジトリの `docs/` フォルダを GitHub Pages のソースとして使用します。

#### ビルドと `docs/` への配置

```bash
# 1. RustをWebAssemblyにコンパイル
wasm-pack build

# 2. 静的ファイルをビルド
cd www
npm install
npm run build

# 3. ビルド成果物を docs/ にコピー
cd ..
cp -r www/dist/* docs/
```

#### GitHub Pages の設定

1. GitHub リポジトリの **Settings** → **Pages** を開く
2. **Source** を `Deploy from a branch` に設定
3. **Branch** を `master`（または `main`）、フォルダを `/docs` に設定
4. **Save** をクリック

#### 変更をプッシュ

```bash
git add docs/
git commit -m "Update GitHub Pages build"
git push
```

数分後、`https://<ユーザー名>.github.io/<リポジトリ名>/` でゲームが公開されます。

## 技術スタック

| 技術 | 用途 |
|---|---|
| Rust | ゲームロジック |
| wasm-pack / wasm-bindgen | Rust → WebAssembly コンパイル |
| JavaScript | ゲームループ・入力処理 |
| Canvas 2D API | 描画 |
| Webpack | JSバンドル |
