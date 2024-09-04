# RustyLox: A Rusty Take on Lox Interpretation

Welcome to **RustyLox**, where the beauty of Lox meets the rugged charm of Rust. This is not just another interpreter—this is *the* interpreter, finely crafted with the precision of a blacksmith and the elegance of a cat. Whether you're here to learn, contribute, or just gawk at the code, you’re in for a treat.

## 📜 **Project Overview**

**RustyLox** is a Lox interpreter implemented in Rust, inspired by Robert Nystrom’s *Crafting Interpreters*. Our goal is to create a Lox interpreter that's both lightning-fast and a little bit cheeky. Think of it as Lox with a rust-colored patina.

**Update**

As of 03/09/2024 Rustylox supports a [live playground](https://mvishiu11.github.io/rustylox-playground)! Check it out!

## 📑 **Table of Contents**

- [📜 Project Overview](#-project-overview)
  - [Current gramar](#current-grammar-in-ebnf)
  - [Quick feature overview](#quick-feature-overview)
- [🚀 Current Features](#-current-features)
- [🛠 Lexing: The First Step in Our Grand Adventure](#-lexing-the-first-step-in-our-grand-adventure)
  - [How it works?](#how-it-works)
  - [Technical Details](#technical-details)
- [🛠 Parsing: Turning Tokens into Meaningful Expressions](#-parsing-turning-tokens-into-meaningful-expressions)
  - [How it works?](#how-it-works-1)
  - [Technical Details](#technical-details-1)
- [🛣 Future Plans](#-future-plans)
- [🤓 Getting Started](#-getting-started)
- [🤝 Contributing](#-contributing)
- [🎩 Final Thoughts](#-final-thoughts)

## 🚀 **Current Features**

### Current grammar in [EBNF](https://en.wikipedia.org/wiki/Extended_Backus%E2%80%93Naur_form)

```ebnf
program      = { declaration }, EOF ;

declaration  = varDecl 
             | statement ;

varDecl      = "var", IDENTIFIER, [ "=" expression ], ";" ;

statement    = exprStmt 
             | printStmt 
             | whileStmt
             | forStmt
             | ifStmt
             | block 
             | breakStmt
             | continueStmt ;

breakStmt    = "break", ";" ;

continueStmt = "continue", ";" ;

block        = "{", { declaration }, "}" ;

exprStmt     = expression, ";" ;

printStmt    = "print", expression, ";" ;

whileStmt    = "while", "(", expression, ")", statement ;

forStmt      = "for", "(", ( varDecl | exprStmt | ";" ),
                [expression], ";",
                [expression], ")", statement ;

ifStmt       = "if", "(", expression, ")" statement
               [ "else", statement ] ; 

expression   = assignment ;

assignment   = IDENTIFIER, "=", assignment
             | logic_or ;

logic_or     = logic_and, { "or", logic_and } ;

logic_and    = equality, { "and", equality } ;

equality     = comparison { ( "!=" | "==" ) comparison } ;

comparison   = term { ( ">" | ">=" | "<" | "<=" ) term } ;

term         = factor { ( "-" | "+" ) factor } ;

factor       = unary { ( "/" | "*" | "%" ) unary } ;

unary        = ( "!" | "-" ) unary
             | primary ;

primary      = NUMBER 
             | STRING 
             | "true" 
             | "false" 
             | "nil"
             | "(" expression ")" 
             | IDENTIFIER ;
```

### Quick feature overview
- **Tokenization**: Efficiently processes the Lox language, covering:
  - **Keywords**: Recognizes reserved words such as `if`, `else`, `for`, `while`, `class`, `return`, and others.
  - **Operators**: Identifies arithmetic operators (`+`, `-`, `*`, `/`), relational operators (`==`, `!=`, `<`, `>`, `<=`, `>=`), logical operators (`and`, `or`), and assignment operators (`=`, `+=`, `-=`).
  - **Delimiters**: Handles punctuation and delimiters including parentheses (`(`, `)`), braces (`{`, `}`), brackets (`[`, `]`), commas (`,`), and semicolons (`;`).
  - **Literals**: Supports string literals, numeric literals (integers and floating-point numbers), and boolean literals (`true`, `false`).
  - **Identifiers**: Detects and tokenizes variable names, function names, and other user-defined identifiers.

- **Parsing**: Capable of interpreting the Lox language syntax, including:
  - **Basic Arithmetic**: Parses expressions involving addition (`+`), subtraction (`-`), multiplication (`*`), and division (`/`), and supports proper precedence and associativity rules.
  - **Parenthesized Expressions**: Handles nested expressions within parentheses, ensuring correct order of operations.
  - **Unary Operations**: Interprets unary operators such as negation (`-`) and logical not (`!`).
  - **Binary Operations**: Supports binary operations with appropriate precedence levels.
  - **Grouping**: Recognizes and correctly processes expressions grouped within parentheses for clarity and precedence.
  - **Assignment Statements**: Parses assignment operations to variables, including compound assignments like `+=` and `-=`.

- **Error Handling**: Features robust error reporting with a touch of humor, providing clear and contextually relevant messages for unexpected characters, invalid syntax, and other anomalies. Includes mechanisms for graceful recovery and informative feedback.

- **EOF Handling**: Accurately detects and manages the end-of-file condition, ensuring that tokenization and parsing are correctly terminated and that no residual tokens are left unprocessed.

- **User-Friendly Debugging**: Offers detailed diagnostic information for developers, including line numbers and token details, to facilitate efficient debugging and development.

- **Interactive Feedback in CLI**: Our CLI provides real-time feedback during development, allowing users to see immediate results and identify issues as they arise.

## 🛠 **Lexing: The First Step in Our Grand Adventure**

Ah, lexing, the art of turning a stream of characters into a manageable array of tokens. It’s the first step in our journey from raw Lox code to beautiful execution. Here in RustyLox, the lexer is more than just a glorified word counter. It’s a sophisticated, high-class tokenizer with a penchant for elegance.

### How It Works

1. **Input**: You feed it a string of Lox code. It could be Shakespearean, minimalist, or a train wreck of syntax. The lexer doesn’t judge (much).
2. **Processing**: The lexer reads through the string, character by character, and transforms it into a series of tokens. These tokens are categorized by their type (keywords, identifiers, literals, etc.).
3. **Output**: The result is a neat, tidy list of tokens that make the rest of the interpretation process feel like a walk in the park.

**Example**

For input such as this *test.lox* file:

```js
var greeting = "Hello, world!";
print greeting;
if (greeting != "Hello, world!") {
    print "Something went wrong.";
} else {
    print "All good!";
}
```

We run the following command:

```bash
./rustylox.sh tokenize "example/test_tokenize.lox"
```

And get the output:

```bash
Token { token_type: Var, lexeme: "var", line: 1 }
Token { token_type: Identifier, lexeme: "greeting", line: 1 }
Token { token_type: Equal, lexeme: "=", line: 1 }
Token { token_type: String, lexeme: "\"Hello, world!\"", line: 1 }
Token { token_type: Semicolon, lexeme: ";", line: 1 }
Token { token_type: Print, lexeme: "print", line: 2 }
Token { token_type: Identifier, lexeme: "greeting", line: 2 }
Token { token_type: Semicolon, lexeme: ";", line: 2 }
Token { token_type: If, lexeme: "if", line: 3 }
Token { token_type: LeftParen, lexeme: "(", line: 3 }
Token { token_type: Identifier, lexeme: "greeting", line: 3 }
Token { token_type: BangEqual, lexeme: "!=", line: 3 }
Token { token_type: String, lexeme: "\"Hello, world!\"", line: 3 }
Token { token_type: RightParen, lexeme: ")", line: 3 }
Token { token_type: LeftBrace, lexeme: "{", line: 3 }
Token { token_type: Print, lexeme: "print", line: 4 }
Token { token_type: String, lexeme: "\"Something went wrong.\"", line: 4 }
Token { token_type: Semicolon, lexeme: ";", line: 4 }
Token { token_type: RightBrace, lexeme: "}", line: 5 }
Token { token_type: Else, lexeme: "else", line: 5 }
Token { token_type: LeftBrace, lexeme: "{", line: 5 }
Token { token_type: Print, lexeme: "print", line: 6 }
Token { token_type: String, lexeme: "\"All good!\"", line: 6 }
Token { token_type: Semicolon, lexeme: ";", line: 6 }
Token { token_type: RightBrace, lexeme: "}", line: 7 }
Token { token_type: Eof, lexeme: "", line: 7 }
```

As you can see, it works great!

### Technical Details

#### **Lexer Implementation**

- **Deterministic Finite Automaton with 1-Lookahead (DFA-1)**: Our lexer employs a Deterministic Finite Automaton (DFA) enhanced with a single-character lookahead capability. This sophisticated mechanism enables precise and efficient token classification by examining the upcoming character to resolve ambiguities in token boundaries and types. The DFA-1 framework ensures that the lexer can handle the regular patterns present in the Lox language with optimal performance, leveraging its deterministic nature to guarantee linear time complexity in tokenization.

- **Regular Language Processing**: The Lox language is defined as a regular language within the lexical domain. This classification facilitates the application of FSM-based techniques, allowing the lexer to operate with exceptional efficiency and accuracy. The regularity of Lox's lexical structure ensures that our DFA-1 approach is both theoretically sound and practically effective in managing token streams.

**Further Reading**

- [Deterministic Finite Automaton](https://en.wikipedia.org/wiki/Deterministic_finite_automaton): A comprehensive overview of deterministic finite automata and their role in formal language theory.
- [Finite State Machines](https://en.wikipedia.org/wiki/Finite-state_machine): An introduction to finite state machines, their properties, and applications.
- [Tokenization](https://en.wikipedia.org/wiki/Lexical_analysis): A detailed discussion on lexical analysis and the process of tokenization in programming languages.

## 🛠 **Parsing: Turning Tokens into Meaningful Expressions**

Welcome to the world of parsing, where we turn our neatly categorized tokens into structured, meaningful expressions. In RustyLox, parsing transforms tokens into an Abstract Syntax Tree (AST), a hierarchical representation of your code that reflects its syntactic structure.

### How It Works

1. **Input**: We start with a sequence of tokens produced by the lexer.
2. **Processing**: The parser applies grammatical rules to these tokens to construct an AST. This involves handling expressions, statements, and control flow constructs.
3. **Output**: The result is an AST that represents the structure of the code, enabling further stages of interpretation or compilation.

**Example**

For the simpler *test.lox* file (since for now we only support parsing arithmetic):

```js
// Simple arithmetic
10 + 2 * (3 - 1);
```

We run:

```bash
./rustylox.sh parse "example/test_parse_arithmetic.lox"
```

The parser would produce an AST like:

```
BinaryExpression (Plus)
├──   Number (10)
└──   BinaryExpression (Star)
  ├──     Number (2)
  └──     Grouping
    └──       BinaryExpression (Minus)
      ├──         Number (3)
      └──         Number (1)
```

### Technical Details

#### **Parser Implementation**

- **Recursive Descent Parsing with Context-Free Grammar**: The Lox language is modeled as a context-free language, which aligns perfectly with recursive descent parsing techniques. This parsing strategy utilizes a collection of mutually recursive functions, each designed to recognize and process different non-terminals defined by the language's grammar. The recursive descent parser systematically traverses the input token stream to construct a syntax tree, reflecting the nested and hierarchical structure of Lox expressions and statements. This approach provides a straightforward yet powerful mechanism for syntax analysis, enabling the parser to effectively handle the complex constructs inherent in context-free grammars.

- **Advanced Error Handling**: Our parser incorporates advanced error handling mechanisms that significantly enhance its robustness and user-friendliness. By employing comprehensive error recovery techniques and providing detailed error messages, the parser ensures that syntax errors are reported with contextually relevant information, facilitating easier debugging and correction. The error handling framework gracefully manages unexpected tokens and invalid syntax, maintaining parser stability and delivering a seamless development experience.

**Further Reading**

- [Recursive Descent Parsing](https://en.wikipedia.org/wiki/Recursive_descent_parser): An in-depth exploration of recursive descent parsing techniques and their application in language processing.
- [Abstract Syntax Trees](https://en.wikipedia.org/wiki/Abstract_syntax_tree): A detailed explanation of abstract syntax trees and their role in representing hierarchical language structures.
- [Context-Free Grammars](https://en.wikipedia.org/wiki/Context-free_grammar): An overview of context-free grammars, their theoretical foundations, and their significance in formal language theory.

Certainly! Here’s a professional and detailed section on interpreter implementation, styled similarly to your lexing section:

---

## 🎭 **Interpreting: Breathing Life into Tokens**

Ah, interpreting, where the magic of execution happens! It’s the enchanting process that turns our neat tokens and well-formed expressions into actual results. In RustyLox, the interpreter isn’t just a mechanical evaluator; it’s a thoughtful and precise calculator with a flair for handling both simple arithmetic and complex expressions.

### How It Works

1. **Input**: You provide it with an abstract syntax tree (AST) of Lox expressions, the result of the parsing phase. This AST represents your Lox code's structure and logic, ready for evaluation.
2. **Processing**: The interpreter traverses this AST, evaluating each node according to its type—whether it's a literal value, a unary operation, a binary operation, or a grouped expression.
3. **Output**: The result is a computed value, an error, or sometimes a sophisticated calculation that reflects the logic encoded in your Lox program.

**Example**

Consider this simple Lox expression:

```js
- (3 + 5) * 2
```

The output would be:

```bash
-16
```

### Technical Details

#### **Interpreter Implementation**

- **Expression Evaluation**: Our interpreter evaluates Lox expressions by recursively traversing the AST. It handles different expression types such as literals, unary operations, binary operations, and groupings. This method ensures that complex expressions are broken down into simpler, manageable computations.

- **Error Handling**: The interpreter is equipped with robust error handling to gracefully manage issues like type errors, division by zero, and unknown operators. This resilience ensures that any deviations from expected behavior are reported with clear and informative error messages.

- **Floating-Point Arithmetic**: For numerical operations, the interpreter performs floating-point arithmetic, supporting various binary operations like addition, subtraction, multiplication, and division. It also handles comparison operations, providing results in the form of floating-point numbers.

- **Logical Operations**: Unary operations like logical NOT and negation are implemented with care, ensuring accurate Boolean evaluations and numeric negations. 

**Further Reading**

- [Abstract Syntax Trees (AST)](https://en.wikipedia.org/wiki/Abstract_syntax_tree): An introduction to abstract syntax trees and their role in representing the structure of source code.
- [Expression Evaluation](https://en.wikipedia.org/wiki/Expression_(computer_science)): A detailed look at the evaluation of expressions and the principles behind expression processing.
- [Error Handling in Rust](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html): A guide to error handling in Rust, including the use of `Result` and `Option` types.

## 🛣 **Future Plans**

Here’s where we’re headed next:

1. **Interpretation**: Implement the execution of Lox code based on the AST.
1. **REPL Support**: Introduce a Read-Eval-Print Loop for real-time Lox code interpretation.
1. **Optimization**: Enhance performance to achieve even faster execution times.
1. **Extended Error Messages**: Provide detailed and descriptive error messages to improve debugging.

## 🤓 **Getting Started**

If you’re itching to get your hands dirty, here’s how you can start using **RustyLox**:

1. **Clone the Repo**:
   ```sh
   git clone https://github.com/yourusername/rustylox.git
   cd rustylox
   ```

1. **Run the Interpreter**:
   ```sh
   ./rustylox.sh interpret your_file.lox
   ```

1. **Or launch our friendly CLI!**
```sh
./rustylox.sh cli
```

In this case you'll see:

```sh
🚀 Welcome to the Lox programming language REPL!
✨ Program logs will be displayed here. Stay tuned!
> 
```

1. **Enjoy**: Sit back, relax, and watch as RustyLox does its magic.

## 🤝 **Contributing**

Got ideas? Found a bug? Want to add a feature? We’re all ears. Just make sure your code is as polished as your wit.

1. **Fork the Repo**
2. **Create a Branch**
3. **Submit a Pull Request**

## 🎩 **Final Thoughts**

RustyLox isn’t just an interpreter; it’s an adventure. It’s for those who like their code fast, safe, and just a bit irreverent. So, buckle up, because this is going to be a fun ride.

Remember, in RustyLox, the only thing more important than the code is the story we tell along the way. And maybe, just maybe, you’ll find that writing interpreters is more fun than you thought.

Happy interpreting!

---

*RustyLox - Because interpreting Lox should be both an art and a science.*