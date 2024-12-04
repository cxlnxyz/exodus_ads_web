# Exodus - Active Directory Synchronization

## Overview

Exodus is a web application for synchronizing Active Directory. It provides a user-friendly interface for managing users, systems, and synchronization processes.

## Features

- User authentication with session management
- Dashboard displaying synchronization statistics
- User management
- System management
- Dark mode support

## Prerequisites

- Rust
- Cargo

## Setup

1. Clone the repository:
    ```sh
    git clone https://github.com/beklauter/exodus_ads_web.git
    cd exodus_ads_web/
    ```

2. Install dependencies:
    ```sh
    cargo build --realease
    ```

3. Run the server:
    ```sh
    cargo run --realease
    ```

## Project Structure

- `src/loader/web/webserver.rs`: Main server setup and route definitions.
- `src/loader/web/login.rs`: Handles login functionality.
- `src/loader/web/dashboard.rs`: Handles dashboard functionality.
- `src/loader/web/middleware.rs`: Middleware for authentication.
- `src/sites/index.html`: Main landing page.
- `src/sites/login.html`: Login page.
- `src/sites/dashboard.html`: Dashboard page.
- `src/sites/unauthorized.html`: Unauthorized access page.

## Routes

- `/`: Landing page
- `/login`: Login page
  - `GET`: Display login form
  - `POST`: Handle login form submission
- `/dashboard`: Dashboard page (requires authentication)
- `/logout`: Logout and clear session

## Authentication

The application uses session-based authentication. The `AuthMiddleware` in `middleware.rs` ensures that only authenticated users can access protected routes.

## Dark Mode

The application supports dark mode, which can be toggled using the button in the header of each page. The preference is saved in `localStorage`.

## License

This project is licensed under the GNU License.