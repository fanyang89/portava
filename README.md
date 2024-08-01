# portava

portava - are the ports available?

## Usage

```bash
Usage: portava [OPTIONS] <COMMAND>

Commands:
  listen   Try to listen at range
  connect  Try to connect to range
  help     Print this message or the help of the given subcommand(s)
```

Example:

```bash
# run at peer1
portava listen --ip-addr 127.0.0.1 --port-range 10000-20000
```

```bash
# run at peer2
portava connect --ip-addr 127.0.0.1 --port-range 10000-20000
```

Then:

```
2024-08-01T14:55:49.963656+08:00  INFO portava: Bind success ports: 10000-12344,12346-13579,13582-13601,13603-20000
2024-08-01T14:55:49.963824+08:00  INFO portava: Bind failed ports: 12345,13580-13581,13602
2024-08-01T14:56:09.690614+08:00  INFO portava: Accept success ports: 10000-12344,12346-13579,13582-13601,13603-20000
```
