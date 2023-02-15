# Pixlstack

This project serves as the basis for my personal blog/portfolio website, created with Rust and Actix Web as the backend, and SvelteKit as the frontend.

Pixlstack is a mono repo containing all the folders necessary for container setups for the backend, frontend, and nginx for processing requests. 

## Technologies Used

- Rust
    - Actix Web
- SvelteKit
    - TailWindCSS
    - Flowbite
- Nginx

## Installation

1. Clone the repository
2. Install Docker and Docker Compose
3. Run `docker-compose up -d` to start the containers

## Usage

Once the containers are up and running, you can access the website by navigating to `http://localhost` in your web browser.

## Notes

Using this on localhost means you are unable to use contact form as it uses turnstile from cloudflare and it needs to be using the actual domain used in the turnstile setup.
