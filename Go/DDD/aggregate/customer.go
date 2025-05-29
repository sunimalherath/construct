package aggregate

import (
	"errors"

	"github.com/google/uuid"
	"github.com/sunimalherath/construct/ddd/entity"
	"github.com/sunimalherath/construct/ddd/valueobject"
)

var (
	ErrInvalidPerson = errors.New("invalid person")
)

type Customer struct {
	person   *entity.Person // Person is the root entity of Customer, hence person.ID will be the main identifier for the customer.
	products []*entity.Item

	transactions []valueobject.Transaction
}

func NewCustomer(name string) (Customer, error) {
	if name == "" {
		return Customer{}, ErrInvalidPerson
	}

	person := &entity.Person{
		ID:   uuid.New(),
		Name: name,
	}

	return Customer{
		person:       person,
		products:     make([]*entity.Item, 0),
		transactions: make([]valueobject.Transaction, 0),
	}, nil
}

// pointer types used here because the Entities can change state.
