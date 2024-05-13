package main

import "fmt"

func main() {
    bloom := NewBloom(100000, 0.2)
    fmt.Printf("n = %v\n", bloom.size())
    fmt.Printf("Bloom uses %v hashes\n", bloom.nHashes())

    c := 0
    for i := 0; i <= 1000; i++ {
        ip := Ip([]byte{252, byte(i * 37), byte(i / 256), byte(i % 256)});
        if bloom.Lookup(ip) {
            c++
        }
        bloom.Insert(ip)
    }
    fmt.Println(c)
}
