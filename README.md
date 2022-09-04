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

- Preproccessor? (Put on Halt for now) :x:  
  - #include :x: 
  - #ifndef :x: 
  - #if :x: 
  - ... :x: 
- Lexer :heavy_check_mark: 
  - Allow Hex Numbers & Co. 
  - Sensible Error Generation :o: 
  - Fix Column Number Generation :o: 
- Parser :heavy_check_mark:
  - Expresions :heavy_check_mark:
  - Type Names :heavy_check_mark:
  - Declarations :heavy_check_mark:
  - Statements :heavy_check_mark: 
  - Typedef Check -> Context dependent Part :heavy_check_mark:
  - Full Parse for subset? of C11 Standard :warning:
  - Error Continuation :o:
- Enviroment Builder :warning:
  - Symbol Table extraction :warning:
  - Type Checking on AST :warning:
  - Constant Expression Runner :warning:
  - MIR generation :warning:
- MIR to cranelift backend :o:
- Custom MIR codegen backend :o:
  - Conversion to SSA :o:
  - SSA optimisier :o:
  - Register Allocator :o:
  - Code Generation from SSA :o:

---

## Implementation Details:

#### Lexer:
  - Only Integer/Float Numbers ([0-9+(.0-9*)?])


#### Parser:
  - Handwritten Recursive Descent Parser (Predictive: No Backtracking)
  - Only minimally tested, check src/parser/tests/ for tests


#### Enviroment Builder:
  - In Progress