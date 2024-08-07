# ActixExample

This project is a simple HTTP server implementation using the Actix web framework. The project provides a `greet` service that responds to different endpoints with data received through path parameters, query parameters, and JSON payloads. The project is structured in a modular way, with the `greeter_service` module being used from another part of the project.

## Endpoints

- **GET /{name}/{surname}**
  - This endpoint receives `name` and `surname` as path parameters and responds with a greeting message.

  ```rust
  #[get("/{name}/{surname}")]
  pub async fn greet_path(path: web::Path<GreetPathParams>) -> impl Responder {
      let name = &path.name;
      let surname = &path.surname;
      greeter(&format!("{} {}", name, surname))
  }
  ```

- **GET /**
  - This endpoint receives `name` and `surname` as as query parameters and responds with a greeting message.

  ```rust
    #[get("/")]
    pub async fn greet_query(params: web::Query<GreetPathParams>) -> impl Responder {
        let name = &params.name;
        let surname = &params.surname;
        greeter(&format!("{} {}", name, surname))
    }
  ```

- **POST /**
  - This endpoint receives `name` and `surname` as JSON payload and responds with a greeting message.

  ```rust
    #[post("/")]
    pub async fn greet_json(params: web::Json<GreetPathParams>) -> impl Responder {
        let name = &params.name;
        let surname = &params.surname;
        greeter(&format!("{} {}", name, surname))
    }
  ```

## Structure

- The project is designed with modularity in mind. The greeter_service module is imported from another part of the project, demonstrating how different parts of a project can be organized and reused.

## Starter Project

- This project serves as a starter template for building more complex web applications using the Actix web framework. It provides basic examples of handling different types of requests and structuring a project in a modular way.
