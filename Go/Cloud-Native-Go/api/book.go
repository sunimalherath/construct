package api

import (
	"encoding/json"
	"net/http"
)

type Book struct {
	Title  string `json:"title"`
	Author string `json:"author"`
	ISBN   string `json:"isbn"`
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

var books = []Book{
	{Title: "Go", Author: "John", ISBN: "0123456789"},
	{Title: "Swift", Author: "Jane", ISBN: "9876543210"},
}

func AllBooks() []Book {
	return books
}

func writeJSON(w http.ResponseWriter, books []Book) {
	booksJData, err := json.Marshal(books)
	if err != nil {
		panic(err)
	}

	w.Header().Add("Content Type", "application/json; charset-udf-8")
	w.Write(booksJData)
}

func BooksHandleFunc(w http.ResponseWriter, r *http.Request) {
	switch r.Method {
	case http.MethodGet:
		books := AllBooks()
		writeJSON(w, books)
	default:
		w.WriteHeader(http.StatusBadRequest)
		w.Write([]byte("Unsupported request method"))
	}
}
