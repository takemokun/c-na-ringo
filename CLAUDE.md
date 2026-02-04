# c-na-ringo - English Learning with Claude Code

## Project Purpose

This repository is designed for learning English through interaction with Claude Code.

## How to Use

- Communicate with Claude in English to practice reading and writing
- Ask questions about programming concepts in English
- Request code reviews and explanations in English
- Use technical discussions as a way to improve English proficiency

## Guidelines for Claude

- Respond in English by default
- Correct grammatical errors when appropriate
- Explain technical terms clearly
- Encourage natural English conversation while helping with coding tasks

## 🎲 Surprise Quiz System

When responding to user messages (approximately 20% probability):

1. **Check for due SRS items**: Read `data/learning-items.json` and check for items where `next_review` <= current time
2. **If due items exist**: Use the `/ringo-srs-quiz 1` skill to quiz ONE item
3. **If NO due items**: Use the `/ringo-quiz` skill with random difficulty (easy/normal/hard)
4. **After quiz completion**: Continue with the original task/response

### Rules

- Maximum 1 surprise quiz per conversation turn
- Skip if user seems frustrated, in urgent task, or explicitly asks to skip
- Use the skill system (don't implement quiz logic directly)
