# NetCheck server
Rust programming language playground project written just for fun. Initial purpose was to start using it instead of slow ``curl ifconfig.me`` (which I use a lot). So no need to install ``curl`` now when it is not needed, because `netcat` or `telnet` is sufficient and usually preinstalled.

#### Build and run
```
cargo build --release
./scripts/netcheck_server start
./scripts/netcheck_server stop
```

#### Live test
```
nc mov.lt 80
```
_I'm using this server for development so there is no guaranty that live test will work 24/7._
