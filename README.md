# Textcamp

It's a multiplayer text based adventure game!

Textcamp is barely "playable" at the moment. If you're interested in announcements down the road, please send an e-mail to <peat@text.camp> or follow [@textdotcamp](https://twitter.com/textdotcamp) on Twitter.

## Demo

You can find a playable demo at <https://play.text.camp/>. The game play is very limited, but demonstrates the basic functionality in a public environment.

## Building

![build](https://github.com/textcamp/textcamp/workflows/Rust/badge.svg)

Textcamp requires the latest stable Rust. To install Rust on your system, head over to <https://rustup.rs/> and follow the instructions. Run `cargo build` in the git repository and wait a bit. Easy!

Alternatively, if you have Docker running, you can build it there with no system dependencies. `docker build -t textcamp .` in the git repository will do the trick.

## Running

Copy the `example.env` file to `.env` and adjust the parameters to your taste.

With a local build: `cargo run`

With a Docker build: `docker run -dp 8080:8080 textcamp`

In either case, you can access the running server by pointing your web browser at <https://localhost:8080/>.

## Configuration

Ports, logging levels, and other parameters are configurable via environment variables. Please see the `.env` file and the `Dockerfile` for defaults for different environments.

## Contributing

We adhere to the [Contributor Covenant](https://www.contributor-covenant.org/version/2/0/code_of_conduct/) in order to maintain a respectful and productive community.

## Legal

Copyright (c) 2018-2020 Peat Bakke (<peat@text.camp>).

This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
