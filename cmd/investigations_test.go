// Unless explicitly stated otherwise all files in this repository are licensed
// under the Apache License Version 2.0.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2024-present Datadog, Inc.

package cmd

import (
	"bytes"
	"encoding/json"
	"io"
	"net/http"
	"os"
	"strings"
	"testing"
)

func TestInvestigationsCmd(t *testing.T) {
	if investigationsCmd == nil {
		t.Fatal("investigationsCmd is nil")
	}

	if investigationsCmd.Use != "investigations" {
		t.Errorf("Use = %s, want investigations", investigationsCmd.Use)
	}

	if investigationsCmd.Short == "" {
		t.Error("Short description is empty")
	}

	if investigationsCmd.Long == "" {
		t.Error("Long description is empty")
	}
}

func TestInvestigationsCmd_Subcommands(t *testing.T) {
	expectedCommands := []string{"trigger", "get", "list"}

	commands := investigationsCmd.Commands()

	commandMap := make(map[string]bool)
	for _, cmd := range commands {
		commandMap[cmd.Name()] = true
	}

	for _, expected := range expectedCommands {
		if !commandMap[expected] {
			t.Errorf("Missing subcommand: %s", expected)
		}
	}
}

func TestInvestigationsTriggerCmd(t *testing.T) {
	if investigationsTriggerCmd == nil {
		t.Fatal("investigationsTriggerCmd is nil")
	}

	if investigationsTriggerCmd.Use != "trigger" {
		t.Errorf("Use = %s, want trigger", investigationsTriggerCmd.Use)
	}

	if investigationsTriggerCmd.Short == "" {
		t.Error("Short description is empty")
	}

	if investigationsTriggerCmd.RunE == nil {
		t.Error("RunE is nil")
	}

	// Check flags
	flags := investigationsTriggerCmd.Flags()
	requiredFlags := []string{"type", "monitor-id", "event-id", "event-ts"}
	for _, name := range requiredFlags {
		if flags.Lookup(name) == nil {
			t.Errorf("Missing --%s flag", name)
		}
	}
}

func TestInvestigationsGetCmd(t *testing.T) {
	if investigationsGetCmd == nil {
		t.Fatal("investigationsGetCmd is nil")
	}

	if investigationsGetCmd.Use != "get [investigation-id]" {
		t.Errorf("Use = %s, want 'get [investigation-id]'", investigationsGetCmd.Use)
	}

	if investigationsGetCmd.Short == "" {
		t.Error("Short description is empty")
	}

	if investigationsGetCmd.RunE == nil {
		t.Error("RunE is nil")
	}

	if investigationsGetCmd.Args == nil {
		t.Error("Args validator is nil")
	}
}

func TestInvestigationsListCmd(t *testing.T) {
	if investigationsListCmd == nil {
		t.Fatal("investigationsListCmd is nil")
	}

	if investigationsListCmd.Use != "list" {
		t.Errorf("Use = %s, want list", investigationsListCmd.Use)
	}

	if investigationsListCmd.Short == "" {
		t.Error("Short description is empty")
	}

	if investigationsListCmd.RunE == nil {
		t.Error("RunE is nil")
	}

	flags := investigationsListCmd.Flags()
	listFlags := []string{"page-offset", "page-limit", "monitor-id"}
	for _, name := range listFlags {
		if flags.Lookup(name) == nil {
			t.Errorf("Missing --%s flag", name)
		}
	}
}

func TestInvestigationsCmd_ParentChild(t *testing.T) {
	commands := investigationsCmd.Commands()

	for _, cmd := range commands {
		if cmd.Parent() != investigationsCmd {
			t.Errorf("Command %s parent is not investigationsCmd", cmd.Use)
		}
	}
}

