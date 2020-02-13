# topoi-core

This is the main codebase of Topoi Core. A simple, explicit core language for topoi programming language.

## What is Topoi Core?

Just like GHC's core, the Topoi core is the minimal representation of topoi programming (that syntax is yet to be decided). This allows us to proof the language semantics and focus on our type system.

## Code Structure

Topoi core is (now) written in Rust programming language. Our program only runs on the test case, which means you are going to directly write the AST in rust. We have a plan to support text based language file parser (see below).

## Roadmap

- [x] Reading the book 'The Little Typer'
- [ ] Decide the AST
- [ ] Decide the syntax of topoi core
- [ ] Parser
- [ ] Lexer / pretty printer / formatter

## Issue and Contribution

Please use Github issues for bug reports and requests.

To contribute directly, open a pull request on [Github repo]. Files must be contributed under the MIT license.

If you see something that can be improved, please contribute or contact us!
