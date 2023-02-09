# Quiz-rs - Terminal quiz cards application

Quiz-rs is a quiz card application built within the terminal.


## Screenshots

![Example cards](https://user-images.githubusercontent.com/123209942/217742705-8db66fc1-6a64-4de1-80bc-3fa2010aa4d9.png)


## Features

- Create quiz cards in markdown
- Variety of card types
    - Multiple choice
    - Multiple answer
    - Flashcard
    - Fill in the blanks
    - Place in the correct order


## Run Locally

1. Clone the project

```bash
  git clone https://github.com/BrookJeynes/quiz-rs
```

2. Go to the project directory

```bash
  cd quiz-rs
```

3. Create a set of cards within `input.md` at the projects root. See the #Documentation section on card formatting

4. Start the application

```bash
  cargo run
```

## Documentation

Below is an example of `input.md`.

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

#### Attributions
This application was heavily inspired by [hascard](https://github.com/Yvee1/hascard).
