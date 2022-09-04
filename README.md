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

- :x: Preproccessor? (Put on Halt for now)
  - :x: #include
  - :x: #ifndef
  - :x: #if
  - :x: ...
- :heavy_check_mark: Lexer
  - :x: Allow Hex Numbers & Co.
  - :x: Sensible Error Generation
  - :x: Fix Column Number Generation
- :heavy_check_mark: Parser
  - :heavy_check_mark: Expresions
  - :heavy_check_mark: Type Names
  - :heavy_check_mark: Declarations
  - :heavy_check_mark: Statements
  - :heavy_check_mark: Typedef Check -> Context dependent Part
  - :heavy_check_mark: Full Parse for subset? of C11 Standard
  - :x: Error Continuation
- :warning: Enviroment Builder
  - :warning: Symbol Table extraction
  - :warning: Type Checking on AST
  - :warning: Constant Expression Runner
  - :warning: MIR generation
- :x: MIR to cranelift backend
- :x: Custom MIR codegen backend
  - :x: Conversion to SSA
  - :x: SSA optimisier
  - :x: Register Allocator
  - :x: Code Generation from SSA

---

## Implementation Details:

#### Lexer:
  - Only Integer/Float Numbers ([0-9+(.0-9*)?])


#### Parser:
  - Handwritten Recursive Descent Parser (Predictive: No Backtracking)
  - Only minimally tested, check src/parser/tests/ for tests


#### Enviroment Builder:
  - In Progress