func TestBuildTriggerRequestBody_MonitorAlert(t *testing.T) {
	// Save and restore globals
	origType := invTriggerType
	origMonitorID := invMonitorID
	origEventID := invEventID
	origEventTS := invEventTS
	defer func() {
		invTriggerType = origType
		invMonitorID = origMonitorID
		invEventID = origEventID
		invEventTS = origEventTS
	}()

	invTriggerType = "monitor_alert"
	invMonitorID = 123456
	invEventID = "evt-abc-123"
	invEventTS = 1706918956000

	body, err := buildTriggerRequestBody()
	if err != nil {
		t.Fatalf("buildTriggerRequestBody() error = %v", err)
	}

	// Verify structure
	data, ok := body["data"].(map[string]any)
	if !ok {
		t.Fatal("body[data] is not a map")
	}

	if data["type"] != "trigger_investigation_request" {
		t.Errorf("data.type = %v, want trigger_investigation_request", data["type"])
	}

	attrs, ok := data["attributes"].(map[string]any)
	if !ok {
		t.Fatal("data.attributes is not a map")
	}

	trigger, ok := attrs["trigger"].(map[string]any)
	if !ok {
		t.Fatal("attributes.trigger is not a map")
	}

	if trigger["type"] != "monitor_alert_trigger" {
		t.Errorf("trigger.type = %v, want monitor_alert_trigger", trigger["type"])
	}

	mat, ok := trigger["monitor_alert_trigger"].(map[string]any)
	if !ok {
		t.Fatal("trigger.monitor_alert_trigger is not a map")
	}

	if mat["monitor_id"] != int64(123456) {
		t.Errorf("monitor_id = %v, want 123456", mat["monitor_id"])
	}

	if mat["event_id"] != "evt-abc-123" {
		t.Errorf("event_id = %v, want evt-abc-123", mat["event_id"])
	}

	if mat["event_ts"] != int64(1706918956000) {
		t.Errorf("event_ts = %v, want 1706918956000", mat["event_ts"])
	}
}

func TestBuildTriggerRequestBody_Validation(t *testing.T) {
	tests := []struct {
		name        string
		triggerType string
		monitorID   int64
		eventID     string
		eventTS     int64
		wantErr     string
	}{
		{
			name:        "monitor_alert missing monitor-id",
			triggerType: "monitor_alert",
			monitorID:   0,
			eventID:     "evt-123",
			eventTS:     1706918956000,
			wantErr:     "--monitor-id is required",
		},
		{
			name:        "monitor_alert missing event-id",
			triggerType: "monitor_alert",
			monitorID:   123,
			eventID:     "",
			eventTS:     1706918956000,
			wantErr:     "--event-id is required",
		},
		{
			name:        "monitor_alert missing event-ts",
			triggerType: "monitor_alert",
			monitorID:   123,
			eventID:     "evt-123",
			eventTS:     0,
			wantErr:     "--event-ts is required",
		},
		{
			name:        "invalid type",
			triggerType: "invalid",
			wantErr:     "must be monitor_alert",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			origType := invTriggerType
			origMonitorID := invMonitorID
			origEventID := invEventID
			origEventTS := invEventTS
			defer func() {
				invTriggerType = origType
				invMonitorID = origMonitorID
				invEventID = origEventID
				invEventTS = origEventTS
			}()

			invTriggerType = tt.triggerType
			invMonitorID = tt.monitorID
			invEventID = tt.eventID
			invEventTS = tt.eventTS

			_, err := buildTriggerRequestBody()
			if err == nil {
				t.Fatal("expected error but got nil")
			}

			if !strings.Contains(err.Error(), tt.wantErr) {
				t.Errorf("error = %q, want to contain %q", err.Error(), tt.wantErr)
			}
		})
	}
}

func TestBuildTriggerRequestBody_JSONRoundtrip(t *testing.T) {
	origType := invTriggerType
	origMonitorID := invMonitorID
	origEventID := invEventID
	origEventTS := invEventTS
	defer func() {
		invTriggerType = origType
		invMonitorID = origMonitorID
		invEventID = origEventID
		invEventTS = origEventTS
	}()

	invTriggerType = "monitor_alert"
	invMonitorID = 999
	invEventID = "evt-round"
	invEventTS = 1234567890000

	body, err := buildTriggerRequestBody()
	if err != nil {
		t.Fatalf("buildTriggerRequestBody() error = %v", err)
	}

	jsonBytes, err := json.Marshal(body)
	if err != nil {
		t.Fatalf("json.Marshal() error = %v", err)
	}

	var parsed map[string]any
	if err := json.Unmarshal(jsonBytes, &parsed); err != nil {
		t.Fatalf("json.Unmarshal() error = %v", err)
	}

	// Verify the JSON roundtrip preserves structure
	data := parsed["data"].(map[string]any)
	if data["type"] != "trigger_investigation_request" {
		t.Errorf("after roundtrip: data.type = %v", data["type"])
	}
}

func TestReadRawResponse_Success(t *testing.T) {
	body := `{"data":{"id":"inv-123","type":"investigation"}}`
	resp := &http.Response{
		StatusCode: 200,
		Body:       io.NopCloser(strings.NewReader(body)),
	}

	result, err := readRawResponse(resp)
	if err != nil {
		t.Fatalf("readRawResponse() error = %v", err)
	}

	data, ok := result["data"].(map[string]any)
	if !ok {
		t.Fatal("result[data] is not a map")
	}

	if data["id"] != "inv-123" {
		t.Errorf("id = %v, want inv-123", data["id"])
	}
}

