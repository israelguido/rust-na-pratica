# API CRUD com Rust + Axum + SQLx + PostgreSQL + JWT

Esta documentação reúne os comandos `curl` utilizados para testar a API desenvolvida com:

* Rust
* Axum
* SQLx
* PostgreSQL
* JWT
* Serde
* Tokio

## URL base

Durante o desenvolvimento local, a API está disponível em:

```text
http://127.0.0.1:3000
```

---

# 1. Testar a API

Endpoint:

```text
GET /
```

Comando:

```bash
curl http://127.0.0.1:3000/
```

Resposta esperada:

```text
API Rust + SQLx
```

---

# 2. Login

Endpoint:

```text
POST /login
```

O login recebe:

```json
{
    "email": "admin@email.com",
    "senha": "123456"
}
```

Comando:

```bash
curl -X POST http://127.0.0.1:3000/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "admin@email.com",
    "senha": "123456"
  }'
```

Resposta esperada:

```json
{
    "token": "eyJ..."
}
```

O token retornado deverá ser utilizado para acessar as rotas protegidas.

---

# 3. Salvar o JWT em uma variável

Para facilitar os próximos testes, copie o token retornado pelo `/login` e salve em uma variável:

```bash
TOKEN="COLE_SEU_TOKEN_AQUI"
```

Exemplo:

```bash
TOKEN="eyJhbGciOiJIUzI1NiJ9..."
```

Agora podemos utilizar:

```bash
-H "Authorization: Bearer $TOKEN"
```

em todas as requisições protegidas.

---

# 4. Testar acesso sem JWT

Endpoint:

```text
GET /usuarios
```

Faça a requisição sem enviar o token:

```bash
curl -i http://127.0.0.1:3000/usuarios
```

Resposta esperada:

```text
HTTP/1.1 401 Unauthorized
```

Isso confirma que o middleware está protegendo a rota.

---

# 5. Testar JWT inválido

Também podemos verificar se a API rejeita um token inválido:

```bash
curl -i http://127.0.0.1:3000/usuarios \
  -H "Authorization: Bearer token_invalido"
```

Resposta esperada:

```text
HTTP/1.1 401 Unauthorized
```

---

# 6. Listar usuários

Endpoint:

```text
GET /usuarios
```

Sem utilizar variável:

```bash
curl -X GET http://127.0.0.1:3000/usuarios \
  -H "Authorization: Bearer SEU_TOKEN_AQUI"
```

Utilizando a variável `$TOKEN`:

```bash
curl -X GET http://127.0.0.1:3000/usuarios \
  -H "Authorization: Bearer $TOKEN"
```

Também podemos utilizar `-i` para visualizar o status HTTP:

```bash
curl -i http://127.0.0.1:3000/usuarios \
  -H "Authorization: Bearer $TOKEN"
```

Resposta esperada:

```json
[
    {
        "id": 1,
        "nome": "Israel",
        "email": "israel@teste.com"
    }
]
```

---

# 7. Criar usuário

Endpoint:

```text
POST /usuarios
```

Sem variável:

```bash
curl -X POST http://127.0.0.1:3000/usuarios \
  -H "Authorization: Bearer SEU_TOKEN_AQUI" \
  -H "Content-Type: application/json" \
  -d '{
    "nome": "Israel",
    "email": "israel@teste.com"
  }'
```

Utilizando `$TOKEN`:

```bash
curl -X POST http://127.0.0.1:3000/usuarios \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "nome": "Israel",
    "email": "israel@teste.com"
  }'
```

Resposta esperada:

```json
{
    "mensagem": "Usuário criado com sucesso!"
}
```

---

# 8. Atualizar usuário

Endpoint:

```text
PUT /usuarios/{id}
```

Exemplo atualizando o usuário de ID `1`:

```bash
curl -X PUT http://127.0.0.1:3000/usuarios/1 \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "nome": "Israel Guido",
    "email": "israel@teste.com"
  }'
```

Resposta esperada:

