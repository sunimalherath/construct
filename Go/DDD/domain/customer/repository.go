package customer

import (
	"errors"

	"github.com/google/uuid"
	"github.com/sunimalherath/construct/ddd/aggregate"
)

var (
	ErrCustomerNotFound       = errors.New("customer not found")
	ErrFailedToUpdateCustomer = errors.New("failed to update customer")
	ErrUpdateCustomer         = errors.New("failed to update customer")
)

type CustomerRepository interface {
	Get(uuid.UUID) (aggregate.Customer, error)
	Add(aggregate.Customer) error
	Update(aggregate.Customer) error
}
