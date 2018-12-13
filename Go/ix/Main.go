/*
File Name:  Main.go
Copyright:  2018 Kleissner Investments s.r.o.
Author:     Peter Kleissner

This is a command-line tool to use the public Intelligence X API. Usage:

ix [options] [selector]

Optional parameters:
    -k=[key]        Key must be a UUID
    -s=[sort]       The sort options are: 0 = No sorting, 2 = Most relevant first, 3 = Oldest first, 4 = Newest first

Examples:
ix test.com
ix -s=4 cia.gov

Selector types supported:
* Email address
* Domain, including wildcards like *.example.com
* URL
* IPv4 and IPv6
* CIDRv4 and CIDRv6
* Phone Number
* Bitcoin address
* MAC address
* IPFS Hash
* UUID
* Simhash
* Credit card number
* IBAN

*/

package main

import (
	"context"
	"flag"
	"fmt"
	"html"
	"strings"

	"github.com/IntelligenceX/SDK/Go/ixapi"
)

const defaultMaxResults = 10 // max results to query and show

const frontendBaseURL = "https://intelx.io/"
const templateRecordPlain = "==============================\n#%d Date: %s  Title: %s\n------------------------------\n%s\n-> See full result at %s\n"
const templateFooterPlain = "\n\nDisclaimer: Intelligence X finds information in public electronic records. It does not validate or vet any of the above information."

const commandLineHelp = "ix [options] [selector]\n\nOptional parameters:\n    -k=[key]        Key must be a UUID\n    -s=[sort]       The sort options are: 0 = No sorting, 2 = Most relevant first, 3 = Oldest first, 4 = Newest first\n\nExamples:\nix test.com\nix -s=4 cia.gov"
const textSupportedSelectors = "Selector types supported:\n* Email address\n* Domain, including wildcards like *.example.com\n* URL\n* IPv4 and IPv6\n* CIDRv4 and CIDRv6\n* Phone Number\n* Bitcoin address\n* MAC address\n* IPFS Hash\n* UUID\n* Simhash\n* Credit card number\n* IBAN"

func main() {

	keyArgument := flag.String("k", "", "API Key")
	sortArgument := flag.Int("s", ixapi.SortXScoreDesc, "Sort")
	flag.Parse()

	if len(flag.Args()) == 0 {
		fmt.Println(commandLineHelp + "\n\n" + textSupportedSelectors)
		return
	}

	selectorArgument := flag.Args()[0]

	search(context.Background(), *keyArgument, selectorArgument, *sortArgument)
}

func search(ctx context.Context, Key, Selector string, Sort int) {

	// If no API URL or key is specified, the default one from the package is used.
	search := ixapi.IntelligenceXAPI{}
	search.Init("", Key)
	results, selectorInvalid, err := search.Search(ctx, Selector, Sort, defaultMaxResults, ixapi.DefaultWaitSortTime, ixapi.DefaultTimeoutGetResults)

	if err != nil {
		fmt.Printf("Error querying results: %s\n", err)
		return
	} else if len(results) == 0 && selectorInvalid {
		fmt.Println("Invalid input selector. Please specify a strong selector. " + textSupportedSelectors)
		return
	}

	text := generateResultText(ctx, &search, results)
	fmt.Println(text)
}

func generateResultText(ctx context.Context, api *ixapi.IntelligenceXAPI, Records []ixapi.SearchResult) (text string) {

	for n, record := range Records {
		previewText, _ := api.FilePreview(ctx, &record.Item)
		resultLink := frontendBaseURL + "?did=" + record.SystemID.String()

		title := record.Name
		if title == "" {
			title = "Untitled Document"
		}

		text += fmt.Sprintf(templateRecordPlain, n, record.Date.UTC().Format("2006-01-02 15:04"), title, previewHTMLToText(previewText), resultLink)

		if n >= defaultMaxResults-1 {
			break
		}
	}

	if len(Records) == 0 {
		text += "No results.\n"
	}

	// footer
	text += templateFooterPlain

	return
}

// previewHTMLToText translates an HTML preview to plaintext
func previewHTMLToText(input string) (output string) {
	input = html.UnescapeString(input)

	lines := strings.Split(input, "\n")
	for _, line := range lines {
		output += "| " + line + "\n"
	}

	return output
}
