# Objekt 🗄️

*A pocket‑sized, self‑hostable JSON cache service built in Rust*

Objekt lets you store any JSON, retrieve it later, and track basic metadata, all through a clean and lightweight REST API. Ideal for prototypes, side projects, and CLI tools where simplicity matters.

## ✨ Features

* **Fast & tiny** - Rust powered.
* **Pluggable storage** - ships with in‑memory cache; swap in a filesystem or your own provider.
* **Token auth** - create a user, get a token, slap it in `Authorization` header.
* **Single‑binary deploy** - run locally or in Docker, no DB required.

## 🚀 Quick Start

```bash
# 1. Clone & build
git clone https://github.com/yehorovye/objekt.git
cd objekt
cargo run -p server               # or `docker compose up`

# 2. Required env
export SERVER_SECRET="super‑secret‑bytes"   # used to mint user tokens
# Optional
export PORT=8080                            # default 8080
```

The server now listens on **[http://localhost:8080](http://localhost:8080)**.

## 🔑 Authentication Flow

1. **Create a user**

   ```bash
   curl -X POST http://localhost:8080/auth/yehorovye \
        -H "Content-Type: application/json" \
        -d '{"password": "objekt-ftw"}'
   ```

   ```json
   {
     "ok": true,
     "message": "created user",
     "data": { "token": "ee44d9e0..." }
   }
   ```

2. **Use the token** - pass it verbatim (no `Bearer` prefix) in every mutating call:

   ```
   Authorization: ee44d9e0...
   ```

## 📡 API Reference

| Method | Path            | Protected | Purpose                                                |
| ------ | --------------- | ----- | ---------------------------------------------------------  |
| GET    | `/`             | ❌     | Health probe (“Ok!”)                                      |
| POST   | `/auth/{user}`  | ❌     | Create user → returns token                               |
| GET    | `/store/{key}/` | ❌     | List keys **starting with** `key` (or all with `/store/`) |
| GET    | `/store/{key}`  | ❌     | Fetch value                                               |
| GET    | `/store/{key}$` | ❌     | Fetch metadata                                            |
| PUT    | `/store/{key}`  | ✅     | **Create** entry (fails if exists)                        |
| PATCH  | `/store/{key}`  | ✅     | **Update** existing entry                                 |
| DELETE | `/store/{key}`  | ✅     | Delete entry                                              |
| DELETE | `/store/!`      | ✅     | Purge all your entries                                    |

> **Note**: keys are path‑like, `/` inside keys becomes `:` internally, so feel free to nest.

## 📝 Example Session

```bash
# Add an entry
curl -X PUT http://localhost:8080/store/projects/rust \
     -H "Authorization: $TOKEN" \
     -H "Content-Type: application/json" \
     -d '{"stars": 1337}'

# → 201
{ "ok": true, "message": "Created cache entry", "data": {} }

# Retrieve it
curl http://localhost:8080/store/projects/rust
# -> 200
{ "stars": 1337 }

# See metadata
curl http://localhost:8080/store/projects/rust$
# -> 200
{ "created_at": "2025-06-20T12:00:00Z", "version": 0, "issuer": "yehorovye" }

# List everything under "projects/"
curl http://localhost:8080/store/projects/
# -> ["projects:rust"]
```

## 🛠️ Build & Deploy

```bash
# Release binary
cargo build --release -p server
./target/release/server

# Or Docker
docker build -t objekt .
docker run -e SERVER_SECRET=shhh -p 8080:8080 objekt
```

## 📂 Project Layout

```
crates/          # Reusable libs: ciphers, macros_utils
server/          # Actix‑Web application
└── src/
    ├── routes/  # auth, store, root
    ├── providers/  # memory & filesystem back‑ends
    └── guards/     # auth + path sanitation
```

## ❤️ Contributing

PRs and issues are welcome! Clone, create a branch, and let the CI (GitHub Actions) do its magic.

## License

MIT © 2025 Elisiei Yehorovye
