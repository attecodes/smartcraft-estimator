# SmartCraft Estimator

A full-stack web application for carpenters and builders to create **accurate project estimates** based on materials, linear/square footage, and labor costs.

This project is both a **real-world tool** and a **portfolio showcase**, demonstrating modern full-stack development with Rust, TypeScript, Docker, PostgreSQL, testing, and CI/CD.

## ✨ Features (Planned)

- Project-based cost estimation
- Linear & square footage calculations
- Material library with configurable pricing
- Labor cost & margin calculation
- Professional, shareable estimates (PDF)
- User authentication & saved projects

## 🧱 Tech Stack

### Frontend

- TypeScript
- React (Vite)
- Tailwind CSS
- Zod (validation)
- Playwright / Cypress (E2E testing)

### Backend

- Rust
- Actix Web
- SQLx
- JWT authentication

### Database

- PostgreSQL
- SQL migrations & seed data

### Infrastructure

- Docker & Docker Compose
- GitHub Actions (CI/CD)

## 📐 Why This Project?

As a professional carpenter, estimating jobs accurately is critical — yet many tools are outdated, expensive, or inflexible.

**SmartCraft Estimator** is designed to:

- Reflect real-world construction workflows
- Provide transparent pricing breakdowns
- Be fast, reliable, and type-safe

From a technical perspective, this project showcases:

- Domain-driven backend logic in Rust
- A modern, “vibe-coated” TypeScript UI
- Production-quality testing and CI/CD

## 🗂 Planned Architecture

```
smartcraft-estimator/
├── frontend/ # React + TypeScript UI
├── backend/ # Actix Web API
├── docker-compose.yml
├── .github/
│ └── workflows/
│ └── ci.yml
└── README.md
```

## 🧪 Testing Strategy

- **Backend**
  - Unit tests for estimation logic
  - Integration tests with PostgreSQL
- **Frontend**
  - Component tests
  - End-to-end user flows

## 🚧 Project Status

**Early development**

Current focus:

- Repository setup
- Architecture & API design
- Estimation domain modeling

## 🪚 Inspiration

This project is inspired by Nordic and German craftsmanship principles:

- Precision
- Simplicity
- Durability

## 📄 License

MIT
