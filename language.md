# Objection Programming Language

## Introduction
Objection is an object oriented programming language where everything is strictly typed (including functions and methods). 

*For now all types will need to be stated explicity, but in the future, maybe I can add some type inference as an added challenge.*

## Vision and goal:
The goal is to eventually write objection's compiler in itself.
- Is this a good language? <br/>Probably not.
- Is it efficient? <br />Also highly unlikely.
- Is it readable? <br />For some people, yes. But it is also highly verbose and unpleasant to look at for many.
- What's the point? <br />I just want to try and learn new things. In this case, I want to learn:
    - The rust programming language. (Which I'll be using to write the compiler even though I haven't written a single line of rust code yet)
    - How to design a programming language.
    - Vim for VSCode (to learn about and get used to vim motions and commands)
    - How to write a compiler.
        - Lexing, parsing, and code generation.
        - Some basic assembly (to which I'll be compiling the language).
    - How to write a language server extension for VSCode (including syntax highlighting).
    - Maybe some other things along the way...

A valid program in objection should look like this (subject to change):
```
class Point {
    prv x: int;
    prv y: int;

    // Special constructor method
    pub Point(x: int, y: int): Point = {
        this.x = x;
        this.y = y;
    }

    // Overloading is allowed
    pub Point(val: int): Point = {
        this.x = val;
        this.y = val;
    }

    pub distance(other: Point): float = {
        x_dis: int = this.x - other.x;
        y_dis: int = this.y - other.y;
        return sqrt(x_dis * x_dis + y_dis * y_dis);
    }
}

// Entry point for program
main(): int = {
    a: Point = Point(3, 4);     // Invokes the first constructor
    // a.x                      // Throw error -> x is private
    origin: Point = Point(0);   // Invokess the second constructor
    print(a.distance(origin));  // Should print out 5.0 to the console
    return 0;
}
```

## Design Document (WIP)

### Types

#### Primitive types
- int  (64 bit signed integer)
- float (64 bit floating point number)
- bool (true or false)
- char (8 bit character)

#### Special types
- void (no value)
    - *Not sure how to even make this, but I'll figure it out when I get there*

#### Complex types
- Array (fixed size list of a single type)
    - `Arr: [int; 5];` will create an array of 5 *default* integers
- List (dynamic size list of a single type)
    - `Lst: [int];` will create an empty list of integers
- String (dynamic size list of characters)
    - `Str: string;` will create an empty string
- function (a block of code that can be called)
    - `func(x: int): int = return x + 1;` will create a function that takes an integer and returns an integer. The type is stored internally as `(int) -> int`. (Takes one integer and returns one integer)
    - `func(x: int, y: string, z: float): bool = { return x > 5 && y == "hello" && z < 3.14; }` has the type `(int, string, float) -> bool`. (Takes an integer, a string, and a float and returns a boolean)
    - `func(fn: (int) -> float): int = return fn(5);` has the type `((int) -> float) -> int`. (Take a function that takes an integer and returns a float and returns an integer)
    - `func(x: int): (int) -> int = { return (y: int): int = return x + y; }` has the type `(int) -> (int) -> int`. (Takes an integer and returns a function that takes an integer and returns an integer)

#### User defined types
- class (a blueprint for an object)
    - `class AClass { }` will create a new "type" called AClass
    - class members (including attributes and methods) can be public or private. (Maybe also protected in the future(?))

### Syntax and Semantics

#### Expressions
An expression is a segment of code that either has a literal value, or evaluates to a value.
Expressions can be:
- Literals.
    - ex: `5` is a literal integer.
    - ex: `3.14` is a literal float.
    - ex: `true` is a literal boolean.
    - ex: `'a'` is a literal character.
    - ex: `"Hello, World!"` is a literal string.
- Variables (any non keyword identifier).
    - ex: `x` is a variable.
    - ex: `time_elapsed` is a variable.
- Unary operations.
    - ex: `-5`.
    - ex: `~20`.
    - In general, unary expressions are in the form '$\text{\textcolor{green}{Operand} expr}$' where '$\text{\textcolor{green}{Operand}}$' is the operator and '$\text{expr}$' is another valid expression.
- Binary operations.
    - ex: `5 + 3`.
    - In general, binary expressions are in the form '$\text{expr}_1 \text{\textcolor{green}{Operand} expr}_2$' where '$\text{\textcolor{green}{Operand}}$' is the operator and '$\text{expr}_1$' and '$\text{expr}_2$' are two other valid expressions.
- Parenthesized expressions.
    - ex: `(3 * 1)`.
    - In the form '$(\text{expr})$' where '$\text{expr}$' is another valid expression.
    - Used to change the order of operations.
- Function calls.
    - ex: `add(3, 4)`.
    - In the form '$\text{func}(\text{arg}_1, \text{arg}_2, \ldots, \text{arg}_n)$' where '$\text{func}$' is a valid function and '$\text{arg}_1, \text{arg}_2, \ldots, \text{arg}_n$' are valid expressions.
    - The function must be defined before it is called.
- There is an operation precedence that all expressions follow.
    - The highest precedence is literal and variable expressions (which don't need to be evaluated).
    - The next highest precedence is function calls.
    - The next highest precedence is parenthesized expressions.
    - The next highest precedence is unary expressions.
    - The next highest precedence is binary expressions, which have some extra rules:
        - `[*, /, %, +, -]` is the order of precedence for binary expressions from highest to lowest (left to right).
        - *TODO: Add more precedence rules (?).*
- All expressions inherently have a type (in the code, this will be referred to as the `meta_type`)

#### Statements
A statement is a segment of code that performs an action. All statements must end with a semicolon.
Statements can be:
- Variable assignment.
    - Default declarations are not allowed. (e.g. `x: int;` is not allowed)
    - ex: `x: int = 5;` will create a new variable named `x` of type `int` and assign it the value `5`.
    - Reassignment is allowed: `x: int = 5; x = 6;`
    - we can use the keyword const to make a variable immutable: `const x: int = 5;`
        - An immutable variable cannot be reassigned.
    - The value to the right of the assignment operator must be a valid expression of the correct type.
- Return statement.
    - ex: `return res;` will return the value `res` from the current function.
- Dangling expressions are allowed.
    - ex: `5;` will evaluate to `5` and do nothing.
    - Function calls are also allowed as dangling expressions, but the return value is ignored.
- Functions are a special king of assignment statement.
    - Functions are defined using an identifier followed by parentheses containing a comma separated list of parameters, followed by a colon and the return type, followed by an equal sign and a block of code.
        - ex: `func(x: int): int = { return x + 1; }`

##### Control Flow
Special statements that control the flow of the program (and do not end with a semicolon).
- If statements.
    - ex: `if (x > 5) { return true; }`
    - ex: `if (x > 5) { return true; } else { return false; }`
- While loops.
    - ex: `while (x < 5) { x = x + 1; }`
    - break and continue statements.
        - ex: `break;`
            - Exits the loop when encountered.
        - ex: `continue;`
            - Skips the rest of the loop body and goes to the next iteration when encountered.
- For loops.
    - *Maybe in the future(?)*

#### Comments and Whitespace
- Single line comments are denoted by `//`.
    - ex: `// This is a comment`
    - ex: `x: int = 5; // This is a comment after a statement`
- Multi-line comments are started with `/*` and ended with `*/`, cannot be nested, and can span multiple lines.
    - ex: `/* This is a multi-line comment */`
    - ex: `x: int = 5; /* This is a multi-line comment after a statement */`
    - ex: `x: int = /* This is a comment inside a statement */ 5;`
- Comments have no effect on the program and are ignored by the compiler (On the lexer level).
- Whitespace is also ignored by the compiler (On the lexer level).

#### Scopes
- Scopes are denoted by curly braces `{}`.
- The function body is a scope.
- The body of an if statement is a scope.
- The body of a while loop is a scope.
- Scopes can also be manuakky defined without a control flow statement.
    - ex: `x: int = 5; { y: int = 6; print(y + x); }`. This will print `11` to the console because x is defined in the outer scope, and the inner scope can access it.
    - Alternatively, `x: int = 5; { y: int = 6; } print(y + x);` will throw an error because `y` is not defined in the outer scope.
- Variables defined within a scope are only accessible within that scope. (With an exception for class members if they are public)

#### Classes
- Classes are defined using the `class` keyword followed by an identifier.
    - ex: `class AClass { }`
- Classes can have members (attributes and methods).
- Members can be public or private.
    - Public members can be accessed from outside the class, and are denoted by the `pub` keyword.
    - Private members can only be accessed from within the class, and are denoted by the `prv` keyword.
- Classes have methods that are defined in the same way as functions.
    - ex: `class AClass { pub func(x: int): int = { return x + 1; } }`
    - Methods can access class members regardless of their visibility.
    - Methods can be overloaded.
- Classes have a special method called the constructor.
    - The constructor is a method that is called when an object of the class is created.
    - The constructor has the same name as the class.
- A class can inherit from up to one other class, and no more than one.
    - ex: `class BClass: AClass { }`
    - The subclass inherits all members of the superclass.
    - The subclass can override superclass methods.
    - The subclass can call superclass methods using the `super` keyword.
- Classes can have static members.
    - *Maybe in the future(?)*
- Classes can be abstract.
    - *Maybe in the future(?)*

#### Enumerations
- Using the keyword enum, we can define a new type that can only have a set of predefined values.
    - ex: `enum Color { RED, GREEN, BLUE }`
    - The values of an enumeration are accessed using the dot operator.
        - ex: `Color.RED`
    - By default, the values of an enumeration are integers starting at 0.
        - *Maybe change this behavior in the future(?)*

#### Interfaces
- *Maybe in the future(?)*

### Memory Management
- *For now, it'll be memory leaks galore, but I'll figure out memory once I implement complex types.*

### Modules and Imports
- *Maybe in the future(?)*

### Error Handling
- *I think exceptions are a good idea, but I'll figure it out when I get there.*

## Roadmap
I should probably add dates to these tasks, but since I'm working full time, and I have no help, I'll just add the date of completion as I go.
1. [x] Write the design document
2. [ ] Make a compiler that can compile:
    1. [ ] a program that understands function calls (no params), and the return statement.
    2. [ ] a program that understand assignment and reassignment statements.
    3. [ ] a program that understands all the primitive types.
    4. [ ] a program that understands all kinds of expressions and does the correct type checking.
    5. [ ] a program that understands conditional if else statements.
    6. [ ] a program that understands while loops including break and continue statements.
    7. [ ] a program that understands functions with parameters.
    8. [ ] a program that understands fixed size arrays.
    9. [ ] a program that understands dynamic size lists.
    10. [ ] a program that understands strings.
    11. [ ] a program that understands functions with no return type.
    12. [ ] a program that understands enums.
    13. [ ] a program that understands classes and class members.
    14. [ ] a program that understands class methods.
3. [ ] Syntax highlighting for the language in VSCode.
4. [ ] Write a simple program in the language.
5. [ ] Do some DSA with the language.
6. [ ] Write the language's compiler in itself.
