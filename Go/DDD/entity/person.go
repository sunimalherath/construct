package entity

import "github.com/google/uuid"

// This is an entity to represent a person in all the domains
type Person struct {
	ID   uuid.UUID
	Name string
	Age  int
}
