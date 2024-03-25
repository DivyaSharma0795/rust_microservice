# Individual Project 02 - Continuous Delivery of Rust Microservice

Requirements:

-   Simple REST API/web service in Rust
-   Dockerfile to containerize service
-   CI/CD pipeline files

This is a simple REST API service built with Rust and Warp. It takes a name as input and returns the reversed name.

### Features
-   REST API endpoint at /hello/{name} that takes a name and returns it reversed.
-   Dockerized for easy deployment.
-   CI/CD pipeline using GitLab CI.

### Prerequisites
-   Rust (latest stable version)
-   Docker
-   GitLab account (for CI/CD)

### Usage
1.  Clone the repository.

```
git clone <repository-url>
cd <repository-directory>
```

2.  Build the project.

```
cargo build
```

3.  Run the project.

```
cargo run
```

4.  Build the Docker image.

```
docker build -t rust-microservice .
```

5.  Run the Docker container.

```
docker run -p 3030:3030 rust-microservice
```

The service will be available at `http://localhost:3030/`

### CI/CD
The CI/CD pipeline is configured using GitLab CI. The pipeline consists of the following stages:
-   Build: Builds the project.
-   Test: Runs the tests.

The pipeline is triggered on every push to the repository. This project includes a .gitlab-ci.yml file for continuous integration and deployment with GitLab CI. To use it, you need to set the DOCKERHUB_USERNAME and DOCKERHUB_PASSWORD variables in your GitLab project's CI/CD settings.

Send a GET request to the `/hello/{name}` endpoint, replacing `{name}` with the name you want to reverse. For example:

```
curl http://localhost:3030/hello/Divya
```

The service will respond with the reversed name.

```
"Hello, Divya! Your name in reverse is ayviD."
```

### License
This project is licensed under the MIT License - see the LICENSE file for details.