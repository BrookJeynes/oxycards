# Cards

## Global Formatting

Cards must be prefixed with their type. Here is a list of all available card types:
- flashcard
- multiple_choice
- multiple_answer
- fill_in_the_blanks
- order

When creating more than one card, they must be seperated by a triple dash (`---`).

## Example

```md
flashcard

# Word or question
Explanation or definition of this word, or the answer to the question.

---

multiple_choice

# Multiple choice question - (correct answer is denoted by an *)
* Choice 1
- Choice 2
- Choice 3
- Choice 4

---

multiple_answer

# Multiple answer question - (correct answers is denoted by an *)
[*] Option 1
[ ] Option 2
[ ] Option 3
[*] Option 4

---

fill_in_the_blanks

# Fill in the gaps
The word chook, also know as _chicken_, is a word commonly used in _AUS|Australia_.
---

order

# Order the numbers from largest to smallest - (options are placed in the correct ordering and are shuffed within the application)
1. 100000
2. 4235
3. 23
4. 6
```

## Global Controls

| Key     | Description      |
|---------|------------------|
| q       | Quit Application |
| \<Enter\> | Validate Answer  |
