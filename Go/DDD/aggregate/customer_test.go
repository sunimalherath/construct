package aggregate

import (
	"errors"
	"testing"
)

func TestCustomer_NewCustomer(t *testing.T) {
	type testCase struct {
		testName    string
		name        string
		expectedErr error
	}

	testCases := []testCase{
		{
			testName:    "Empty name validation check",
			name:        "",
			expectedErr: ErrInvalidPerson,
		},
		{
			testName:    "valid person",
			name:        "John Doe",
			expectedErr: nil,
		},
	}

	for _, tc := range testCases {
		t.Run(tc.testName, func(t *testing.T) {
			_, err := NewCustomer(tc.name)
			if !errors.Is(err, tc.expectedErr) {
				t.Errorf("Expected error %v, but got %v", tc.expectedErr, err)
			}
		})
	}
}
