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

/*
Customer fields are not accessible from outside the package - Why? Aggregates should not be directly accessible to access the data.
It's not up to the aggregate how the data is formatted, ==>it's up to the value object.
both person and product are entities and hence are mutable - that's why the pointer types are used.
This reflects the changes here if the person or products are changed elsewhere.
transactions are value objects (immutable) and hence don't use pointers here.
*/

type Customer struct {
	person   *entity.Person // Person is the root entity of Customer, hence person.ID will be the main identifier for the customer.
	products []*entity.Item // Customer can buy many products

	transactions []valueobject.Transaction // Customer can have many transactions
}

// NewCustomer is a factory to create a new customer.
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
