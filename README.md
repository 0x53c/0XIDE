# Oxide: Personal Cloud Project

## Overview

"Oxide" is a personal cloud project focused on learning and experimenting with modern software architecture patterns, API design, and distributed systems. The primary goals include:

-   Developing a scalable API-first system.
-   Gaining practical experience with Kubernetes, Docker, Rust, and other cloud-native technologies.
-   Implementing secure secrets management using Doppler.
-   Exploring microservices architecture and deployment strategies.
-   Creating a platform where different services can be deployed and scaled as needed.

This is primarily a learning project, so flexibility and experimentation are encouraged!

## Technologies Used

-   **Containerization:** Docker
-   **Orchestration:** Kubernetes
-   **Programming Language:** Rust (with potential exploration of Zig for specific components)
-   **Secrets Management:** Doppler
-   **API Design:** OpenAPI/Swagger
-   **Data Storage:** NAS with potential usage of relational databases
-   **API Gateway:** Nginx or Traefik

## Architecture

The system will be built as a distributed system, leveraging containerization with Docker and orchestration with Kubernetes. APIs will be the main focus with well-defined endpoints. NAS will act as network storage. Doppler will manage all secrets.

Key components include:

-   **API Gateway:** Central entry point for all API requests, handling routing, authentication, and authorization.
-   **Microservices:** Individual services that handle specific business functionalities.
-   **Database/Data Store:** Persistent data storage.
-   **NAS Storage:** Network Attached Storage for file storage.
-   **Doppler:** Secure secrets management system.

## Getting Started

1.  **Prerequisites:**
    -   Install Docker on your machine.
    -   Install `kubectl` for Kubernetes interaction.
    -   Install the Doppler CLI.
    -   Install the Rust Toolchain (`rustup` and `cargo`).

2.  **Project Setup:**
    -   Clone the repository: `git clone <repository-url>`
    -   Navigate to the project directory: `cd oxide`
    -   Initialize the project
        -   `cargo new <project>`
        -   Run `cargo build`

3. **Running the service locally**
     -   Build docker image: `docker build . -t <service-name>`
     -   Run the container: `docker run <service-name>`

4.  **Configure Doppler**
    -   Create doppler project and development environment
    -   Set up all required secrets for the project
    -   Install doppler CLI and login

## Development Workflow

-   **API Design:** Define the API using OpenAPI/Swagger.
-   **Service Development:** Write the service using Rust.
-   **Dockerization:** Containerize the service using Docker.
-   **Deployment:** Deploy using Kubernetes manifests.

## Task Management

-   This project uses GitHub Projects for task management.
-   Each task is tracked using GitHub Issues.

## Contributing

This is a personal project, so contributions are currently not expected. However, feedback and suggestions are always welcome!

## License

[Optional: Add a license if you wish]

