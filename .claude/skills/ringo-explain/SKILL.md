---
name: ringo-explain
description: Explain idioms, phrases, words, and grammar in an English sentence. Includes comprehension quiz with automatic SRS integration.
---

# English Sentence Explanation

Break down an English sentence to explain its idioms, phrases, vocabulary, and grammar points. Then test understanding with a quiz. Incorrect answers are automatically added to SRS.

## Usage

```
/ringo-explain <English sentence>
```

## Examples

```
/ringo-explain I've been looking forward to meeting you.
/ringo-explain Had I known about it, I would have told you.
/ringo-explain She couldn't help but laugh at his joke.
/ringo-explain The more I think about it, the less I understand.
```

## Output Flow

### Phase 1: Sentence Breakdown

Analyze the sentence and explain each notable element.

```
## 🔍 英文解説

**原文:** "{input sentence}"

---

### 📖 全体の意味
{Japanese translation and overall meaning}

---

### 🧩 要素解説

**1. {phrase/word/grammar 1}**
- 意味: {Japanese meaning}
- 解説: {Detailed explanation}
- 例文: {Another example sentence}

**2. {phrase/word/grammar 2}**
- 意味: {Japanese meaning}
- 解説: {Detailed explanation}
- 例文: {Another example sentence}

**3. {phrase/word/grammar 3}**
- 意味: {Japanese meaning}
- 解説: {Detailed explanation}
- 例文: {Another example sentence}

---

### 💡 ポイント
- {Key takeaway 1}
- {Key takeaway 2}
```

### Phase 2: Comprehension Quiz

Test understanding of the explained elements.

```
---
## ✏️ 理解度チェック

以下の質問に答えてください:

**Q1.** {Question about element 1}

**Q2.** {Question about element 2}

**Q3.** {Question about element 3}

回答をお願いします（例: Q1: a, Q2: have been, Q3: ...）
```

### Phase 3: Evaluation & SRS Addition

After user answers, evaluate and auto-add incorrect items to SRS.

```
## 📊 結果

| 問題 | 結果 | 正解 |
|------|------|------|
| Q1 | ✓ | - |
| Q2 | ✗ | {correct answer} |
| Q3 | ✗ | {correct answer} |

スコア: 1/3 (33%)

{Brief explanation of incorrect answers}

---
【SRS自動追加】{N}件
不正解だった項目をSRSに追加しました:

1. "{item1}" ({meaning1}) - {type}
2. "{item2}" ({meaning2}) - {type}

次回復習: 明日
```

## What to Explain

### Priority Elements (always explain if present)

1. **Idioms / 慣用句**
   - "look forward to", "couldn't help but", "break the ice"

2. **Phrasal Verbs / 句動詞**
   - "put off", "figure out", "come up with"

3. **Grammar Patterns / 文法パターン**
   - 仮定法, 分詞構文, 関係詞, 比較構文, 倒置

4. **Collocations / コロケーション**
   - "make a decision" (not "do a decision")
   - "heavy rain" (not "strong rain")

5. **Tricky Words / 注意が必要な語**
   - False friends, commonly confused words
   - Words with unexpected meanings in context

### Skip (don't explain unless complex)
- Basic vocabulary (I, you, the, is, etc.)
- Simple grammar that learners already know
- Obvious translations

## Question Types for Quiz

### 1. Meaning Check (意味確認)
```
Q. "look forward to" の意味は？
   a) 前を見る  b) 楽しみにする  c) 期待外れ
```

### 2. Fill-in-the-Blank (穴埋め)
```
Q. I'm looking forward ___ seeing you. (適切な前置詞)
```

### 3. Usage Check (用法確認)
```
Q. "look forward to" の後に来るのは？
   a) 動詞の原形  b) 動名詞 (-ing)  c) to不定詞
```

### 4. Translation (翻訳)
```
Q. 「彼女に会えるのを楽しみにしています」を英語で:
```

