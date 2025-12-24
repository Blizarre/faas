# faas

State of the art! Async code, Rust!, You did not know you wanted it, but it's there!

_Fortune As A Service_

https://faas.marache.net

```
$ curl -s https://faas.marache.net/txt/
"Text processing has made it possible to right-justify any idea, even
one which cannot be justified on any other grounds."
                -- J. Finnegan, USC.⏎
```

The service expose `/` and `/<id>` as html pages with a bottom link to the page itself, and `/txt` and `/txt/<id>` as pure `text/plain` endpoints.

The docker image is built using the official [Docker Hardened images](https://dhi.io/). User ID/Group ID: 65532/65532.

## Configuration

- `fortune_path`: Location of the fortunes file
- `url_prefix`: Url prefix used to generate the share links (`{{url_prefix}}/{{id}}`).

example:

```bash
ROCKET_FORTUNE_PATH=../../fortunes ROCKET_URL_PREFIX=http://localhost:8000/ ./faas
```

Or use a [Rocket configuration file](https://rocket.rs/v0.5-rc/guide/configuration/#rockettoml).

# Performance

Using [baton](https://github.com/americanexpress/baton), my 4-cores arm server in oracle cloud managed to serve requests at a peak 400'000 requests per minutes from my home laptop:

```
$ ./baton -c 200 -r 100000 -u https://faas.marache.net
Configuring to send GET requests to: https://faas.marache.net
Generating the requests...
Finished generating the requests
Sending the requests to the server...
Finished sending the requests
Processing the results...


====================== Results ======================
Total requests:                                100000
Time taken to complete requests:        18.062498286s
Requests per second:                             5536
===================== Breakdown =====================
```

That should be good enough for now.

## Cookies

The file `fortunes` is built from the following cookies:
`ascii-art computers cookie science`
They come from the original package https://packages.debian.org/bullseye/fortune-mod, fortune-mod_1.99.1.orig.tar.gz
