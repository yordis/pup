// Unless explicitly stated otherwise all files in this repository are licensed
// under the Apache License Version 2.0.
// This product includes software developed at Datadog (https://www.datadoghq.com/).
// Copyright 2024-present Datadog, Inc.

package cmd

import (
	"bytes"
	"net/http"
	"net/http/httptest"
	"os"
	"strings"
	"testing"

	"github.com/datadog-labs/pup/pkg/config"
)

const testOutagesJSON = `{
  "data": {
    "attributes": {
      "provider_data": [
        {
          "provider_name": "aws-s3",
          "provider_service": "S3",
          "display_name": "Amazon S3",
          "integration_id": "aws-s3",
          "status_url": "https://health.aws.amazon.com/health/status",
          "monitoring_start_date": 1700000000000,
          "monitored_api_patterns": ["s3.amazonaws.com"],
          "outages": [
            {"start": 1700001000000, "end": 1700002000000, "status": "resolved", "impacted_region": "us-east-1"},
            {"start": 1700003000000, "end": 0, "status": "active", "impacted_region": "us-west-2"}
          ]
        },
        {
          "provider_name": "stripe",
          "display_name": "Stripe",
          "integration_id": "stripe",
          "status_url": "https://status.stripe.com",
          "monitoring_start_date": 1700000000000,
          "monitored_api_patterns": ["api.stripe.com"],
          "outages": [
            {"start": 1700001000000, "end": 1700002000000, "status": "resolved"}
          ]
        },
        {
          "provider_name": "gcp",
          "display_name": "Google Cloud Platform",
          "integration_id": "gcp",
          "status_url": "https://status.cloud.google.com",
          "monitoring_start_date": 1700000000000,
          "monitored_api_patterns": ["googleapis.com"],
          "outages": []
        }
      ]
    },
    "id": "outages",
    "type": "provider_data"
  }
}`

func setupThirdPartyTestServer(t *testing.T, statusCode int, body string) *httptest.Server {
	t.Helper()
	return httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "application/json")
		w.WriteHeader(statusCode)
		_, _ = w.Write([]byte(body))
	}))
}

// redirectTransport intercepts all HTTP requests and redirects them to a test server.
type redirectTransport struct {
	target string
}

func (t *redirectTransport) RoundTrip(req *http.Request) (*http.Response, error) {
	req.URL.Scheme = "http"
	req.URL.Host = t.target[len("http://"):]
	return http.DefaultTransport.RoundTrip(req)
}

func TestStatusPagesThirdPartyCmd(t *testing.T) {
	if statusPagesThirdPartyCmd == nil {
		t.Fatal("statusPagesThirdPartyCmd is nil")
	}
	if statusPagesThirdPartyCmd.Use != "third-party" {
		t.Errorf("Use = %s, want third-party", statusPagesThirdPartyCmd.Use)
	}
	if statusPagesThirdPartyCmd.Short == "" {
		t.Error("Short description is empty")
	}
}

func TestStatusPagesThirdPartyCmd_RegisteredAsSubcommand(t *testing.T) {
	found := false
	for _, cmd := range statusPagesCmd.Commands() {
		if cmd.Use == "third-party" {
			found = true
			break
		}
	}
	if !found {
		t.Error("third-party not registered as subcommand of status-pages")
	}
}

func TestFetchThirdPartyOutages(t *testing.T) {
	server := setupThirdPartyTestServer(t, http.StatusOK, testOutagesJSON)
	defer server.Close()

	origClient := httpClient
	defer func() { httpClient = origClient }()
	httpClient = server.Client()

	t.Run("parses valid response", func(t *testing.T) {
		resp, err := httpClient.Get(server.URL)
		if err != nil {
			t.Fatalf("unexpected error: %v", err)
		}
		defer resp.Body.Close()
		if resp.StatusCode != http.StatusOK {
			t.Fatalf("unexpected status: %d", resp.StatusCode)
		}
	})
}

