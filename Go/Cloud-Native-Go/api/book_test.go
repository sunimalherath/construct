package api

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestBookToJSON(t *testing.T) {
	book := Book{
		Title:  "Go book",
		Author: "Some Guy",
		ISBN:   "0123456789",
	}

	jd := book.ToJSON()

	assert.Equal(t, `{"title":"Go book","author":"Some Guy","isbn":"0123456789"}`, string(jd), "JSON marshalling incorrect")
}

func TestBookFromJSON(t *testing.T) {
	json := []byte(`{"title":"Go book","author":"Some Guy","isbn":"0123456789"}`)

	book := FromJSON(json)

	assert.Equal(t, Book{Title: "Go book", Author: "Some Guy", ISBN: "0123456789"}, book, "JSON unmarshalling is incorrect")
}
