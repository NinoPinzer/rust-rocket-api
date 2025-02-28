# 🚀 Rust Rocket API Template

A structured API template in **Rust** with **Rocket.rs**, utilizing a modular architecture.

## 📂 Project Structure

```
rust-rocket-api/
│── src/
│   ├── config/           # Configuration files (Logger, etc.)
│   │   ├── database.rs
│   │   ├── mod.rs
│   ├── controller/       # Controllers for API endpoints
│   │   ├── mod.rs
│   │   ├── message_controller.rs
│   ├── middleware/       # Middleware (e.g., Logging)
│   │   ├── mod.rs
│   ├── models/           # Data models
│   │   ├── mod.rs
│   │   ├── message.rs
│   ├── routes/           # API routes
│   │   ├── mod.rs
│   │   ├── message_route.rs
│   ├── services/         # Business logic
│   │   ├── mod.rs
│   │   ├── message_service.rs
│   ├── main.rs           # Application entry point
│── Cargo.toml            # Rust project file
│── .gitignore            # Files to be ignored
```

## 🚀 Installation & Usage

### 1️⃣ **Clone the Project**

```sh
git clone https://github.com/NinoPinzer/rust-rocket-api.git
cd rust-rocket-api
```

### 2️⃣ **Install Dependencies**

```sh
cargo build
```

### 3️⃣ **Start the Development Server**

```sh
cargo run
```

The API will now be running at `http://127.0.0.1:8000`.

## 📡 API Endpoints

| Method | Route         | Description       |
| ------ | ------------- | ----------------- |
| GET    | `/users`      | List all users    |
| POST   | `/users`      | Create a new user |
| Update | `/users/<id>` | Update a user     |
| Delete | `/users/<id>` | Delete a user     |


## 🛠 Configuration

Configuration can be adjusted using a `.env` file:

```
DATABASE_URL=postgres://user:password@localhost/db_name
```

## 📜 License

This project is licensed under the **MIT License**.