```json
{
    "mensagem": "Usuario atualizado com sucesso!"
}
```

Para atualizar outro usuário, basta alterar o ID:

```text
/usuarios/1
/usuarios/2
/usuarios/3
```

---

# 9. Deletar usuário

Endpoint:

```text
DELETE /usuarios/{id}
```

Exemplo removendo o usuário de ID `1`:

```bash
curl -X DELETE http://127.0.0.1:3000/usuarios/1 \
  -H "Authorization: Bearer $TOKEN"
```

Resposta esperada:

```json
{
    "mensagem": "Usuário deletado com sucesso!"
}
```

---

# 10. Fluxo completo de teste

O fluxo recomendado para testar a API é:

```text
1. Iniciar a API
       ↓
2. POST /login
       ↓
3. Receber JWT
       ↓
4. Salvar JWT em $TOKEN
       ↓
5. GET /usuarios
       ↓
6. POST /usuarios
       ↓
7. PUT /usuarios/{id}
       ↓
8. DELETE /usuarios/{id}
```

---

# 11. Resumo dos endpoints

| Método | Endpoint         | JWT | Descrição                       |
| ------ | ---------------- | --- | ------------------------------- |
| GET    | `/`              | Não | Testa se a API está funcionando |
| POST   | `/login`         | Não | Realiza login e gera JWT        |
| GET    | `/usuarios`      | Sim | Lista os usuários               |
| POST   | `/usuarios`      | Sim | Cadastra um usuário             |
| PUT    | `/usuarios/{id}` | Sim | Atualiza um usuário             |
| DELETE | `/usuarios/{id}` | Sim | Remove um usuário               |

---

# 12. Headers utilizados

## JSON

Para requisições que enviam JSON:

```bash
-H "Content-Type: application/json"
```

## JWT

Para acessar endpoints protegidos:

```bash
-H "Authorization: Bearer $TOKEN"
```

O formato do header é:

```text
Authorization: Bearer TOKEN
```

---

# 13. Teste rápido completo

## Login

```bash
curl -X POST http://127.0.0.1:3000/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "admin@email.com",
    "senha": "123456"
  }'
```

## Salvar token

```bash
TOKEN="COLE_SEU_TOKEN_AQUI"
```

## Listar

```bash
curl http://127.0.0.1:3000/usuarios \
  -H "Authorization: Bearer $TOKEN"
```

## Criar

```bash
curl -X POST http://127.0.0.1:3000/usuarios \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "nome": "Israel",
    "email": "israel@teste.com"
  }'
```

## Atualizar

```bash
curl -X PUT http://127.0.0.1:3000/usuarios/1 \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "nome": "Israel Guido",
    "email": "israel@teste.com"
  }'
```

## Deletar

```bash
curl -X DELETE http://127.0.0.1:3000/usuarios/1 \
  -H "Authorization: Bearer $TOKEN"
```

## Verificar novamente

```bash
curl http://127.0.0.1:3000/usuarios \
  -H "Authorization: Bearer $TOKEN"
```

---

# Stack utilizada

```text
Rust
 │
 ├── Axum
 │    ├── Router
 │    ├── Json
 │    ├── State
 │    ├── Path
 │    └── Middleware
 │
 ├── SQLx
 │    └── PostgreSQL
 │
 ├── Serde
 │    ├── Serialize
 │    └── Deserialize
 │
 ├── jsonwebtoken
 │    ├── encode
 │    └── decode
 │
 ├── dotenvy
 │    └── .env
 │
 └── Tokio
      └── Runtime assíncrono
```

## Fluxo de autenticação

```text
Cliente
   │
   │ POST /login
   ▼
API Rust
   │
   │ valida login
   ▼
Gera JWT
   │
   ▼
Cliente recebe token
   │
   │ Authorization: Bearer TOKEN
   ▼
Middleware JWT
   │
   ├── Token inválido ──────→ 401 Unauthorized
   │
   └── Token válido
              │
              ▼
          CRUD usuários
              │
              ▼
          PostgreSQL
```
