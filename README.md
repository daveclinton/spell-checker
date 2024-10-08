# SpellChecker

A simple spell checker written in Rust that suggests corrections for misspelled words based on word frequency.

## Features

- **Word Frequency Analysis**: Processes a large text file to build a frequency dictionary.
- **Candidate Generation**: Generates possible corrections by considering edits like deletions, transpositions, replacements, and insertions.
- **Interactive CLI**: Allows users to input words and receive corrected suggestions.

## Usage

### Building the Project

Ensure you have Rust installed. Clone the repository and build the project:

```bash
git clone https://github.com/yourusername/spell_checker.git
cd spell_checker
cargo build --release
