// Unless explicitly stated otherwise all files in this repository are licensed
// under the Apache License Version 2.0.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2024-present Datadog, Inc.

package cmd

import (
	"bytes"
	"encoding/json"
	"os"
	"strings"
	"testing"

	"github.com/DataDog/pup/pkg/agenthelp"
	"github.com/DataDog/pup/pkg/config"
)

func TestAgentSchema(t *testing.T) {
	// Save and restore originals
	origWriter := outputWriter
	origCfg := cfg
	origClient := ddClient
	defer func() {
		outputWriter = origWriter
		cfg = origCfg
		ddClient = origClient
	}()

	var buf bytes.Buffer
	outputWriter = &buf
	cfg = &config.Config{Site: "datadoghq.com"}
	ddClient = nil

	err := ExecuteWithArgs([]string{"agent", "schema"})
	if err != nil {
		t.Fatalf("agent schema error: %v", err)
	}

	output := buf.String()
	if output == "" {
		t.Fatal("agent schema should produce output")
	}

	// Verify it's valid JSON
	var schema agenthelp.Schema
	if err := json.Unmarshal([]byte(output), &schema); err != nil {
		t.Fatalf("agent schema output is not valid JSON: %v", err)
	}

	if schema.Version == "" {
		t.Error("schema version should not be empty")
	}
	if len(schema.Commands) == 0 {
		t.Error("schema commands should not be empty")
	}
	if len(schema.QuerySyntax) == 0 {
		t.Error("schema query_syntax should not be empty")
	}
}

func TestAgentSchemaCompact(t *testing.T) {
	origWriter := outputWriter
	origCfg := cfg
	origClient := ddClient
	defer func() {
		outputWriter = origWriter
		cfg = origCfg
		ddClient = origClient
	}()

	var buf bytes.Buffer
	outputWriter = &buf
	cfg = &config.Config{Site: "datadoghq.com"}
	ddClient = nil

	err := ExecuteWithArgs([]string{"agent", "schema", "--compact"})
	if err != nil {
		t.Fatalf("agent schema --compact error: %v", err)
	}

	output := buf.String()

	var compact agenthelp.CompactSchema
	if err := json.Unmarshal([]byte(output), &compact); err != nil {
		t.Fatalf("agent schema --compact output is not valid JSON: %v", err)
	}

	if compact.Version == "" {
		t.Error("compact schema version should not be empty")
	}
	if len(compact.Commands) == 0 {
		t.Error("compact schema commands should not be empty")
	}
}

func TestAgentGuide(t *testing.T) {
	origWriter := outputWriter
	origCfg := cfg
	origClient := ddClient
	defer func() {
		outputWriter = origWriter
		cfg = origCfg
		ddClient = origClient
	}()

	var buf bytes.Buffer
	outputWriter = &buf
	cfg = &config.Config{Site: "datadoghq.com"}
	ddClient = nil

	err := ExecuteWithArgs([]string{"agent", "guide"})
	if err != nil {
		t.Fatalf("agent guide error: %v", err)
	}

	output := buf.String()
	if !strings.Contains(output, "Pup Agent Guide") {
		t.Error("agent guide should contain the title")
	}
}

func TestAgentGuideDomain(t *testing.T) {
	origWriter := outputWriter
	origCfg := cfg
	origClient := ddClient
	defer func() {
		outputWriter = origWriter
		cfg = origCfg
		ddClient = origClient
	}()

	var buf bytes.Buffer
	outputWriter = &buf
	cfg = &config.Config{Site: "datadoghq.com"}
	ddClient = nil

	err := ExecuteWithArgs([]string{"agent", "guide", "logs"})
	if err != nil {
		t.Fatalf("agent guide logs error: %v", err)
	}

	output := buf.String()
	if !strings.Contains(output, "Logs") {
		t.Error("agent guide logs should contain Logs content")
	}
}

func TestHlpFlag(t *testing.T) {
	origWriter := outputWriter
	origCfg := cfg
	origClient := ddClient
	defer func() {
		outputWriter = origWriter
		cfg = origCfg
		ddClient = origClient
	}()

	var buf bytes.Buffer
	outputWriter = &buf
	cfg = &config.Config{Site: "datadoghq.com"}
	ddClient = nil

	err := ExecuteWithArgs([]string{"--hlp"})
	if err != nil {
		t.Fatalf("--hlp error: %v", err)
	}

	output := buf.String()

	var schema agenthelp.Schema
	if err := json.Unmarshal([]byte(output), &schema); err != nil {
		t.Fatalf("--hlp output is not valid JSON: %v", err)
	}

	if len(schema.Commands) == 0 {
		t.Error("--hlp schema commands should not be empty")
	}
}

func TestHlpFlagSubtree(t *testing.T) {
	origWriter := outputWriter
	origCfg := cfg
	origClient := ddClient
	defer func() {
		outputWriter = origWriter
		cfg = origCfg
		ddClient = origClient
	}()

	var buf bytes.Buffer
	outputWriter = &buf
	cfg = &config.Config{Site: "datadoghq.com"}
	ddClient = nil

	err := ExecuteWithArgs([]string{"monitors", "--hlp"})
	if err != nil {
		t.Fatalf("monitors --hlp error: %v", err)
	}

	output := buf.String()

	var schema agenthelp.Schema
	if err := json.Unmarshal([]byte(output), &schema); err != nil {
		t.Fatalf("monitors --hlp output is not valid JSON: %v", err)
	}

	if len(schema.Commands) != 1 {
		t.Errorf("monitors --hlp should have 1 command, got %d", len(schema.Commands))
	}
	if schema.Commands[0].Name != "monitors" {
		t.Errorf("monitors --hlp command name = %q, want 'monitors'", schema.Commands[0].Name)
	}
}

func TestAgentModeAutoDetect(t *testing.T) {
	origCfg := cfg
	origClient := ddClient
	origWriter := outputWriter
	defer func() {
		cfg = origCfg
		ddClient = origClient
		outputWriter = origWriter
		os.Unsetenv("CLAUDECODE")
	}()

	var buf bytes.Buffer
	outputWriter = &buf
	ddClient = nil

	// Set CLAUDECODE to trigger agent mode
	os.Setenv("CLAUDECODE", "1")

	// Reset cfg so initConfig runs with the env var
	cfg = nil

	// initConfig is called by cobra.OnInitialize, simulate it
	initConfig()

	if !cfg.AgentMode {
		t.Error("AgentMode should be true when CLAUDECODE=1")
	}
	if !cfg.AutoApprove {
		t.Error("AutoApprove should be true in agent mode")
	}
}

func TestAgentModeFlagOverride(t *testing.T) {
	origCfg := cfg
	origClient := ddClient
	origWriter := outputWriter
	origAgentFlag := agentFlag
	defer func() {
		cfg = origCfg
		ddClient = origClient
		outputWriter = origWriter
		agentFlag = origAgentFlag
	}()

	var buf bytes.Buffer
	outputWriter = &buf
	ddClient = nil

	agentFlag = true
	cfg = nil
	initConfig()

	if !cfg.AgentMode {
		t.Error("AgentMode should be true when --agent flag is set")
	}
	if !cfg.AutoApprove {
		t.Error("AutoApprove should be true when --agent flag is set")
	}
}
