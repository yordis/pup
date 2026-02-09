// Unless explicitly stated otherwise all files in this repository are licensed
// under the Apache License Version 2.0.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2024-present Datadog, Inc.

package formatter

import (
	"errors"
	"strings"
	"testing"
)

func TestToJSON(t *testing.T) {
	tests := []struct {
		name      string
		data      interface{}
		wantError bool
		wantContains []string
	}{
		{
			name: "simple map",
			data: map[string]interface{}{
				"foo": "bar",
				"baz": 123,
			},
			wantError: false,
			wantContains: []string{`"foo"`, `"bar"`, `"baz"`, `123`},
		},
		{
			name: "struct",
			data: struct {
				Name string `json:"name"`
				Age  int    `json:"age"`
			}{
				Name: "Alice",
				Age:  30,
			},
			wantError: false,
			wantContains: []string{`"name"`, `"Alice"`, `"age"`, `30`},
		},
		{
			name:      "array",
			data:      []string{"a", "b", "c"},
			wantError: false,
			wantContains: []string{`"a"`, `"b"`, `"c"`},
		},
		{
			name:      "nil",
			data:      nil,
			wantError: false,
			wantContains: []string{`null`},
		},
		{
			name:      "empty map",
			data:      map[string]interface{}{},
			wantError: false,
			wantContains: []string{`{}`},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result, err := ToJSON(tt.data)

			if tt.wantError {
				if err == nil {
					t.Error("ToJSON() expected error but got none")
				}
				return
			}

			if err != nil {
				t.Errorf("ToJSON() unexpected error: %v", err)
				return
			}

			for _, want := range tt.wantContains {
				if !strings.Contains(result, want) {
					t.Errorf("ToJSON() result missing %q. Got: %s", want, result)
				}
			}
		})
	}
}

func TestToTable(t *testing.T) {
	tests := []struct {
		name         string
		data         interface{}
		wantError    bool
		wantContains []string
	}{
		{
			name: "map data",
			data: map[string]interface{}{
				"name":  "test",
				"value": 42,
			},
			wantError:    false,
			wantContains: []string{"name", "test", "value", "42"},
		},
		{
			name: "slice of maps",
			data: []interface{}{
				map[string]interface{}{
					"id":   1,
					"name": "test1",
				},
				map[string]interface{}{
					"id":   2,
					"name": "test2",
				},
			},
			wantError:    false,
			wantContains: []string{"ID", "NAME", "test1", "test2", "1", "2"},
		},
		{
			name:         "empty slice",
			data:         []interface{}{},
			wantError:    false,
			wantContains: []string{"No results found"},
		},
		{
			name: "API response wrapper with data array",
			data: map[string]interface{}{
				"data": []interface{}{
					map[string]interface{}{
						"id":     1,
						"title":  "Incident 1",
						"status": "active",
					},
					map[string]interface{}{
						"id":     2,
						"title":  "Incident 2",
						"status": "resolved",
					},
				},
				"meta": map[string]interface{}{
					"total": 2,
				},
			},
			wantError:    false,
			wantContains: []string{"ID", "TITLE", "STATUS", "Incident 1", "Incident 2", "active", "resolved"},
		},
		{
			name: "API response wrapper with single data object",
			data: map[string]interface{}{
				"data": map[string]interface{}{
					"id":     1,
					"title":  "Single Incident",
					"status": "active",
				},
				"meta": map[string]interface{}{
					"version": "1.0",
				},
			},
			wantError:    false,
			wantContains: []string{"id", "title", "status", "Single Incident", "active"},
		},
		{
			name: "JSON:API format with attributes",
			data: []interface{}{
				map[string]interface{}{
					"id":   "12345",
					"type": "incident",
					"attributes": map[string]interface{}{
						"title":      "Database timeout",
						"severity":   "SEV-2",
						"status":     "active",
						"created_at": "2024-01-15T10:30:00Z",
					},
				},
			},
			wantError: false,
			wantContains: []string{
				"12345", "incident", "Database timeout", "SEV-2", "active",
			},
		},
		{
			name: "JSON:API format with relationships",
			data: []interface{}{
				map[string]interface{}{
					"id":   "12345",
					"type": "incident",
					"attributes": map[string]interface{}{
						"title": "API Error",
					},
					"relationships": map[string]interface{}{
						"commander": map[string]interface{}{
							"data": map[string]interface{}{
								"id":   "user-123",
								"type": "user",
							},
						},
					},
				},
			},
			wantError: false,
			wantContains: []string{
				"12345", "incident", "API Error", "user-123",
			},
		},
		{
			name:         "nil data",
			data:         nil,
			wantError:    false,
			wantContains: []string{},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result, err := ToTable(tt.data)

			if tt.wantError {
				if err == nil {
					t.Error("ToTable() expected error but got none")
				}
				return
			}

			if err != nil {
				t.Errorf("ToTable() unexpected error: %v", err)
				return
			}

			for _, want := range tt.wantContains {
				if !strings.Contains(result, want) {
					t.Errorf("ToTable() result missing %q. Got: %s", want, result)
				}
			}
		})
	}
}

