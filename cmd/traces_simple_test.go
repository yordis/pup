// Unless explicitly stated otherwise all files in this repository are licensed
// under the Apache License Version 2.0.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2024-present Datadog, Inc.

package cmd

import (
	"strings"
	"testing"
)

func TestTracesCmd(t *testing.T) {
	if tracesCmd == nil {
		t.Fatal("tracesCmd is nil")
	}

	if tracesCmd.Use != "traces" {
		t.Errorf("Use = %s, want traces", tracesCmd.Use)
	}

	// Test that the command returns error (under development)
	err := tracesCmd.RunE(tracesCmd, []string{})
	if err == nil {
		t.Error("Expected error from traces command (under development)")
	}
	if !strings.Contains(err.Error(), "under development") {
		t.Errorf("Expected 'under development' error, got: %v", err)
	}
}
