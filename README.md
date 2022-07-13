```
      _             _       ____ ____ 
     | | __ _ _ __ | | __  / ___/ ___|
  _  | |/ _` | '_ \| |/ / | |  | |    
 | |_| | (_| | | | |   <  | |__| |___ 
  \___/ \__,_|_| |_|_|\_\  \____\____|

```
### A WIP minimal C Compiler written in Rust :]

---

## Features:

- [ ] Preproccessor? (Put on Low Priority for now)
  - [ ] #include
  - [ ] #ifndef
  - [ ] #if
  - [ ] ...
- [x] Lexer
  - [ ] Allow Hex Numbers & Co.
  - [ ] Sensible Error Generation
- [ ] Parser
  - [x] Expresions
  - [ ] Type Names
  - [ ] Declarations
  - [ ] Statements
  - [ ] Error Continuation
  - [ ] Full Parse for subset? of C11 Standard
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
 - Handwritten Recursive Descent Parser
 - Still in Progress