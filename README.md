# 🦀 Rujak 

## Rust-based Open Journal Management Framework

**Rujak** is a modern, modular, and scalable **Open Journal Management Framework** built with **Rust**. It is inspired by [Open Journal Systems (OJS)](https://pkp.sfu.ca/ojs/) but developed as a **from-scratch, modern alternative** with a Rust-first approach for performance, security, and scalability.

---

## 🌟 Features

- **Fast & Secure** – Built with Rust for high performance and safety.
- **Modular Architecture** – Independent modules for flexibility and maintainability.
- **RESTful API & GraphQL Support** – Future-proof API for integrations.
- **Role-based Access Control** – Secure user authentication & authorization.
- **Manuscript Submission Workflow** – End-to-end scholarly publishing management.
- **Pluggable Storage Backend** – Supports PostgreSQL, SQLite, or other databases.

---

## 📂 Project Structure

Rujak is structured as a **Rust Cargo workspace**, following a modular approach:

```
rujak/ 
├── backend/ # Core backend services (Rust, Axum, SQLx) 
├── frontend/ # Future frontend (optional, Yew/Leptos or JS-based) 
├── api/ # API specifications (REST/GraphQL) 
├── docs/ # Documentation, setup guides 
├── scripts/ # DevOps scripts (setup, Docker) 
├── tests/ # Unit and integration tests 
├── .github/ # CI/CD (GitHub Actions) 
├── Cargo.toml # Rust workspace configuration 
└── README.md # Project Overview
```

See [STRUCTURE](/docs/STRUCTURE.md) documentation for more details.

## 🚀 Quick Start

### 1️⃣ **Clone the Repository**

```bash
git clone https://github.com/yourusername/rujak.git
cd rujak
```

### Run Backend Locally

```
cd backend
cargo run
```

### Run Backend with Docker

```
docker-compose up --build
```

## 📌 Modules
| Module	| Description |
| --- | --- |
| User Management |	User authentication, roles, permissions |
| Manuscript Submission |	Authors submit and manage their papers |
| Peer Review	| Reviewers are assigned and provide feedback |
| Publication	| Final articles are published for open access |
| Analytics| 	Metrics and reports for journal editors |

## 🔧 Configuration
1. Copy the `.env.example` file and rename it to `.env`
2. Update your database settings in `.env`

    ```ini
    DATABASE_URL=postgres://username:password@localhost/rujak_db
    ```

## 🏗️ Tech Stack
Rujak is powered by modern Rust technologies:

- Backend: Axum, Tokio
- Database: SQLx (PostgreSQL)
- Auth: JWT-based authentication (OAuth2 coming soon)
- Frontend (future): Yew, Leptos, or Next.js
- Containerization: Docker, Kubernetes (future)
- CI/CD: GitHub Actions

## 🤝 Contributing
Want to help? Contributions are welcome!
Please check out our Contributing Guide and open an issue if you have ideas or feedback.

## 🛡️ License
MIT License
Rujak is free and open-source. Inspired by Open Journal Systems (OJS) but independently built from scratch.

> 📢 _This project is inspired by [OJS](https://pkp.sfu.ca/ojs/), but it is a completely new Rust-based implementation without any direct code reuse._

### 📬 Contact
- GitHub: [SHA888]()
- Twitter/X: [@ks_sha888]()
