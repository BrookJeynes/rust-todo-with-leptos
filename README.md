## Building a website in Leptos from scratch - Part 1

This repo holds all the code for my Leptos tutorial series which can be found here: 

![Website preview](https://user-images.githubusercontent.com/25432120/232404896-cad3efd9-735a-4484-a6b7-50c16037ffec.png)

### Running locally
1. Clone the repo and navigate into it
    ```bash
    git clone https://github.com/BrookJeynes/todo-leptos
    cd todo-leptos
    ```
2. Update Tailwind styles
    ```bash
    npx tailwindcss -i ./input.css -o ./style/output.css --watch
    ```
3. Install `cargo-leptos`
    ```bash
    cargo install cargo-leptos
    ```
4. Set rustup to use nightly
    ```bash
    rustup toolchain install nightly
    rustup default nightly
    rustup target add wasm32-unknown-unknown
    ```
5. Run the application
    ```bash
    cargo leptos watch
    ```
6. Open http://localhost:3000/
