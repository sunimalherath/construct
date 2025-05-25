package main

import (
	"fmt"

	"github.com/sunimalherath/construct/consumer-interfaces/internal"
)

func main() {
	basicCalc := internal.BasicCal{}

	justAddCalc := internal.NewJustAdd(basicCalc)

	n1, n2 := 2, 3

	ans := justAddCalc.JustAddTheNumbers(n1, n2)

	fmt.Printf("Just Adding %d and %d returns %d\n", 2, 3, ans)
}
