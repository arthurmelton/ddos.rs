# ddos.rs
A ddos tool coded in rust

# install
```
rustc ddos.rs
```

# run
```
.//ddos* [ip with port] (wait x miliseconds per request)
```
ex (if you do a website dont put the https:// just do everything after)
```
.//ddos* www.google.com:80   - ddos google.com at port 80 unthrodled
.//ddos* 127.0.0.1:80 500    - ddos 127.0.0.1 at port 80 with 1 request every 500 miliseconds
.//ddos* 127.0.0.1:80        - ddos 127.0.0.1 at port 80 unthrodled
```
