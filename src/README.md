# sexpr

> S Expression parser

## Overview

Note when developing this S expression parser:

- Tokenization, a.k.a lexing. The input is transformed into a stream of tokens. Tokenization never fails, although the output may contain error tokens.
