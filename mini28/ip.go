package main

import "fmt"

type Ip [4]byte

func (ip Ip) Repr() uint32 {
	var repr uint32
	for i := 0; i < 4; i++ {
		repr += uint32(ip[i]) << (i * 8)
	}
	return repr
}

func (ip Ip) String() string {
	return fmt.Sprintf("%d.%d.%d.%d", ip[0], ip[1], ip[2], ip[3])
}
