// Unless explicitly stated otherwise all files in this repository are licensed
// under the Apache License Version 2.0.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2024-present Datadog, Inc.

package cmd

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"strings"
	"time"

	"github.com/datadog-labs/pup/pkg/formatter"
	"github.com/olekukonko/tablewriter"
	"github.com/spf13/cobra"
)

const thirdPartyOutagesURL = "https://updog.ai/data/third-party-outages.json"

// httpClient is the HTTP client used for third-party API calls (injectable for testing).
var httpClient = &http.Client{Timeout: 30 * time.Second}

// Third-party outages response types

type thirdPartyOutagesResponse struct {
	Data thirdPartyOutagesData `json:"data"`
}

type thirdPartyOutagesData struct {
	Attributes thirdPartyOutagesAttributes `json:"attributes"`
	ID         string                      `json:"id"`
	Type       string                      `json:"type"`
}

type thirdPartyOutagesAttributes struct {
	ProviderData []thirdPartyProvider `json:"provider_data"`
}

type thirdPartyProvider struct {
	ProviderName         string             `json:"provider_name"`
	ProviderService      string             `json:"provider_service,omitempty"`
	DisplayName          string             `json:"display_name"`
	IntegrationID        string             `json:"integration_id"`
	StatusURL            string             `json:"status_url"`
	MonitoringStartDate  int64              `json:"monitoring_start_date"`
	MonitoredAPIPatterns []string           `json:"monitored_api_patterns"`
	Outages              []thirdPartyOutage `json:"outages"`
}

type thirdPartyOutage struct {
	Start          int64  `json:"start"`
	End            int64  `json:"end"`
	Status         string `json:"status"`
	ImpactedRegion string `json:"impacted_region,omitempty"`
}

var (
	thirdPartySearch     string
	thirdPartyActiveOnly bool
)

var statusPagesThirdPartyCmd = &cobra.Command{
	Use:   "third-party",
	Short: "View third-party service outage signals",
	Long: `View third-party service outage signals from updog.ai.

Shows current and historical outage data for third-party services that may
affect your Datadog integrations, including cloud providers, SaaS platforms,
and other infrastructure dependencies.

EXAMPLES:
  # List all third-party outage signals
  pup status-pages third-party

  # Search by provider or display name
  pup status-pages third-party --search=amazon

  # Show only active outages
  pup status-pages third-party --active

  # Table view with search
  pup status-pages third-party --output=table --search=aws

AUTHENTICATION:
  This command does not require Datadog authentication.
  Data is sourced from https://updog.ai.`,
	RunE: runStatusPagesThirdParty,
}

func init() {
	statusPagesThirdPartyCmd.Flags().StringVar(&thirdPartySearch, "search", "", "Search by provider name or display name (case-insensitive)")
	statusPagesThirdPartyCmd.Flags().BoolVar(&thirdPartyActiveOnly, "active", false, "Show only providers with active (unresolved) outages")
	statusPagesCmd.AddCommand(statusPagesThirdPartyCmd)
}

func fetchThirdPartyOutages() (*thirdPartyOutagesResponse, error) {
	resp, err := httpClient.Get(thirdPartyOutagesURL)
	if err != nil {
		return nil, fmt.Errorf("failed to fetch third-party outages: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		return nil, fmt.Errorf("unexpected status from updog.ai: %d", resp.StatusCode)
	}

	body, err := io.ReadAll(resp.Body)
	if err != nil {
		return nil, fmt.Errorf("failed to read response: %w", err)
	}

	var result thirdPartyOutagesResponse
	if err := json.Unmarshal(body, &result); err != nil {
		return nil, fmt.Errorf("failed to parse response: %w", err)
	}

	return &result, nil
}

func filterProviders(providers []thirdPartyProvider, search string, activeOnly bool) []thirdPartyProvider {
	if search == "" && !activeOnly {
		return providers
	}

	query := strings.ToLower(search)
	var filtered []thirdPartyProvider
	for _, p := range providers {
		if search != "" {
			name := strings.ToLower(p.ProviderName)
			display := strings.ToLower(p.DisplayName)
			if !strings.Contains(name, query) && !strings.Contains(display, query) {
				continue
			}
		}
		if activeOnly {
			hasActive := false
			for _, o := range p.Outages {
				if o.Status != "resolved" {
					hasActive = true
					break
				}
			}
			if !hasActive {
				continue
			}
		}
		filtered = append(filtered, p)
	}
	return filtered
}

// providerStatus returns an ASCII signal and status string for a provider.
func providerStatus(p thirdPartyProvider) (signal string, status string) {
	for _, o := range p.Outages {
		if o.Status != "resolved" {
			return "▼ DOWN", o.Status
		}
	}
	return "▲ UP", "operational"
}

// formatThirdPartyTable renders providers as a custom table with signal indicators.
func formatThirdPartyTable(providers []thirdPartyProvider) string {
	var buf bytes.Buffer
	table := tablewriter.NewWriter(&buf)
	table.Header("PROVIDER", "DISPLAY NAME", "SERVICE", "SIGNAL", "STATUS")

	for _, p := range providers {
		signal, status := providerStatus(p)
		_ = table.Append(p.ProviderName, p.DisplayName, p.ProviderService, signal, status)
	}

	_ = table.Render()
	return buf.String()
}

func runStatusPagesThirdParty(cmd *cobra.Command, args []string) error {
	data, err := fetchThirdPartyOutages()
	if err != nil {
		return err
	}

	providers := filterProviders(data.Data.Attributes.ProviderData, thirdPartySearch, thirdPartyActiveOnly)

	// Custom table rendering for human-readable output
	if formatter.OutputFormat(outputFormat) == formatter.FormatTable {
		if len(providers) == 0 {
			printOutput("No results found\n")
			return nil
		}
		printOutput("%s", formatThirdPartyTable(providers))
		return nil
	}

	return formatAndPrint(providers, nil)
}