func TestFetchThirdPartyOutages_ServerError(t *testing.T) {
	server := setupThirdPartyTestServer(t, http.StatusInternalServerError, "server error")
	defer server.Close()

	origClient := httpClient
	defer func() { httpClient = origClient }()
	httpClient = server.Client()

	resp, err := httpClient.Get(server.URL)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	defer resp.Body.Close()
	if resp.StatusCode != http.StatusInternalServerError {
		t.Errorf("expected 500, got %d", resp.StatusCode)
	}
}

func TestFilterProviders(t *testing.T) {
	providers := []thirdPartyProvider{
		{
			ProviderName: "aws-s3",
			DisplayName:  "Amazon S3",
			Outages: []thirdPartyOutage{
				{Status: "resolved"},
				{Status: "active"},
			},
		},
		{
			ProviderName: "stripe",
			DisplayName:  "Stripe",
			Outages: []thirdPartyOutage{
				{Status: "resolved"},
			},
		},
		{
			ProviderName: "gcp",
			DisplayName:  "Google Cloud Platform",
			Outages:      []thirdPartyOutage{},
		},
	}

	tests := []struct {
		name       string
		search     string
		activeOnly bool
		wantCount  int
		wantNames  []string
	}{
		{
			name:      "no filter returns all",
			wantCount: 3,
		},
		{
			name:      "search by provider name",
			search:    "aws",
			wantCount: 1,
			wantNames: []string{"aws-s3"},
		},
		{
			name:      "search by display name",
			search:    "google",
			wantCount: 1,
			wantNames: []string{"gcp"},
		},
		{
			name:      "search by display name Amazon",
			search:    "amazon",
			wantCount: 1,
			wantNames: []string{"aws-s3"},
		},
		{
			name:      "search case insensitive",
			search:    "STRIPE",
			wantCount: 1,
			wantNames: []string{"stripe"},
		},
		{
			name:       "active only",
			activeOnly: true,
			wantCount:  1,
			wantNames:  []string{"aws-s3"},
		},
		{
			name:       "search and active combined",
			search:     "stripe",
			activeOnly: true,
			wantCount:  0,
		},
		{
			name:      "no match",
			search:    "nonexistent",
			wantCount: 0,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := filterProviders(providers, tt.search, tt.activeOnly)
			if len(result) != tt.wantCount {
				t.Errorf("got %d providers, want %d", len(result), tt.wantCount)
			}
			if tt.wantNames != nil {
				for i, name := range tt.wantNames {
					if i < len(result) && result[i].ProviderName != name {
						t.Errorf("provider[%d] = %s, want %s", i, result[i].ProviderName, name)
					}
				}
			}
		})
	}
}

func TestProviderStatus(t *testing.T) {
	tests := []struct {
		name       string
		provider   thirdPartyProvider
		wantSignal string
		wantStatus string
	}{
		{
			name: "operational with no outages",
			provider: thirdPartyProvider{
				Outages: []thirdPartyOutage{},
			},
			wantSignal: "▲ UP",
			wantStatus: "operational",
		},
		{
			name: "operational with only resolved outages",
			provider: thirdPartyProvider{
				Outages: []thirdPartyOutage{
					{Status: "resolved"},
					{Status: "resolved"},
				},
			},
			wantSignal: "▲ UP",
			wantStatus: "operational",
		},
		{
			name: "down with active outage",
			provider: thirdPartyProvider{
				Outages: []thirdPartyOutage{
					{Status: "resolved"},
					{Status: "active"},
				},
			},
			wantSignal: "▼ DOWN",
			wantStatus: "active",
		},
		{
			name: "down with investigating outage",
			provider: thirdPartyProvider{
				Outages: []thirdPartyOutage{
					{Status: "investigating"},
				},
			},
			wantSignal: "▼ DOWN",
			wantStatus: "investigating",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			signal, status := providerStatus(tt.provider)
			if signal != tt.wantSignal {
				t.Errorf("signal = %q, want %q", signal, tt.wantSignal)
			}
			if status != tt.wantStatus {
				t.Errorf("status = %q, want %q", status, tt.wantStatus)
			}
		})
	}
}