func TestToYAML(t *testing.T) {
	tests := []struct {
		name         string
		data         interface{}
		wantError    bool
		wantContains []string
	}{
		{
			name: "map data",
			data: map[string]interface{}{
				"name":  "test",
				"value": 42,
			},
			wantError:    false,
			wantContains: []string{"name:", "test", "value:", "42"},
		},
		{
			name: "slice data",
			data: []string{"a", "b", "c"},
			wantError:    false,
			wantContains: []string{"a", "b", "c"},
		},
		{
			name:      "nil data",
			data:      nil,
			wantError: false,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result, err := ToYAML(tt.data)

			if tt.wantError {
				if err == nil {
					t.Error("ToYAML() expected error but got none")
				}
				return
			}

			if err != nil {
				t.Errorf("ToYAML() unexpected error: %v", err)
				return
			}

			for _, want := range tt.wantContains {
				if !strings.Contains(result, want) {
					t.Errorf("ToYAML() result missing %q. Got: %s", want, result)
				}
			}
		})
	}
}

func TestFormatOutput(t *testing.T) {
	data := map[string]string{"key": "value"}

	tests := []struct {
		name      string
		format    OutputFormat
		wantError bool
	}{
		{
			name:      "JSON format",
			format:    FormatJSON,
			wantError: false,
		},
		{
			name:      "Table format",
			format:    FormatTable,
			wantError: false,
		},
		{
			name:      "YAML format",
			format:    FormatYAML,
			wantError: false,
		},
		{
			name:      "unknown format defaults to JSON",
			format:    OutputFormat("unknown"),
			wantError: false,
		},
		{
			name:      "empty format defaults to JSON",
			format:    OutputFormat(""),
			wantError: false,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result, err := FormatOutput(data, tt.format)

			if tt.wantError {
				if err == nil {
					t.Error("FormatOutput() expected error but got none")
				}
				return
			}

			if err != nil {
				t.Errorf("FormatOutput() unexpected error: %v", err)
				return
			}

			if result == "" {
				t.Error("FormatOutput() returned empty string")
			}

			// All formats should contain the data
			if !strings.Contains(result, "key") {
				t.Error("FormatOutput() should contain data")
			}
		})
	}
}

func TestFormatError(t *testing.T) {
	tests := []struct {
		name string
		err  error
		want string
	}{
		{
			name: "simple error",
			err:  errors.New("something went wrong"),
			want: "Error: something went wrong",
		},
		{
			name: "formatted error",
			err:  errors.New("failed to connect: connection refused"),
			want: "Error: failed to connect: connection refused",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := FormatError(tt.err)
			if result != tt.want {
				t.Errorf("FormatError() = %q, want %q", result, tt.want)
			}
		})
	}
}

func TestFormatSuccess(t *testing.T) {
	tests := []struct {
		name         string
		message      string
		data         interface{}
		wantError    bool
		wantContains []string
	}{
		{
			name:      "success with data",
			message:   "Operation completed",
			data:      map[string]string{"result": "OK"},
			wantError: false,
			wantContains: []string{
				`"status"`,
				`"success"`,
				`"message"`,
				`"Operation completed"`,
				`"data"`,
				`"result"`,
				`"OK"`,
			},
		},
		{
			name:      "success without data",
			message:   "Done",
			data:      nil,
			wantError: false,
			wantContains: []string{
				`"status"`,
				`"success"`,
				`"message"`,
				`"Done"`,
			},
		},
		{
			name:      "success with array data",
			message:   "List retrieved",
			data:      []int{1, 2, 3},
			wantError: false,
			wantContains: []string{
				`"status"`,
				`"success"`,
				`"data"`,
				`1`,
				`2`,
				`3`,
			},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result, err := FormatSuccess(tt.message, tt.data)

			if tt.wantError {
				if err == nil {
					t.Error("FormatSuccess() expected error but got none")
				}
				return
			}

			if err != nil {
				t.Errorf("FormatSuccess() unexpected error: %v", err)
				return
			}

			for _, want := range tt.wantContains {
				if !strings.Contains(result, want) {
					t.Errorf("FormatSuccess() result missing %q. Got: %s", want, result)
				}
			}
		})
	}
}

func TestOutputFormat_Constants(t *testing.T) {
	// Verify format constants are correctly defined
	if FormatJSON != "json" {
		t.Errorf("FormatJSON = %q, want \"json\"", FormatJSON)
	}
	if FormatTable != "table" {
		t.Errorf("FormatTable = %q, want \"table\"", FormatTable)
	}
	if FormatYAML != "yaml" {
		t.Errorf("FormatYAML = %q, want \"yaml\"", FormatYAML)
	}
}
