// Unless explicitly stated otherwise all files in this repository are licensed
// under the Apache License Version 2.0.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2024-present Datadog, Inc.

package agenthelp

import (
	_ "embed"
	"strings"
)

//go:embed guide.md
var guideContent string

// GetGuide returns the full steering guide.
func GetGuide() string {
	return guideContent
}

// GetGuideSection returns a specific domain section from the guide.
// Returns the full guide if the domain is not found.
func GetGuideSection(domain string) string {
	// Try capitalized version first (e.g., "## Logs")
	capitalized := strings.ToUpper(domain[:1]) + domain[1:]
	heading := "## " + capitalized
	idx := strings.Index(guideContent, heading)
	if idx == -1 {
		// Try exact case
		heading = "## " + domain
		idx = strings.Index(guideContent, heading)
		if idx == -1 {
			return guideContent
		}
	}

	// Find the next ## heading
	rest := guideContent[idx+len(heading):]
	nextSection := strings.Index(rest, "\n## ")
	if nextSection == -1 {
		return guideContent[idx:]
	}
	return guideContent[idx : idx+len(heading)+nextSection]
}
