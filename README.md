A simple command line tool for testing TCP handing time comsumed of ip:port, which can read ip:port list from STDIN of file.

``` bash
➜  test_tcp_lantency git:(master) ✗ cargo run -- -h
test_tcp_lantency 0.1.0

QingGo

USAGE:
    test_tcp_lantency [OPTIONS] [FILE]

ARGS:
    <FILE>    

OPTIONS:
    -h, --help              Print help information
    -r <REPEAT_TIME>        [default: 3]
    -V, --version           Print version information

➜  test_tcp_lantency git:(master) ✗ cargo run -- ./ip_list.txt -r 5
127.0.0.1:80 9999.0
185.199.108.154:80 0.3083489916

➜  test_tcp_lantency git:(master) ✗ cat ip_list.txt |  cargo run 
127.0.0.1:80 9999.0
185.199.108.154:80 0.34357951366666667
```