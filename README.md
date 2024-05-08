# pl-zero-rs --- A PL/0 Compiler Written in Rust

```
::::::::: :::            :::::::::::
:+:    :+::+:           :+: :+:   :+:
+:+    +:++:+          +:+  +:+  :+:+
+#++:++#+ +#+         +#+   +#+ + +:+
+#+       +#+        +#+    +#+#  +#+
#+#       #+#       #+#     #+#   #+#
###       #######  ###       #######
```

## About

[PL/0](https://en.wikipedia.org/wiki/PL/0) is an instructional language devised by Niklaus Wirth, the inventor of Pascal, as a simple
enough language that producing a compiler for it could be done by a student as an exercise. Since then, many computer science students
have worked with this language in their senior-level Compiler Techniques or Programming Language Design classes. It has an unambiguous
syntax, and lends itself well to extension. In fact, this compiler could be developed into a Pascal compiler with relatively little effort.

## Usage

Run `cargo run -- -f [FILE.pl0]` to ~~compile~~ parse the PL/0 syntax. Code generation will be added in the future.
At the moment, it only verifies that the program is syntactically legal; it doesn't maintain a symbol table, or output PL/0 assembly code.

## Roadmap

In the future, this program will:
- [ ] Maintain a symbol table and disallow multiple variables with the same name.
- [ ] Output standard C89, making this a PL/0 ➜ C89 transpiler.
- [ ] Output PL/0 assembly code for the PL/0 virtual machine (unimplemented).
- [ ] Support a debug mode to give an insight into what the compiler is doing, and where it is finding errors.
- [x] Add a test battery. (Dr. Brian Callahan provides an excellent test suite in [his repo](https://github.com/ibara/pl0c) for his C-based compiler.)

And long-term, this PL/0 implementation will be extended to support:
- Input & Output
- String data types
- And more!
