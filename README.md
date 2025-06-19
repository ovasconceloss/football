# FB2025 – Football 2025

**FB2O25** is a lightweight, offline-first football management simulator built with performance and robust data management in mind. Inspired by classics like *Football Manager*, it distills the core gameplay experience—tactical decisions, team management, and detailed match simulations—into a streamlined desktop application.

> ⚽ Tactical depth without the complexity.

> 💻 100% offline with robust save/load. Cross-platform.

> 🛠️ Engineered for performance, clarity, and control.

-----

## 🚀 Overview

**SO2025** aims to offer an alternative for players who love football simulation but prefer a focused, high-performance experience. Designed for a **fast, native desktop environment**, with zero network dependencies for core gameplay, FB2025 focuses on **core mechanics** like:

  - Squad selection and player development
  - Simplified tactics (formations and mentality)
  - High-performance match simulations
  - Robust finance and transfer system
  - Season progression and club objectives
  - Complete save/load system for multiple careers

-----

## 🧱 Tech Stack

| Layer             | Tech                                            | Purpose                                                                                                                                                                                                                                                                                           |
|-------------------|-------------------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| **Backend Logic** | [Node.js](https://nodejs.org/en) & [TypeScript](https://www.typescriptlang.org/) | Core application logic, data handling, API layer, and managing save states. Provides a robust and type-safe foundation.                                                                                                                                                            |
| **Web Framework** | [Fastify](https://www.fastify.io/)              | High-performance web framework for the Node.js backend. Ensures rapid API responses and efficient resource utilization.                                                                                                                                                           |
| **API Layer** | [GraphQL](https://graphql.org/)                 | Flexible and efficient query language for your API. Enables the frontend to request exactly the data it needs, minimizing over-fetching and under-fetching, crucial for complex inter-connected game data (players, clubs, matches).                                                      |
| **Database** | [SQLite](https://sqlite.org/)                   | Lightweight, serverless, and robust relational database. Used for core game data within each save file (`.db` files per career) and a separate metadata database for save management.                                                                                             |
| **ORM** | [Prisma](https://www.prisma.io/)                | Modern, type-safe ORM for Node.js and TypeScript. Simplifies database interactions, provides robust migrations, and enhances data integrity with SQLite.                                                                                                                           |
| **Frontend UI** | [React](https://react.dev/)                     | Declarative and efficient JavaScript library for building the interactive and rich user interface of the game.                                                                                                                                                                        |
| **Desktop Shell** | [Tauri](https://tauri.app/)                     | Lightweight, secure, and performant framework for building cross-platform desktop applications (Windows, macOS, Linux) with web technologies. Enables seamless communication between the web frontend and Rust backend logic.                                                      |

-----

## 📦 Current Version

**v0.1 – In Development**

### Features:

  - [ ] Robust Save/Load System: Create and manage multiple independent career save files.
  - [ ] Basic team management
  - [ ] Predefined tactics & mentalities
  - [ ] High-performance match simulation engine (results, key stats)
  - [ ] Simple transfer and scouting system
  - [ ] Budget, contracts, and wage tracking
  - [ ] Season progression and objectives (WIP)
  - [ ] League standings and history (WIP)
  - [ ] Awards and news feed (Planned)

> ✅ MVP scope is intentionally narrow to prioritize stability, usability, and offline robustness.

-----

## 🎯 Project Goals

| Goal                                | Description                                                                                                                                                                                                                                                                                               |
|-------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| ⚡ **Exceptional Performance** | Leveraging Node.js (Fastify, GraphQL) for efficient data handling and **Rust (via Tauri commands)** for CPU-intensive game simulations, ensuring a responsive and smooth user experience without UI freezes.                                                                                                |
| 🧩 **Simplicity & Accessibility** | Accessible gameplay for users who enjoy football simulation but prefer a streamlined experience over excessive micromanagement.                                                                                                                                                                             |
| 💾 **Offline-first & Data Control** | Zero online requirements for core gameplay. Users have full control over their game data, with robust local save files and management.                                                                                                                                                                      |
| 💼 **Desktop-Native Experience** | Delivers a fast, lightweight, and smooth cross-platform desktop experience using Tauri's native WebView, ensuring low resource consumption and a familiar feel on Windows, macOS, and Linux.                                                                                                                     |
| 🛡️ **Type Safety & Maintainability** | Extensive use of **TypeScript** across the backend and frontend, combined with **Prisma's** type-safety for database interactions, ensures robust, bug-resistant, and easily maintainable code for a complex game.                                                                                             |

-----

## 🛠 Development

> ⚙ **Clone & Build**

```bash
# Clone the repository
git clone https://github.com/ovasconceloss/football.git
cd football

# Install backend dependencies
cd backend
pnpm install # or yarn/npm install
cd ..

# Install frontend dependencies
cd frontend
pnpm install # or yarn/npm install
```

### 🧪 Requirements

  - **Rust (1.70+):** For the Tauri core and performance-critical game logic.
  - **Node.js (18+):** For the backend application and frontend tooling.
  - **npm / Yarn:** Package manager.
  - **SQLite (bundled):** Database engine, no separate installation required.
  - **Tauri CLI:** For building and managing the desktop application.
  - **TypeScript:** Language for most of the codebase.

## 👤 Author

[@ovasconceloss](https://github.com/ovasconceloss) – Project Lead, Developer & Designer

-----

## 📄 License

This project is licensed under the GNU General Public License v3.0 – see the [LICENSE](https://www.google.com/search?q=./LICENSE) file for details.

## 🤝 Contributing

Interested in contributing, translating, or suggesting improvements?

  - Open an issue or feature request
  - Fork the repo and submit a pull request
  - Join the discussion\!

-----

> ⚽ *Tactical depth without the complexity. Experience football management, optimized.*

-----