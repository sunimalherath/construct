package entity

import "github.com/google/uuid"

// This is an entity to represent a person in all the domains
type Item struct {
	ID          uuid.UUID
	Name        string
	Description string
}