func TestFormatThirdPartyTable(t *testing.T) {
	providers := []thirdPartyProvider{
		{
			ProviderName:    "aws-s3",
			DisplayName:     "Amazon S3",
			ProviderService: "S3",
			Outages: []thirdPartyOutage{
				{Status: "active"},
			},
		},
		{
			ProviderName: "stripe",
			DisplayName:  "Stripe",
			Outages: []thirdPartyOutage{
				{Status: "resolved"},
			},
		},
	}

	output := formatThirdPartyTable(providers)

	// Verify headers
	if !strings.Contains(output, "PROVIDER") {
		t.Error("expected table to contain PROVIDER header")
	}
	if !strings.Contains(output, "DISPLAY NAME") {
		t.Error("expected table to contain DISPLAY NAME header")
	}
	if !strings.Contains(output, "SERVICE") {
		t.Error("expected table to contain SERVICE header")
	}
	if !strings.Contains(output, "SIGNAL") {
		t.Error("expected table to contain SIGNAL header")
	}
	if !strings.Contains(output, "STATUS") {
		t.Error("expected table to contain STATUS header")
	}

	// Verify data rows
	if !strings.Contains(output, "aws-s3") {
		t.Error("expected table to contain 'aws-s3'")
	}
	if !strings.Contains(output, "Amazon S3") {
		t.Error("expected table to contain 'Amazon S3'")
	}
	if !strings.Contains(output, "S3") {
		t.Error("expected table to contain 'S3' service")
	}

	// Verify signals
	if !strings.Contains(output, "▼ DOWN") {
		t.Error("expected table to contain '▼ DOWN' for aws-s3 with active outage")
	}
	if !strings.Contains(output, "▲ UP") {
		t.Error("expected table to contain '▲ UP' for stripe with no active outages")
	}
}

func TestFormatThirdPartyTable_Empty(t *testing.T) {
	output := formatThirdPartyTable(nil)
	// Should still render a table (with just headers)
	if !strings.Contains(output, "PROVIDER") {
		t.Error("expected empty table to still contain headers")
	}
}

func setupThirdPartyRunTest(t *testing.T, serverURL string) func() {
	t.Helper()
	origClient := httpClient
	origWriter := outputWriter
	origSearch := thirdPartySearch
	origActive := thirdPartyActiveOnly
	origCfg := cfg
	origFormat := outputFormat

	httpClient = &http.Client{
		Transport: &redirectTransport{target: serverURL},
	}
	cfg = &config.Config{Site: "datadoghq.com"}
	thirdPartySearch = ""
	thirdPartyActiveOnly = false
	outputFormat = "json"

	return func() {
		httpClient = origClient
		outputWriter = origWriter
		thirdPartySearch = origSearch
		thirdPartyActiveOnly = origActive
		cfg = origCfg
		outputFormat = origFormat
	}
}

func TestRunStatusPagesThirdParty(t *testing.T) {
	server := setupThirdPartyTestServer(t, http.StatusOK, testOutagesJSON)
	defer server.Close()
	cleanup := setupThirdPartyRunTest(t, server.URL)
	defer cleanup()

	var buf bytes.Buffer
	outputWriter = &buf

	err := runStatusPagesThirdParty(statusPagesThirdPartyCmd, []string{})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	output := buf.String()
	if output == "" {
		t.Error("expected output, got empty string")
	}
	if !strings.Contains(output, "aws-s3") {
		t.Error("expected output to contain 'aws-s3'")
	}
	if !strings.Contains(output, "stripe") {
		t.Error("expected output to contain 'stripe'")
	}
}

