# Rust na prática

Códigos, exemplos e projetos da jornada aprendendo Rust, acompanhando os vídeos do canal [@israelguido](https://www.youtube.com/@israelguido) no YouTube.

[![GitHub forks](https://img.shields.io/github/forks/israelguido/rust-na-pratica?style=flat&logo=github)](https://github.com/israelguido/rust-na-pratica/forks)
[![GitHub stars](https://img.shields.io/github/stars/israelguido/rust-na-pratica?style=flat&logo=github)](https://github.com/israelguido/rust-na-pratica/stargazers)
[![GitHub downloads](https://img.shields.io/github/downloads/israelguido/rust-na-pratica/total?style=flat&logo=github)](https://github.com/israelguido/rust-na-pratica/releases)
[![GitHub last commit](https://img.shields.io/github/last-commit/israelguido/rust-na-pratica?style=flat)](https://github.com/israelguido/rust-na-pratica/commits/main)

## Índice das aulas

| # | Aula | Pasta | O que você aprende |
|---|------|-------|--------------------|
| 1 | [Hello World](#1-hello-world) | [`hello-world/`](./hello-world) | Primeiro binário com Cargo e `println!` |
| 2 | [API simples](#2-api-simples) | [`api-simples/`](./api-simples) | Rotas HTTP básicas com Axum e JSON |
| 3 | [API com parâmetro na rota](#3-api-com-parmetro-na-rota) | [`api-simples-parametro/`](./api-simples-parametro) | Como receber valores dinâmicos em `GET /hello/{name}` |
| 4 | [API com POST e payload](#4-api-com-post-e-payload) | [`api-simples-parametro-post/`](./api-simples-parametro-post) | Receber JSON em `POST` e processar dados de um usuário |
| 5 | [Site estático com Axum](#5-site-estatico-com-axum) | [`site/`](./site) | Criar páginas HTML em rotas como `/` e `/sobre` |
| 6 | [Site com templates Askama](#6-site-com-templates-askama) | [`site-askama-template/`](./site-askama-template) | Separar HTML em templates com Askama e renderizar dados dinamicamente |

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

## 3. API com parâmetro na rota

Pasta: [`api-simples-parametro/`](./api-simples-parametro)

Exemplo de rota dinâmica com [Axum](https://docs.rs/axum): o valor do caminho entra na URL e é usado no retorno JSON.

```bash
cd api-simples-parametro
cargo run
```

Teste com:

```bash
curl http://127.0.0.1:3000/hello/Israel
```

Resposta esperada:

```json
{"message":"Hello, Israel!"}
```

## 4. API com POST e payload

Pasta: [`api-simples-parametro-post/`](./api-simples-parametro-post)

Nesta aula, a API recebe dados em JSON via `POST` e também mantém a rota dinâmica com parâmetro na URL.

```bash
cd api-simples-parametro-post
cargo run
```

Exemplo de requisição:

```bash
curl -X POST http://127.0.0.1:3000/usuario \
  -H "Content-Type: application/json" \
  -d '{"nome":"Israel","idade":30}'
```

## 5. Site estático com Axum

Pasta: [`site/`](./site)

Exemplo de site em Rust com [Axum](https://docs.rs/axum), onde o servidor responde com páginas HTML em duas rotas:

- `/` — página inicial com o título "Meu primeiro site com Rust"
- `/sobre` — página de apresentação com link para voltar ao início

```bash
cd site
cargo run
```

Depois, abra no navegador:

```text
http://127.0.0.1:3000
```

Você também pode acessar a página de sobre em:

```text
http://127.0.0.1:3000/sobre
```

## 6. Site com templates Askama

Pasta: [`site-askama-template/`](./site-askama-template)

Nesta aula, o HTML deixa de ser montado diretamente em strings e passa a ser renderizado por templates usando [Askama](https://askama.readthedocs.io/). A ideia é separar a lógica do Rust da marcação HTML.

```bash
cd site-askama-template
cargo run
```

Depois, acesse no navegador:

```text
http://127.0.0.1:3000
```

Os templates ficam dentro da pasta `templates/` e são usados para renderizar as páginas inicial e "sobre".

## Como usar este repositório

```bash
git clone https://github.com/israelguido/rust-na-pratica.git
cd rust-na-pratica
```

Cada aula é um crate independente. Entre na pasta da aula e rode `cargo run`.

## Canal

Vídeos no YouTube: [youtube.com/@israelguido](https://www.youtube.com/@israelguido)
