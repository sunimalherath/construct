package internal

type AddCalculator interface {
	Add(num1, num2 int) int
}

type JustAdd struct {
	AddCalc AddCalculator
}

func NewJustAdd(addCalc AddCalculator) *JustAdd {
	return &JustAdd{
		AddCalc: addCalc,
	}
}

func (j JustAdd) JustAddTheNumbers(n1, n2 int) int {
	return j.AddCalc.Add(n1, n2)
}
