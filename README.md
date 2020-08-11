# Textcamp

It's an open source multiplayer text based adventure game!

## Demo

You can find a playable demo at <https://play.text.camp/>. The game play is very limited, but demonstrates the basic functionality in a public environment.

## Building

![build](https://github.com/textcamp/textcamp/workflows/Rust/badge.svg)

Textcamp requires the latest stable Rust. To install Rust on your system, head over to <https://rustup.rs/> and follow the instructions. Run `cargo build` in the git repository and wait a bit. Easy!

Alternatively, if you have Docker running, you can build it there with no system dependencies. `docker build -t textcamp .` in the git repository will do the trick.

## Running

_Textcamp currently requires AWS DynamoDB and SES to be configured in order to run. If you'd like to help make Textcamp run as a standalone system, or with alternative databases and e-mail providers, please get in touch!_

Copy the `example.env` file to `.env` and adjust the parameters to your taste.

With a local build: `cargo run`

With a Docker build: `docker run -dp 8080:8080 textcamp`

In either case, you can access the running server by pointing your web browser at <https://localhost:8080/>.

## Configuration

Ports, logging levels, and other parameters are configurable via environment variables. Please see the `.env` file and the `Dockerfile` for defaults for different environments.

## Contributing

_We adhere to the [Contributor Covenant](https://www.contributor-covenant.org/version/2/0/code_of_conduct/) in order to maintain a respectful and productive community._

Interested in chipping in? We're looking for folks interested in contributing to the stories and adventures in Textcamp, improving the documentation, and building out the platform itself.

Head over to our [GitHub Projects](https://github.com/textcamp/textcamp/projects) and take a look to see how you can help and say hello!

## Contact

* Twitter - <https://twitter.com/textdotcamp>
* E-Mail - <play@text.camp>

## Legal

Copyright (c) 2018-2020 Peat Bakke (<peat@text.camp>).

This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
