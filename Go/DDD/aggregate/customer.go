package aggregate

import (
	"github.com/sunimalherath/construct/ddd/entity"
	"github.com/sunimalherath/construct/ddd/valueobject"
)

type Customer struct {
	person   *entity.Person // Person is the root entity of Customer, hence person.ID will be the main identifier for the customer.
	products []*entity.Item

	transactions valueobject.Transaction
}
