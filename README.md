<h1 align="center">QnA Web Application</h1>

<div align="center">
  <img src="https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white"/>
</div>

This project is a Q&A web application built in Rust, designed as a hands-on approach to learning the language and its ecosystem. Inspired by book [Rust Web Development](https://www.amazon.com/Rust-Web-Development-Bastian-Gruber/dp/1617299006) by Bastian Gruber, the app is built using the Warp web framework and Tokio for asynchronous runtime. It leverages Rust's strong type system, safety, and concurrency features while integrating SQLx for database interactions and PostgreSQL for persistent storage. The application incorporates JWT authentication, user management, and error handling with custom middlewares. Additionally, it makes use of Serde for data serialization, Reqwest for HTTP requests, and Rust-Argon2 for password hashing. Logging is implemented using Log and Tracing for efficient debugging and monitoring. Future improvements might include adding features such as real-time notifications, advanced search, or advanced user permissions.
