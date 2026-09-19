# Rust na prática

Códigos, exemplos e projetos da jornada aprendendo Rust, acompanhando os vídeos do canal [@israelguido](https://www.youtube.com/@israelguido) no YouTube.

[![GitHub forks](https://img.shields.io/github/forks/israelguido/rust-na-pratica?style=flat&logo=github)](https://github.com/israelguido/rust-na-pratica/forks)
[![GitHub stars](https://img.shields.io/github/stars/israelguido/rust-na-pratica?style=flat&logo=github)](https://github.com/israelguido/rust-na-pratica/stargazers)
[![GitHub downloads](https://img.shields.io/github/downloads/israelguido/rust-na-pratica/total?style=flat&logo=github)](https://github.com/israelguido/rust-na-pratica/releases)
[![GitHub last commit](https://img.shields.io/github/last-commit/israelguido/rust-na-pratica?style=flat)](https://github.com/israelguido/rust-na-pratica/commits/main)

## Índice das aulas

| # | Aula | Pasta | O que você aprende |
|---|------|--------|---------------------|
| 1 | [Hello World](#1-hello-world) | [`hello-world/`](./hello-world) | Primeiro binário com Cargo e `println!` |
| 2 | [API simples](#2-api-simples) | [`api-simples/`](./api-simples) | Servidor HTTP com Axum, Tokio e JSON |

## 1. Hello World

Pasta: [`hello-world/`](./hello-world)

Primeiro programa em Rust: um `main` que imprime `Hello, world!`.

```bash
cd hello-world
cargo run
```

## 2. API simples

Pasta: [`api-simples/`](./api-simples)

API mínima com [Axum](https://docs.rs/axum): rota `GET /hello` devolvendo JSON na porta `3000`.

```bash
cd api-simples
cargo run
```

Depois, em outro terminal:

```bash
curl http://127.0.0.1:3000/hello
```

Resposta esperada:

```json
{"message":"Hello, World!"}
```

## Como usar este repositório

```bash
git clone https://github.com/israelguido/rust-na-pratica.git
cd rust-na-pratica
```

Cada aula é um crate independente. Entre na pasta da aula e rode `cargo run`.

## Canal

Vídeos no YouTube: [youtube.com/@israelguido](https://www.youtube.com/@israelguido)
