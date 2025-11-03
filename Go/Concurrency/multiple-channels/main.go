package main

import (
	"fmt"
	"os"
	"time"
)

func main() {
	fmt.Println("Multiple Channels with os.Exit")

	x := make(chan string)
	y := make(chan string)
	z := make(chan string)

	go func() {
		for {
			time.Sleep(1 * time.Second)
			x <- "message from X"
		}
	}()

	go func() {
		for {
			time.Sleep(3 * time.Second)
			y <- "message from Y"
		}
	}()

	go func() {
		for {
			time.Sleep(10 * time.Second)
			z <- "time to terminate"
		}
	}()

	for {
		select {
		case msg := <-x:
			fmt.Println(msg)
		case msg := <-y:
			fmt.Println(msg)
		case <-z:
			os.Exit(1)

		}
	}
}
