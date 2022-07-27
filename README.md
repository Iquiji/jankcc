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
- [x] Parser
  - [x] Expresions
  - [x] Type Names
  - [x] Declarations
  - [x] Statements
  - [x] Typedef Check -> Context dependent Part
  - [x] Full Parse for subset? of C11 Standard
  - [ ] Error Continuation
- [ ] Enviroment Builder
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
  - Only minimally tested, check src/parser/tests/ for tests


#### Enviroment Builder:
  - In Progress