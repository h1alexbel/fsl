# fsl. FakeHub State Language

[![EO principles respected here](https://www.elegantobjects.org/badge.svg)](https://www.elegantobjects.org)
[![DevOps By Rultor.com](http://www.rultor.com/b/h1alexbel/fsl)](http://www.rultor.com/p/h1alexbel/fsl)
[![We recommend IntelliJ IDEA](https://www.elegantobjects.org/intellij-idea.svg)](https://www.jetbrains.com/idea/)

[![just](https://github.com/h1alexbel/fsl/actions/workflows/just.yml/badge.svg)](https://github.com/h1alexbel/fsl/actions/workflows/just.yml)
[![Crates.io Version](https://img.shields.io/crates/v/fsl)](https://crates.io/crates/fsl)
[![codecov](https://codecov.io/gh/h1alexbel/fsl/graph/badge.svg?token=0bcdqd2UKT)](https://codecov.io/gh/h1alexbel/fsl)
[![PDD status](http://www.0pdd.com/svg?name=h1alexbel/fsl)](http://www.0pdd.com/p?name=h1alexbel/fsl)
[![Hits-of-Code](https://hitsofcode.com/github/h1alexbel/fsl)](https://hitsofcode.com/view/github/h1alexbel/fsl)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](https://github.com/h1alexbel/fsl/blob/master/LICENSE.txt)
[![Known Vulnerabilities](https://snyk.io/test/github/h1alexbel/fsl/badge.svg)](https://snyk.io/test/github/h1alexbel/fsl)

FSL is a specific [DSL] for managing state inside [fakehub].

**Motivation**. When working with [fakehub], automated testing tool that mocks
GitHub REST API, we often require to setup initial state of the system with
testing data. Instead of repeating testing preparations, we developed
FSL, a language that takes file as an input, processes it, and creates FakeHub
instance with desired data.

## Quick Start

TBD..

## How to Use

TBD..

## How to contribute?

Make sure that you have [Rust], [just]+ installed on your system, then fork
this repository, make changes, send us a [pull request][guidelines]. We will
review your changes and apply them to the `master` branch shortly, provided
they don't violate our quality standards. To avoid frustration, before sending
us your pull request please run full build:

```bash
just full
```

[fakehub]: https://github.com/h1alexbel/fakehub
[DSL]: https://en.wikipedia.org/wiki/Domain-specific_language
[guidelines]: https://www.yegor256.com/2014/04/15/github-guidelines.html
[Rust]: https://www.rust-lang.org/tools/install
[just]: https://just.systems/man/en/chapter_4.html
