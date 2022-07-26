```
      _             _       ____ ____ 
     | | __ _ _ __ | | __  / ___/ ___|
  _  | |/ _` | '_ \| |/ / | |  | |    
 | |_| | (_| | | | |   <  | |__| |___ 
  \___/ \__,_|_| |_|_|\_\  \____\____|

```
### A WIP minimal hobby C Compiler written in Rust :]

---

## Features:

- [ ] Preproccessor? (Put on Halt for now)
  - [ ] #include
  - [ ] #ifndef
  - [ ] #if
  - [ ] ...
- [x] Lexer
  - [ ] Allow Hex Numbers & Co.
  - [ ] Sensible Error Generation
  - [ ] Fix Column Number Generation
- [ ] Parser
  - [x] Expresions
  - [x] Type Names
  - [x] Declarations
  - [x] Statements
  - [ ] Typedef Check -> Context dependent Part
  - [ ] Full Parse for subset? of C11 Standard
  - [ ] Error Continuation
- [ ] Type Checking on AST
- [ ] Constant Expression Runner
- [ ] Conversion to SSA
- [ ] SSA optimisier
- [ ] Register Allocator
- [ ] Code Generation from SSA

---

## Implementation Details:

#### Lexer:
  - Only Integer/Float Numbers ([0-9+(.0-9*)?])


#### Parser:
 - Handwritten Recursive Descent Parser (Predictive: No Backtracking)
 - Still in Progress -> Working on Statement Parsing now!
