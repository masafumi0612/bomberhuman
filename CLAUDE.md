# CLAUDE.md — BomberHuman

Rustで実装しWebAssemblyにコンパイルするボンバーマン風マルチプレイヤーブラウザゲーム。

## 技術スタック

- **Rust** — ゲームロジック (`src/`)
- **WebAssembly** — `wasm-pack` + `wasm-bindgen` 経由
- **JavaScript** — ゲームループ・入力処理・Canvasレンダリング (`www/`)
- **Webpack** — JSバンドラー

## ビルド・実行方法

```bash
# Rust → WASM コンパイル
wasm-pack build

# 開発サーバー起動 (localhost:8080)
cd www && npm install && npm start

# プロダクションビルド
cd www && npm run build
```

## プロジェクト構成

```
src/
  lib.rs                  # WASMエントリーポイント・ゲーム定数・メイン更新ロジック
  game_state.rs           # ゲーム状態管理
  models/                 # エンティティ: Player, Bomb, Fire, Wall, SBlock, Pow, World
  controllers/
    collisions.rs         # 衝突判定 (壁・炎・爆弾・パワーアップ)
  geometry/               # Point, Size, 衝突トレイト
  javascript/draw.js      # Canvas描画関数
www/
  index.js                # ゲームループ (requestAnimationFrame)・キーボード/ゲームパッド入力
  index.html              # HTMLキャンバスページ
  image/                  # スプライト (player1-4.png, bomb.png, fire.jpg など)
docs/                     # ビルド済みWASM + JS (静的デモ用)
```

## ゲーム定数 (src/lib.rs)

| 定数 | 値 |
|---|---|
| WORLD_SIZE | 750 × 650 px |
| PLAYER_SIZE | 50 × 50 px |
| PLAYER_SPEED | 150 px/秒 |
| GRID | 50 px |
| BOMB_TTL | 3 秒 |
| BOMB_POWER | 1 (グリッドマス) |
| BOMB_NUM | 1 個 |
| FIRE_TTL | 1 秒 |
| SBLOCK_MAKE_PERCENT | 70% |

## 操作方法

| プレイヤー | 移動 | 爆弾設置 |
|---|---|---|
| P1 | 矢印キー | Space |
| P2 | WASD | X |
| P3 | TFGH | B |
| P4 | IJKL | O |

ゲームパッド: アナログスティックで移動、ボタン0で爆弾設置。

## 設計上のポイント

- プレイヤースポーン位置 (四隅): `(75,75)` `(675,75)` `(75,575)` `(675,575)`
- マップは永続壁 (外周 + チェッカーボード配置) とランダム配置の破壊可能ブロック (SBlock, 70%) で構成
- パワーアップはSBlock内に隠れており、破壊すると出現: 爆風強化・爆弾数増加・移動速度アップ
- 炎は4方向に伸び、爆弾を誘爆し、SBlockを破壊する
- 壁との衝突はスライド処理あり (角に引っかかりにくい)
