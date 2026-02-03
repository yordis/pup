// Unless explicitly stated otherwise all files in this repository are licensed
// under the Apache License Version 2.0.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2024-present Datadog, Inc.

package util

import (
	"fmt"
	"regexp"
	"strconv"
	"time"
)

// ParseTimeParam parses time parameters supporting multiple formats:
// - "now" for current time
// - Unix timestamps (e.g., "1704067200")
// - Relative time (e.g., "1h", "30m", "7d")
// - ISO date strings (e.g., "2024-01-01T00:00:00Z")
func ParseTimeParam(timeStr string) (time.Time, error) {
	// Handle "now"
	if timeStr == "now" {
		return time.Now(), nil
	}

	// Try parsing as Unix timestamp
	if matched, _ := regexp.MatchString(`^\d+$`, timeStr); matched {
		timestamp, err := strconv.ParseInt(timeStr, 10, 64)
		if err == nil {
			return time.Unix(timestamp, 0), nil
		}
	}

	// Try parsing relative time (e.g., "1h", "30m", "2d")
	re := regexp.MustCompile(`^(\d+)([smhd])$`)
	matches := re.FindStringSubmatch(timeStr)
	if len(matches) == 3 {
		value, err := strconv.Atoi(matches[1])
		if err != nil {
			return time.Time{}, fmt.Errorf("invalid time value: %w", err)
		}

		unit := matches[2]
		var duration time.Duration

		switch unit {
		case "s":
			duration = time.Duration(value) * time.Second
		case "m":
			duration = time.Duration(value) * time.Minute
		case "h":
			duration = time.Duration(value) * time.Hour
		case "d":
			duration = time.Duration(value) * 24 * time.Hour
		}

		return time.Now().Add(-duration), nil
	}

	// Try parsing as ISO date
	t, err := time.Parse(time.RFC3339, timeStr)
	if err == nil {
		return t, nil
	}

	return time.Time{}, fmt.Errorf("invalid time format: %s", timeStr)
}

// ParseTimeToUnix parses time string and returns Unix timestamp in seconds
func ParseTimeToUnix(timeStr string) (int64, error) {
	t, err := ParseTimeParam(timeStr)
	if err != nil {
		return 0, err
	}
	return t.Unix(), nil
}

// ParseTimeToUnixMilli parses time string and returns Unix timestamp in milliseconds
func ParseTimeToUnixMilli(timeStr string) (int64, error) {
	t, err := ParseTimeParam(timeStr)
	if err != nil {
		return 0, err
	}
	return t.UnixMilli(), nil
}
