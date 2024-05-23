<br />
<div align="center" id="readme-top">
<h3 align="center">NeoSocket</h3>

  <p align="center">
    A fast websocket room api
    <br />
    <a href="https://neosocket.angelillo15.es">View Demo</a>
    ·
    <a href="https://github.com/Angelillo15/neosocket/issues/new?labels=bug&template=bug-report.md">Report Bug</a>
    ·
    <a href="https://github.com/Angelillo15/neosocket/issues/new?labels=enhancement&template=feature-request.md">Request Feature</a>
  </p>
</div>

## About The Project
<p align="center">
  <img src="/.github/neosocket.svg" width="200px" alt="NeoSocket logo"/>
</p>

This project wants to be something similar to [Lucko's ByteSocks](https://github.com/lucko/bytesocks) but written in 
Rust, making it faster and more efficient.

In summary this project allows clients to create like `rooms` where they can send messages to all the clients in the room, making it a good option for people who want to recreate a viewer like spark or luckperms one 
### Built With

* [Rust](https://www.rust-lang.org/)
* [Actix-rs](https://actix.rs/)
* [Serde](https://serde.rs/)
* [Tokio](https://tokio.rs/)

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- GETTING STARTED -->

## Getting Started

This is an example of how you may give instructions on setting up your project locally.
To get a local copy up and running follow these simple example steps.

### Prerequisites

This is an example of how to list things you need to use the software and how to install them.

* rust

### Building

_Below is an example of how you can instruct your audience on installing and setting up your app. This template doesn't
rely on any external dependencies or services._
[![NeoSocket running](https://i.imgur.com/TwxTl6j.gif)](https://example.com)

1. Clone the repo
   ```sh
   git clone https://github.com/Angelillo15/neosocket.git
   ```
2. Build the project
   ```sh
   # to do a debug build
   cargo build
   # to do a release build (recommended)
   cargo build --release
   ```
3. Run the project
   ```sh
   # to run the project in debug mode
   ./target/debug/neosocket
   # to run the project in release mode (recommended)
   ./target/release/neosocket
   ```

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Usage

Just install the project and run it. It will start a server on `0.0.0.0:2022` by default. You can change the port by
setting the `PORT` environment variable and the host by setting the `HOST` environment variable.
<!-- ROADMAP -->

<p align="right">(<a href="#readme-top">back to top</a>)</p>
<!-- LICENSE -->

## License

Distributed under the MIT License. See `LICENSE.txt` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Contact
* [Discord](https://discord.nookure.com)
* [Email](mailto:contact@angelillo15.es)

<p align="right">(<a href="#readme-top">back to top</a>)</p>