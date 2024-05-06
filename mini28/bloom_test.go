package mini28

import "testing"

func TestIp(t *testing.T) {
	ip := Ip([]byte{252, 253, 254, 255})
	if got := ip.String(); got != "252.253.254.255" {
		t.Fatal(got)
	}
	if got := ip.Repr(); got != 0xfffefdfc {
		t.Fatalf("%x\n", got)
	}
}

func TestBloom(t *testing.T) {
	ip1 := Ip([]byte{252, 253, 254, 255})
	ip2 := Ip([]byte{0xca, 0xfe, 0xba, 0xbe})
	bloom := NewBloom(2, 0.5)

	if bloom.Lookup(ip1) || bloom.Lookup(ip2) {
		t.Fatal()
	}

	bloom.Insert(ip1)
	if !bloom.Lookup(ip1) {
		t.Fatal()
	}

	bloom.Insert(ip2)
	if !bloom.Lookup(ip1) {
		t.Fatal()
	}
	if !bloom.Lookup(ip2) {
		t.Fatal()
	}
}
