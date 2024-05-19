package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	callgraph, err := ReadCallgraph(scanner)
	if err != nil {
		panic(err)
	}
	fmt.Println(BuildReport(callgraph))
}