### 5. Error Correction (誤り訂正)
```
Q. 間違いを直してください: "I look forward to meet you."
```

## SRS Auto-Addition Logic

### When to Add
- Items from **incorrect** quiz answers
- Extract the specific phrase/grammar that was missed

### Item Format

```json
{
  "id": "item_YYYYMMDD_HHMMSS_XXX",
  "type": "phrase|idiom|grammar|word",
  "front": "English expression",
  "back": "Japanese meaning/explanation",
  "context": "Original sentence from explanation",
  "context_ja": "Japanese translation",
  "source": "ringo-explain",
  "created_at": "ISO timestamp",
  "next_review": "tomorrow",
  "ease_factor": 2.5,
  "interval_days": 0,
  "status": "new"
}
```

### Duplicate Check
- Check `front` field (case-insensitive)
- Skip if already exists in SRS

## Example Session

**User:** `/ringo-explain I've been looking forward to meeting you.`

**Claude:**
```
## 🔍 英文解説

**原文:** "I've been looking forward to meeting you."

---

### 📖 全体の意味
「あなたに会えるのをずっと楽しみにしていました。」

---

### 🧩 要素解説

**1. I've been ~ing（現在完了進行形）**
- 意味: ずっと〜し続けている
- 解説: 過去から現在まで継続している動作を表す。"I've been waiting for an hour." のように使う。
- 例文: She's been studying English for 3 years.（彼女は3年間英語を勉強し続けている）

**2. look forward to（句動詞）**
- 意味: 〜を楽しみにする
- 解説: "to" は前置詞なので、後ろには名詞か動名詞(-ing)が来る。to不定詞ではないので注意！
- 例文: I look forward to hearing from you.（ご連絡をお待ちしています）

**3. meeting（動名詞）**
- 意味: 会うこと
- 解説: "look forward to" の後なので動名詞。"to meet" ではなく "to meeting" が正しい。
- 例文: I'm used to getting up early.（早起きには慣れている）

---

### 💡 ポイント
- "look forward to" の後は必ず **動名詞(-ing)** または **名詞**
- よくある間違い: ❌ "look forward to meet" → ✓ "look forward to meeting"

---
## ✏️ 理解度チェック

以下の質問に答えてください:

**Q1.** "look forward to" の後に来るのは？
   a) 動詞の原形  b) 動名詞 (-ing)  c) to不定詞

**Q2.** 穴埋め: I'm looking forward ___ your reply. (前置詞)

**Q3.** 誤りを直してください: "I look forward to meet you next week."

回答をお願いします
```

**User:** `Q1: b, Q2: to, Q3: I look forward to meeting you next week.`

**Claude:**
```
## 📊 結果

| 問題 | 結果 |
|------|------|
| Q1 | ✓ |
| Q2 | ✓ |
| Q3 | ✓ |

スコア: 3/3 (100%) 🎉

素晴らしい！完璧に理解できています。
"look forward to + 動名詞" のパターンをしっかりマスターしましたね。

【SRS追加】なし（全問正解のため）
```

## Guidelines

1. **Focus on learnable elements** - Prioritize idioms, phrasal verbs, and tricky grammar
2. **Explain in Japanese** - Main explanations in Japanese for clarity
3. **Provide context** - Always give additional example sentences
4. **Quiz what you explained** - Questions should directly relate to the breakdown
5. **Be encouraging** - Celebrate correct answers, gently explain mistakes

## Edge Cases

### Simple Sentence
If the sentence is very simple with nothing notable:
```
この文は基本的な構造です。特に難しい表現はありませんが、
確認のためクイズを出しますね。
```

### Very Complex Sentence
Focus on 3-5 most important elements, mention others briefly:
```
この文には多くの要素がありますが、特に重要な3つを解説します:
```

### Unclear Input
If input is not a valid English sentence:
```
英文を入力してください。例: /ringo-explain She couldn't help but smile.
```
