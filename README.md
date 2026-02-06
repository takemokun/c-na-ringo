# c-na-ringo 🍎🎵

Claude Code を使った英語お勉強レポジトリ。
間隔反復システム（SRS）機能搭載済。

> SRS: Spaced Repetition Software

## 🚀 セットアップ

⚠️[Rust](https://rustup.rs/) が必要です⚠️

```bash
git clone https://github.com/takemokun/c-na-ringo.git
cd c-na-ringo
claude /ringo-setup
```

> YOLOモード (`--dangerously-skip-permissions`) 推奨だけど自己責任で！💀

## 📚 スキル一覧

### 翻訳・添削

```
/ringo-learning I eated a lot of sushi yesterday.
```

日本語→英語の翻訳、または英語の添削。ミスは自動でSRSに追加されるよ。

```
/ringo-learning I have went to gym this morning.  今朝ジムに行きました
```

日本語と英語を入力したら、その日本語にあった英語になっているか教えてくれるよ。

### 質問

```
/ringo-question "very delicious" ってネイティブ使う？
```

文法・語彙・使い分けなどの質問に回答してくれるよ。

### 英文解説

```
/ringo-explain I dare you to explain this.
```

入力した英文の慣用句・フレーズ・文法を解説し、理解度クイズを出題。不正解は自動でSRSに追加されるよ。

### 翻訳クイズ

```
/ringo-quiz hard
```

日本語⇄英語のランダム翻訳クイズ。難易度は easy / normal / hard から選べるよ。
間違えた表現は自動でSRSに追加されるよ。

### SRS管理

```
/ringo-srs-add procrastinate 先延ばしにする
/ringo-srs-quiz 5
/ringo-srs-review
```

- `srs-add`: 単語を手動で追加
- `srs-quiz`: フラッシュカードで復習（SM-2アルゴリズム採用、目玉機能）
- `srs-review`: 進捗・統計を確認。現実と向き合える

## 🔄 学習の流れ

1. **練習** → `/ringo-learning` で英作文してみる
2. **フィードバック** → ミスが添削され、自動でSRSに保存される
3. **復習** → `/ringo-srs-quiz` で復習。（スキル使わなくても勝手にクイズは出してくるよ）
4. **確認** → `/ringo-srs-review` で進捗チェック

## 💾 データ

学習記録は `data/` に保存されます（gitignore済み）。
SRSのデータ管理は Rust製の `ringo-srs` CLI が担当しています（SM-2アルゴリズム）。
