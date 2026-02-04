// Unless explicitly stated otherwise all files in this repository are licensed
// under the Apache License Version 2.0.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2024-present Datadog, Inc.

package cmd

import (
	"testing"
)

func TestDataGovernanceCmd(t *testing.T) {
	if dataGovernanceCmd == nil {
		t.Fatal("dataGovernanceCmd is nil")
	}

	if dataGovernanceCmd.Use != "data-governance" {
		t.Errorf("Use = %s, want data-governance", dataGovernanceCmd.Use)
	}

	if dataGovernanceCmd.Short == "" {
		t.Error("Short description is empty")
	}

	if dataGovernanceCmd.Long == "" {
		t.Error("Long description is empty")
	}
}

func TestDataGovernanceCmd_Subcommands(t *testing.T) {
	// Check that scanner subcommand exists
	commands := dataGovernanceCmd.Commands()

	foundScanner := false
	for _, cmd := range commands {
		if cmd.Name() == "scanner" {
			foundScanner = true
		}
	}

	if !foundScanner {
		t.Error("Missing scanner subcommand")
	}
}

func TestDataGovernanceScannerRulesCmd(t *testing.T) {
	if dataGovernanceScannerRulesCmd == nil {
		t.Fatal("dataGovernanceScannerRulesCmd is nil")
	}

	if dataGovernanceScannerRulesCmd.Use != "rules" {
		t.Errorf("Use = %s, want rules", dataGovernanceScannerRulesCmd.Use)
	}

	if dataGovernanceScannerRulesCmd.Short == "" {
		t.Error("Short description is empty")
	}

	// Check for list subcommand
	commands := dataGovernanceScannerRulesCmd.Commands()
	foundList := false
	for _, cmd := range commands {
		if cmd.Name() == "list" {
			foundList = true
			if cmd.RunE == nil {
				t.Error("Scanner rules list command RunE is nil")
			}
		}
	}
	if !foundList {
		t.Error("Missing rules list subcommand")
	}
}

func TestDataGovernanceScannerRulesListCmd(t *testing.T) {
	if dataGovernanceScannerRulesListCmd == nil {
		t.Fatal("dataGovernanceScannerRulesListCmd is nil")
	}

	if dataGovernanceScannerRulesListCmd.Use != "list" {
		t.Errorf("Use = %s, want list", dataGovernanceScannerRulesListCmd.Use)
	}

	if dataGovernanceScannerRulesListCmd.Short == "" {
		t.Error("Short description is empty")
	}

	if dataGovernanceScannerRulesListCmd.RunE == nil {
		t.Error("RunE is nil")
	}
}

func TestDataGovernanceCmd_CommandHierarchy(t *testing.T) {
	// Verify scanner is a subcommand of data-governance
	commands := dataGovernanceCmd.Commands()
	foundScanner := false
	for _, cmd := range commands {
		if cmd.Name() == "scanner" {
			foundScanner = true
			if cmd.Parent() != dataGovernanceCmd {
				t.Error("scanner parent is not dataGovernanceCmd")
			}
		}
	}
	if !foundScanner {
		t.Error("scanner subcommand not found in data-governance")
	}

	// Verify rules is a subcommand of scanner
	scannerCommands := dataGovernanceScannerCmd.Commands()
	foundRules := false
	for _, cmd := range scannerCommands {
		if cmd.Name() == "rules" {
			foundRules = true
			if cmd.Parent() != dataGovernanceScannerCmd {
				t.Error("rules parent is not dataGovernanceScannerCmd")
			}
		}
	}
	if !foundRules {
		t.Error("rules subcommand not found in scanner")
	}

	// Verify list is a subcommand of rules
	rulesCommands := dataGovernanceScannerRulesCmd.Commands()
	for _, cmd := range rulesCommands {
		if cmd.Parent() != dataGovernanceScannerRulesCmd {
			t.Errorf("Command %s parent is not dataGovernanceScannerRulesCmd", cmd.Use)
		}
	}
}
