# 🧱 objekt

A **simple**, **memory-efficient**, and **token-authenticated** key-value cache server, built in pure Rust.

Supports ShareX-style uploads, local file-based storage (with custom storage support), and RESTful API interaction.

---
[Deepwiki](https://deepwiki.com/simxnet/objekt)

---

## 🚀 Features

- 🔐 **Token-based Authentication** with per-user hash tokens
- 🧠 **In-memory user store** + file-based storage backend (easily swappable)
- ⚡ **Fast and efficient**: runs on Actix Web + Tokio
- 🔍 **Structured metadata** per key (`issuer`, `created_at`, `version`)
- 📦 **REST API** for:
  - Creating users and tokens
  - Inserting (`PUT`), deleting (`DELETE`), updating (`PUT!`) entries
  - Fetching values and metadata
  - Listing keys by prefix

---

## 📦 API Overview

### 🔐 Create User & Token

```http
POST /auth/{username}
Content-Type: application/json

{
  "password": "your-password"
}
````

→ Returns a `token` to use in `Authorization` header.

---

### 📤 Add Entry (fails if exists)

```http
PUT /store/{key}
Authorization: <your-token>
Content-Type: application/json

{ "some": "data" }
```

---

### 🔁 Upsert Entry

```http
PUT /store/{key}!
Authorization: <your-token>
Content-Type: application/json

{ "updated": true }
```

---

### ❌ Remove Entry

```http
DELETE /store/{key}
Authorization: <your-token>
```

---

### 📥 Get Entry

```http
GET /store/{key}
```

---

### 📋 List Keys

```http
GET /store/{prefix}/
```

---

### 🧾 Metadata

```http
GET /store/{key}$
```

---

## 🛠 Setup

### 📦 Requirements

* Rust nightly (see `rust-toolchain.toml`)
* Cargo
* Optional: Docker + ShareX support

### 🧪 Run Locally

```sh
# clone and build
git clone https://github.com/your-org/objekt.git
cd objekt

# set server secret
echo 'SERVER_SECRET=supersecret' > .env

# run
cargo run -p server
```

Server runs at `http://localhost:8080`

---

## 🐳 Docker

```sh
docker build -t objekt .
docker run -p 8080:8080 -e SERVER_SECRET=supersecret objekt
```

---

## 📁 Storage

By default, entries are stored in:

```
./store/
├── some:key              # value file (json)
├── some:key.meta         # metadata file
```

---

## ✨ Design Notes

* Keys are sanitized: `/` becomes `:`
* Routes ending with `!` trigger upsert
* Routes ending with `$` access metadata
* User data is stored in memory only — tokens persist via client reuse

---

## 📜 License

MIT

---

## ❤️ Made with Rust, Actix, and good taste.
> thanks to all my friends that were watching me while I was making this lol
