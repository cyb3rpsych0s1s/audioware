# Plugin settings

Provided you installed [ModSettings](https://github.com/jackhumbert/mod_settings) you can customize plugin settings, as follow:

| settings    | possible values                    | default |
|-------------|------------------------------------|---------|
| buffer size | `auto` / `64` / `128` / `256` / `512` / `1024` | `auto`    |

## Buffer size

`buffer size` refers to the amount of audio data that is processed *at one time*. It can directly affect the performance and responsiveness of your audio application.

- smaller buffer sizes reduce latency but require more processing power.
- larger buffer sizes increase stability at the cost of higher latency.

> `auto` is used when no specific buffer size is set and uses default behavior of your machine.
