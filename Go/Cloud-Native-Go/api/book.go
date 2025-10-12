package api

import (
	"encoding/json"
	"io"
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

var books = map[string]Book{
	"0123456789": {Title: "Go", Author: "John", ISBN: "0123456789"},
	"9876543210": {Title: "Swift", Author: "Jane", ISBN: "9876543210"},
}

func AllBooks() map[string]Book {
	return books
}

func CreateBook(book Book) (string, bool) {
	books[book.ISBN] = book

	return book.ISBN, true
}

func GetBook(isbn string) (Book, bool) {
	if bk, found := books[isbn]; found {
		return bk, true
	}

	return Book{}, false
}

func UpdateBook(isbn string, book Book) bool {
	_, found := GetBook(isbn)
	if !found {
		return false
	}

	books[isbn] = book

	return true
}

func DeleteBook(isbn string) {
	delete(books, isbn)
}

func writeJSON(w http.ResponseWriter, books any) {
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
	case http.MethodPost:
		body, err := io.ReadAll(r.Body)
		if err != nil {
			w.WriteHeader(http.StatusInternalServerError)
		}

		book := FromJSON(body)

		isbn, created := CreateBook(book)
		if created {
			w.Header().Add("location", "/api/books/"+isbn)
		} else {
			w.WriteHeader(http.StatusConflict)
		}

	default:
		w.WriteHeader(http.StatusBadRequest)
		w.Write([]byte("Unsupported request method"))
	}
}

func BookHandleFunc(w http.ResponseWriter, r *http.Request) {
	isbn := r.URL.Path[len("/api/books/"):]

	switch method := r.Method; method {
	case http.MethodGet:
		book, found := GetBook(isbn)
		if found {
			writeJSON(w, book)
		} else {
			w.WriteHeader(http.StatusNotFound)
		}
	case http.MethodPut:
		body, err := io.ReadAll(r.Body)
		if err != nil {
			w.WriteHeader(http.StatusInternalServerError)
		}

		book := FromJSON(body)
		exists := UpdateBook(isbn, book)
		if exists {
			w.WriteHeader(http.StatusOK)
		} else {
			w.WriteHeader(http.StatusNotFound)
		}
	case http.MethodDelete:
		DeleteBook(isbn)
		w.WriteHeader(http.StatusOK)
	default:
		w.WriteHeader(http.StatusBadRequest)
		w.Write([]byte("Unsupported request method"))
	}
}
