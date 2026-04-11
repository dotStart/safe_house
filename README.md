# safe_house

**This application is still very early in its development and may include a variety of bugs, 
constraints and other issues (in fact it is my first Rust application) - USE WITH EXTREME CAUTION**

A simple self-hosted end-to-end encrypted text and file sharing service.

## Screenshots

| Upload                                                 | Document                                             | Administration                                      |
|--------------------------------------------------------|------------------------------------------------------|-----------------------------------------------------|
| [![Upload View][screenshot_upload]][screenshot_upload] | [![Document View][screenshot_view]][screenshot_view] | [![Admin View][screenshot_admin]][screenshot_admin] |

[screenshot_upload]: screenshots/upload.png

[screenshot_view]: screenshots/view.png

[screenshot_admin]: screenshots/admin.png

## Running

safe_house ships as a self-contained single executable and will listen to HTTP requests on 
port `8000` by default.

There are multiple supported methods of deployment:

### Executable

```shell
./safe_house
```

### Docker

```shell
docker run -p 8000:8000 -v /my/path:/app/data ghcr.io/dotstart/safe_house:latest
```

## Configuration

By default, safe_house will attempt to load `safe_house.toml` within the current working
directory. If no such file can be found, safe defaults are assumed. Additionally, configuration
parameters may be set through environment variables prefixed by `SAFEHOUSE_`.

Refer to the [example configuration file](safe_house.example.toml) for information on the available
options as well as their environment variable versions.

Additionally, safe_house currently permits the direct configuration of its underlying web framework 
[rocket](https://rocket.rs) through `rocket.toml` or, alternatively, environment variables prefixed
by `ROCKET_`. Refer to the [official documentation](https://rocket.rs/guide/v0.4/configuration/) for
more information.

## Building

safe_house may be built via `cargo` just like any other rust application. Please note, however, that
by default, the web UI is excluded from the build in order to simplify development.

To run the application in development mode, you may use the following command:

```shell
cargo run --package safe_house --bin safe_house --profile dev
```

equally, the web UI may be served using `npm` from [its project directory](web/) using the following
command:

```shell
npm run dev
```

If you wish to build a full executable (including its UI), you will first have to build the web UI
from its directory using `npm`:

```shell
npm run build
```

and then build the self-contained executable using `cargo`:

```shell
cargo build --features ui --package safe_house --bin safe_house --profile release
```

The resulting executables may be found in the `target/release` directory (or `target/dev` when
using the `dev` profile).

## License

```
This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as
published by the Free Software Foundation, either version 3 of the
License, or (at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
```

A [full version of the license](LICENSE.md) is included within this repository.