func TestReadRawResponse_ErrorCodes(t *testing.T) {
	tests := []struct {
		name       string
		statusCode int
		body       string
		wantErr    string
	}{
		{
			name:       "401 unauthorized",
			statusCode: 401,
			body:       "Unauthorized",
			wantErr:    "authentication failed",
		},
		{
			name:       "403 forbidden",
			statusCode: 403,
			body:       "Forbidden",
			wantErr:    "access denied",
		},
		{
			name:       "404 not found",
			statusCode: 404,
			body:       "Not Found",
			wantErr:    "not found",
		},
		{
			name:       "429 rate limited",
			statusCode: 429,
			body:       "Rate Limited",
			wantErr:    "rate limited",
		},
		{
			name:       "500 server error",
			statusCode: 500,
			body:       "Internal Server Error",
			wantErr:    "server error",
		},
		{
			name:       "400 bad request",
			statusCode: 400,
			body:       "Bad Request",
			wantErr:    "request failed",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			resp := &http.Response{
				StatusCode: tt.statusCode,
				Body:       io.NopCloser(strings.NewReader(tt.body)),
			}

			_, err := readRawResponse(resp)
			if err == nil {
				t.Fatal("expected error but got nil")
			}

			if !strings.Contains(err.Error(), tt.wantErr) {
				t.Errorf("error = %q, want to contain %q", err.Error(), tt.wantErr)
			}
		})
	}
}

func TestReadRawResponse_InvalidJSON(t *testing.T) {
	resp := &http.Response{
		StatusCode: 200,
		Body:       io.NopCloser(strings.NewReader("not json")),
	}

	_, err := readRawResponse(resp)
	if err == nil {
		t.Fatal("expected error for invalid JSON")
	}

	if !strings.Contains(err.Error(), "parsing response JSON") {
		t.Errorf("error = %q, want to contain 'parsing response JSON'", err.Error())
	}
}

func TestRunInvestigationsTrigger(t *testing.T) {
	cleanup := setupTestClient(t)
	defer cleanup()

	tests := []struct {
		name    string
		wantErr bool
	}{
		{
			name:    "requires valid client",
			wantErr: true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			var buf bytes.Buffer
			outputWriter = &buf
			defer func() { outputWriter = os.Stdout }()

			// Set required flags
			origType := invTriggerType
			invTriggerType = "monitor_alert"
			origMonitorID := invMonitorID
			invMonitorID = 123
			origEventID := invEventID
			invEventID = "evt-123"
			origEventTS := invEventTS
			invEventTS = 1706918956000
			defer func() {
				invTriggerType = origType
				invMonitorID = origMonitorID
				invEventID = origEventID
				invEventTS = origEventTS
			}()

			err := runInvestigationsTrigger(investigationsTriggerCmd, []string{})
			if (err != nil) != tt.wantErr {
				t.Errorf("runInvestigationsTrigger() error = %v, wantErr %v", err, tt.wantErr)
			}
		})
	}
}

func TestRunInvestigationsGet(t *testing.T) {
	cleanup := setupTestClient(t)
	defer cleanup()

	tests := []struct {
		name    string
		args    []string
		wantErr bool
	}{
		{
			name:    "with valid ID",
			args:    []string{"inv-123"},
			wantErr: true, // Will fail without real API
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			var buf bytes.Buffer
			outputWriter = &buf
			defer func() { outputWriter = os.Stdout }()

			err := runInvestigationsGet(investigationsGetCmd, tt.args)
			if (err != nil) != tt.wantErr {
				t.Errorf("runInvestigationsGet() error = %v, wantErr %v", err, tt.wantErr)
			}
		})
	}
}

func TestRunInvestigationsList(t *testing.T) {
	cleanup := setupTestClient(t)
	defer cleanup()

	tests := []struct {
		name    string
		wantErr bool
	}{
		{
			name:    "requires valid client",
			wantErr: true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			var buf bytes.Buffer
			outputWriter = &buf
			defer func() { outputWriter = os.Stdout }()

			err := runInvestigationsList(investigationsListCmd, []string{})
			if (err != nil) != tt.wantErr {
				t.Errorf("runInvestigationsList() error = %v, wantErr %v", err, tt.wantErr)
			}
		})
	}
}
