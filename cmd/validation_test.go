// Unless explicitly stated otherwise all files in this repository are licensed
// under the Apache License Version 2.0.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2024-present Datadog, Inc.

package cmd

import (
	"testing"

	"github.com/spf13/cobra"
)

// TestCommandValidation tests command-level validation that doesn't require API calls

func TestMonitorsDeleteCmd_ArgsValidation(t *testing.T) {
	tests := []struct {
		name    string
		args    []string
		wantErr bool
	}{
		{
			name:    "valid - one arg",
			args:    []string{"12345"},
			wantErr: false,
		},
		{
			name:    "invalid - no args",
			args:    []string{},
			wantErr: true,
		},
		{
			name:    "invalid - too many args",
			args:    []string{"12345", "67890"},
			wantErr: true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			// Test cobra Args validator
			if monitorsDeleteCmd.Args != nil {
				err := monitorsDeleteCmd.Args(monitorsDeleteCmd, tt.args)
				if (err != nil) != tt.wantErr {
					t.Errorf("Args validation error = %v, wantErr %v", err, tt.wantErr)
				}
			}
		})
	}
}

func TestAPIKeysGetCmd_ArgsValidation(t *testing.T) {
	tests := []struct {
		name    string
		args    []string
		wantErr bool
	}{
		{
			name:    "valid - one arg",
			args:    []string{"key-123"},
			wantErr: false,
		},
		{
			name:    "invalid - no args",
			args:    []string{},
			wantErr: true,
		},
		{
			name:    "invalid - too many args",
			args:    []string{"key-123", "key-456"},
			wantErr: true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if apiKeysGetCmd.Args != nil {
				err := apiKeysGetCmd.Args(apiKeysGetCmd, tt.args)
				if (err != nil) != tt.wantErr {
					t.Errorf("Args validation error = %v, wantErr %v", err, tt.wantErr)
				}
			}
		})
	}
}

func TestDashboardsGetCmd_ArgsValidation(t *testing.T) {
	tests := []struct {
		name    string
		args    []string
		wantErr bool
	}{
		{
			name:    "valid - one arg",
			args:    []string{"dashboard-123"},
			wantErr: false,
		},
		{
			name:    "invalid - no args",
			args:    []string{},
			wantErr: true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if dashboardsGetCmd.Args != nil {
				err := dashboardsGetCmd.Args(dashboardsGetCmd, tt.args)
				if (err != nil) != tt.wantErr {
					t.Errorf("Args validation error = %v, wantErr %v", err, tt.wantErr)
				}
			}
		})
	}
}

// Test command structure and relationships

func TestAllCommands_HaveShortDescription(t *testing.T) {
	// Test that all major commands have short descriptions
	commands := []*cobra.Command{
		authCmd,
		metricsCmd,
		monitorsCmd,
		dashboardsCmd,
		logsCmd,
		tracesCmd,
		slosCmd,
		incidentsCmd,
		rumCmd,
		syntheticsCmd,
		usersCmd,
		tagsCmd,
		usageCmd,
		apiKeysCmd,
	}

	for _, cmd := range commands {
		t.Run(cmd.Use, func(t *testing.T) {
			if cmd.Short == "" {
				t.Errorf("Command %s is missing Short description", cmd.Use)
			}
		})
	}
}

func TestAllCommands_HaveUse(t *testing.T) {
	commands := []*cobra.Command{
		authCmd,
		metricsCmd,
		monitorsCmd,
		dashboardsCmd,
		logsCmd,
		slosCmd,
		incidentsCmd,
		usersCmd,
		tagsCmd,
		usageCmd,
		apiKeysCmd,
	}

	for _, cmd := range commands {
		t.Run(cmd.Use, func(t *testing.T) {
			if cmd.Use == "" {
				t.Error("Command is missing Use field")
			}
		})
	}
}

// Test global flags

func TestRootCmd_GlobalFlags(t *testing.T) {
	// Test that global flags are properly configured
	flags := rootCmd.PersistentFlags()

	tests := []struct {
		name     string
		flagName string
	}{
		{"output flag", "output"},
		{"yes flag", "yes"},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			flag := flags.Lookup(tt.flagName)
			if flag == nil {
				t.Errorf("Global flag %s not found", tt.flagName)
			}
		})
	}
}

func TestRootCmd_Version(t *testing.T) {
	// Test that root command has version set
	if rootCmd.Version == "" {
		t.Error("rootCmd.Version is empty")
	}
}
