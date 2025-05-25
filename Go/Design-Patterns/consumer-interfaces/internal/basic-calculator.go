package internal

import "errors"

type BasicCal struct {
}

func (b BasicCal) Add(num1, num2 int) int {
	return num1 + num2
}

func (b BasicCal) Subtract(num1, num2 int) int {
	return num1 - num2
}

func (b BasicCal) Multiply(num1, num2 int) int {
	return num1 * num2
}

func (b BasicCal) Divide(num1, num2 int) (*int, error) {
	if num2 == 0 {
		return nil, errors.New("")
	}

	ans := num1 / num2

	return &ans, nil
}
