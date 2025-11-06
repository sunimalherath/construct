package main

import (
	"fmt"
	"sync"
)

var (
	wg              sync.WaitGroup
	widgetInventory int = 1000
	inventoryCh         = make(chan int, 10000)
)

func main() {
	fmt.Println("Starting inventory count = ", widgetInventory)

	wg.Add(2)

	go makeSales(inventoryCh)
	go newPurchases(inventoryCh)

	wg.Wait()

	for len(inventoryCh) > 0 {
		widgetInventory += <-inventoryCh
	}

	fmt.Println("Ending inventory count = ", widgetInventory)
}

func makeSales(saleCh chan int) {
	for range 3000 {
		saleCh <- -100
	}

	wg.Done()
}

func newPurchases(purchaseCh chan int) {
	for range 3000 {
		purchaseCh <- 100
	}

	wg.Done()
}
