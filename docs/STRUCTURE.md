# Project Structure

```
rujak/
├── .github/
│   └── workflows
│       └── ci.yml (GitHub Actions CI/CD workflow)
├── api/
│   └── openapi.yaml (API specification)
│ 
├── backend/
│   ├── src/
│   │   ├── controllers/
│   │   ├── submission.rs
│   │   ├── review.rs
│   │   └── user.rs
│   ├── models/
│   │   ├── submission.rs
│   │   ├── reviewer.rs
│   │   └── user.rs
│   ├── routes/
│   │   ├── submission_routes.rs
│   │   ├── review.rs
│   │   └── auth.rs
│   ├── services/
│   │   ├── auth_service.rs
│   │   ├── submission_service.rs
│   │   └── review_service.rs
│   ├── utils/
│   │   └── helpers.rs
│   ├── Cargo.toml
│   └── main.rs
│
├── frontend/
│   ├── src/
│   │   ├── components/
│   │   ├── pages/
│   │   ├── api/
│   │   └── Cargo.toml
│
├── docs/
│   ├── overview.md
│   ├── setup.md
│   ├── contributing.md
│   └── LICENSE.md
│
├── scripts/
│   ├── setup.sh
│   └── docker-compose.yml
│
├── tests/
│   ├── unit/
│   └── integration/
│
├── .gitignore
└── README.md
```