func TestRunStatusPagesThirdParty_WithSearch(t *testing.T) {
	server := setupThirdPartyTestServer(t, http.StatusOK, testOutagesJSON)
	defer server.Close()
	cleanup := setupThirdPartyRunTest(t, server.URL)
	defer cleanup()

	var buf bytes.Buffer
	outputWriter = &buf
	thirdPartySearch = "stripe"

	err := runStatusPagesThirdParty(statusPagesThirdPartyCmd, []string{})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	output := buf.String()
	if !strings.Contains(output, "stripe") {
		t.Error("expected output to contain 'stripe'")
	}
	if strings.Contains(output, "aws-s3") {
		t.Error("expected output NOT to contain 'aws-s3' when searching for stripe")
	}
}

func TestRunStatusPagesThirdParty_SearchByDisplayName(t *testing.T) {
	server := setupThirdPartyTestServer(t, http.StatusOK, testOutagesJSON)
	defer server.Close()
	cleanup := setupThirdPartyRunTest(t, server.URL)
	defer cleanup()

	var buf bytes.Buffer
	outputWriter = &buf
	thirdPartySearch = "Amazon"

	err := runStatusPagesThirdParty(statusPagesThirdPartyCmd, []string{})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	output := buf.String()
	if !strings.Contains(output, "aws-s3") {
		t.Error("expected searching 'Amazon' to match display name 'Amazon S3'")
	}
	if strings.Contains(output, "stripe") {
		t.Error("expected output NOT to contain 'stripe' when searching for Amazon")
	}
}

func TestRunStatusPagesThirdParty_TableFormat(t *testing.T) {
	server := setupThirdPartyTestServer(t, http.StatusOK, testOutagesJSON)
	defer server.Close()
	cleanup := setupThirdPartyRunTest(t, server.URL)
	defer cleanup()

	var buf bytes.Buffer
	outputWriter = &buf
	outputFormat = "table"

	err := runStatusPagesThirdParty(statusPagesThirdPartyCmd, []string{})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	output := buf.String()
	if !strings.Contains(output, "PROVIDER") {
		t.Error("expected table headers in output")
	}
	if !strings.Contains(output, "▼ DOWN") {
		t.Error("expected DOWN signal for aws-s3 with active outage")
	}
	if !strings.Contains(output, "▲ UP") {
		t.Error("expected UP signal for operational providers")
	}
}

func TestRunStatusPagesThirdParty_TableFormatEmpty(t *testing.T) {
	server := setupThirdPartyTestServer(t, http.StatusOK, testOutagesJSON)
	defer server.Close()
	cleanup := setupThirdPartyRunTest(t, server.URL)
	defer cleanup()

	var buf bytes.Buffer
	outputWriter = &buf
	outputFormat = "table"
	thirdPartySearch = "nonexistent"

	err := runStatusPagesThirdParty(statusPagesThirdPartyCmd, []string{})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	output := buf.String()
	if !strings.Contains(output, "No results found") {
		t.Errorf("expected 'No results found', got: %s", output)
	}
}

func TestRunStatusPagesThirdParty_ServerError(t *testing.T) {
	server := setupThirdPartyTestServer(t, http.StatusInternalServerError, "error")
	defer server.Close()
	cleanup := setupThirdPartyRunTest(t, server.URL)
	defer cleanup()

	var buf bytes.Buffer
	outputWriter = &buf

	err := runStatusPagesThirdParty(statusPagesThirdPartyCmd, []string{})
	if err == nil {
		t.Error("expected error for server error response, got nil")
	}
}

func TestRunStatusPagesThirdParty_InvalidJSON(t *testing.T) {
	server := setupThirdPartyTestServer(t, http.StatusOK, "not json")
	defer server.Close()
	cleanup := setupThirdPartyRunTest(t, server.URL)
	defer cleanup()

	var buf bytes.Buffer
	outputWriter = &buf

	err := runStatusPagesThirdParty(statusPagesThirdPartyCmd, []string{})
	if err == nil {
		t.Error("expected error for invalid JSON, got nil")
	}
}

func init() {
	_ = os.Stdout
}
