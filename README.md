# OxidizedLox: A Rusty Take on Lox Interpretation

Welcome to **OxidizedLox**, where the beauty of Lox meets the rugged charm of Rust. This is not just another interpreter—this is *the* interpreter, finely crafted with the precision of a blacksmith and the elegance of a cat. Whether you're here to learn, contribute, or just gawk at the code, you’re in for a treat.

## 📜 **Project Overview**

**OxidizedLox** is a Lox interpreter implemented in Rust, taking inspiration from Robert Nystrom’s *Crafting Interpreters*. In this journey, we’ve set out to create a Lox interpreter that’s both lightning-fast and a little bit cheeky. Think of it as Lox with a rust-colored patina.

But why Rust, you ask? Well, because Rust gives us the performance and safety we crave while making us feel like we’re living dangerously. It’s like riding a motorcycle with a helmet on—thrilling, yet secure.

### 🛠 **Lexing: The First Step in Our Grand Adventure**

Ah, lexing, the art of turning a stream of characters into a manageable array of tokens. It’s the first step in our journey from raw Lox code to beautiful execution. Here in OxidizedLox, the lexer is more than just a glorified word counter. It’s a sophisticated, high-class tokenizer with a penchant for elegance.

Here’s how it works:

1. **Input**: You feed it a string of Lox code. It could be Shakespearean, minimalist, or a train wreck of syntax. The lexer doesn’t judge (much).
2. **Processing**: The lexer reads through the string, character by character, and transforms it into a series of tokens. These tokens are categorized by their type (keywords, identifiers, literals, etc.).
3. **Output**: The result is a neat, tidy list of tokens that make the rest of the interpretation process feel like a walk in the park.

### 💡 **Why This Approach?**
- **Rust’s Safety Guarantees**: Memory safety, without the need for a garbage collector. Our lexer is practically bulletproof.
- **Performance**: Rust gives us speed, which means our Lox interpreter can run faster than a caffeinated squirrel.
- **Simplicity**: The lexer is built with simplicity in mind. We like to think of it as the Swiss Army knife of tokenizers—compact but capable.

## 🚀 **Current Features**

- **Tokenization**: Our lexer can currently handle the full Lox language, including:
  - Keywords (`if`, `else`, `for`, etc.)
  - Operators (`+`, `-`, `*`, `/`, `==`, etc.)
  - Delimiters (`(`, `)`, `{`, `}`, `;`, etc.)
  - Literals (Strings, Numbers)
  - Identifiers (Variable and function names)
- **Error Handling**: The lexer will politely inform you of any unexpected characters it encounters, with a healthy dose of sarcasm, of course.
- **EOF Handling**: We’re so thorough, we even know when the file ends.

## 🛣 **Future Plans**

Here’s where we’re headed next:

1. **Parsing**: Turning our tokens into an Abstract Syntax Tree (AST). Yes, the lexer was just the appetizer.
2. **Interpretation**: Actually running the code. Because what's the point of all this if it doesn’t do anything?
3. **REPL Support**: A Read-Eval-Print Loop for those who want to interpret Lox code in real-time. Think of it as talking to your interpreter, and it talks back.
4. **Optimization**: Because there’s always room for improvement. Even if it’s just squeezing out a few more nanoseconds.
5. **Extended Error Messages**: We plan to make our error messages as descriptive as a novel, complete with literary references and motivational quotes.

## 🤓 **Getting Started**

If you’re itching to get your hands dirty, here’s how you can start using **OxidizedLox**:

1. **Clone the Repo**: 
   ```sh
   git clone https://github.com/yourusername/rustylox.git
   cd rustylox
   ```

2. **Build the Project**: 
   ```sh
   cargo build --release
   ```

3. **Run the Interpreter**: 
   ```sh
   ./runner.sh tokenize your_file.lox
   ```

4. **Enjoy**: Sit back, relax, and watch as OxidizedLox does its magic.

## 🤝 **Contributing**

Got ideas? Found a bug? Want to add a feature? We’re all ears. Just make sure your code is as polished as your wit.

1. **Fork the Repo**
2. **Create a Branch**
3. **Submit a Pull Request**

## 🎩 **Final Thoughts**

OxidizedLox isn’t just an interpreter; it’s an adventure. It’s for those who like their code fast, safe, and just a bit irreverent. So, buckle up, because this is going to be a fun ride.

Remember, in OxidizedLox, the only thing more important than the code is the story we tell along the way. And maybe, just maybe, you’ll find that writing interpreters is more fun than you thought.

Happy interpreting!

---

*OxidizedLox - Because interpreting Lox should be both an art and a science.*