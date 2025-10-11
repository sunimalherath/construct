package api

import (
	"encoding/json"
	"net/http"
)

type Book struct {
	Title  string
	Author string
	ISBN   string
}

func (b Book) ToJSON() []byte {
	jsonData, err := json.Marshal(b)
	if err != nil {
		panic(err)
	}

	return jsonData
}

func FromJSON(data []byte) Book {
	book := Book{}

	err := json.Unmarshal(data, &book)
	if err != nil {
		panic(err)
	}

	return book
}

func BooksHandleFunc(w http.ResponseWriter, r *http.Request) {
	//
}
