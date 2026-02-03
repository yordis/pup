// Unless explicitly stated otherwise all files in this repository are licensed
// under the Apache License Version 2.0.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2024-present Datadog, Inc.

package formatter

import (
	"encoding/json"
	"fmt"
)

// OutputFormat represents the output format type
type OutputFormat string

const (
	// FormatJSON outputs in JSON format
	FormatJSON OutputFormat = "json"
	// FormatTable outputs in table format
	FormatTable OutputFormat = "table"
	// FormatYAML outputs in YAML format
	FormatYAML OutputFormat = "yaml"
)

// FormatOutput formats the output based on the specified format
func FormatOutput(data interface{}, format OutputFormat) (string, error) {
	switch format {
	case FormatJSON:
		return ToJSON(data)
	case FormatTable:
		return ToTable(data)
	case FormatYAML:
		return ToYAML(data)
	default:
		return ToJSON(data)
	}
}

// ToJSON formats data as JSON
func ToJSON(data interface{}) (string, error) {
	bytes, err := json.MarshalIndent(data, "", "  ")
	if err != nil {
		return "", fmt.Errorf("failed to marshal JSON: %w", err)
	}
	return string(bytes), nil
}

// ToTable formats data as a table (simplified for now)
func ToTable(data interface{}) (string, error) {
	// For now, just use JSON. We can enhance this later with proper table formatting
	return ToJSON(data)
}

// ToYAML formats data as YAML (simplified for now)
func ToYAML(data interface{}) (string, error) {
	// For now, just use JSON. We can add YAML library later
	return ToJSON(data)
}

// FormatError formats an error message
func FormatError(err error) string {
	return fmt.Sprintf("Error: %v", err)
}

// FormatSuccess formats a success message with optional data
func FormatSuccess(message string, data interface{}) (string, error) {
	result := map[string]interface{}{
		"status":  "success",
		"message": message,
	}
	if data != nil {
		result["data"] = data
	}
	return ToJSON(result)
}
