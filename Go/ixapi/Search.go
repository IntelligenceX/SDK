/*
File Name:  Search.go
Copyright:  2018 Kleissner Investments s.r.o.
Author:     Peter Kleissner

Simple high-level search code.
*/

package ixapi

import (
	"context"
	"time"

	"github.com/go-dedup/simhash"
)

// DefaultWaitSortTime is the suggested time to give the API to process and sort all the results, before the client queries them.
const DefaultWaitSortTime = 400 * time.Millisecond

// DefaultTimeoutGetResults is the suggested timeout after which the search will be terminated.
const DefaultTimeoutGetResults = 20 * time.Second

// Search starts a search and queries all results. It takes a selector as input.
// WaitSort should be a few hundred ms, giving the API time to sort the results before querying them.
// Limit is the max count of results to query per bucket. The total number of results returned might be higher.
// TimeoutGetResults is the max amount of time for querying all results. This should be at least a few seconds but a timeout of 10-30 seconds makes sense.
// If the input selector is invalid (not a strong selector), the function returns selectorInvalid set to true with no error reported.
func (api *IntelligenceXAPI) Search(ctx context.Context, Selector string, Sort, Limit int, WaitSort, TimeoutGetResults time.Duration) (records []SearchResult, selectorInvalid bool, err error) {

	// make the search
	searchID, selectorInvalid, err := api.SearchStartAdvanced(ctx, IntelligentSearchRequest{Term: Selector, Sort: Sort, MaxResults: Limit})
	if err != nil {
		return nil, false, err
	}

	// give some time for sorting
	time.Sleep(WaitSort)

	records, err = api.SearchGetResultsAll(ctx, searchID, Limit, TimeoutGetResults)
	if err != nil {
		return nil, false, err
	}

	return records, selectorInvalid, nil
}

// SearchWithDates starts a search and queries all results. It takes a selector and dates as input. Sorting is newest first.
// WaitSort should be a few hundred ms, giving the API time to sort the results before querying them.
// Limit is the max count of results to query per bucket. The total number of results returned might be higher.
// TimeoutGetResults is the max amount of time for querying all results. This should be at least a few seconds but a timeout of 10-30 seconds makes sense.
func (api *IntelligenceXAPI) SearchWithDates(ctx context.Context, Selector string, DateFrom, DateTo time.Time, Limit int, WaitSort, TimeoutGetResults time.Duration) (records []SearchResult, selectorInvalid bool, err error) {

	// make the search
	searchID, selectorInvalid, err := api.SearchStartAdvanced(ctx, IntelligentSearchRequest{Term: Selector, Sort: SortDateDesc, MaxResults: Limit, DateFrom: DateFrom.Format("2006-01-02 15:04:05"), DateTo: DateTo.Format("2006-01-02 15:04:05")})
	if err != nil {
		return nil, false, err
	}

	// give some time for sorting
	time.Sleep(WaitSort)

	records, err = api.SearchGetResultsAll(ctx, searchID, Limit, TimeoutGetResults)
	if err != nil {
		return nil, false, err
	}

	return records, selectorInvalid, nil
}

// GetTag gets a tags value for the first occurrence. Empty if not found.
func (item *Item) GetTag(Class int16) (Value string) {
	if item.Tags == nil {
		return ""
	}

	for _, tag := range item.Tags {
		if tag.Class == Class {
			return tag.Value
		}
	}

	return ""
}

// TagLanguage is ISO 639-1 defined
const TagLanguage = 0

// SimhashCompareItems compares 2 items for data equalness and returns the hamming distance. The closer to 0 the more equal they are.
// Never compare Simhashes directly because with different content types and even on the same type with different encoding they use different algorithms.
func SimhashCompareItems(Item1, Item2 *Item) uint8 {
	// Check if simhashes are both valid, otherwise no comparison possible. Also content types must match.
	if Item1.Simhash == 0 || Item2.Simhash == 0 || Item1.Type != Item2.Type {
		return 64
	}

	// only can make simhashes from text for now. In the future maybe others.
	switch Item1.Type {
	case 1: // Text
		// Text: Languages must match. Otherwise the simhash comparison is meaningless and leads to false positives.
		// Especially for CJK (Chinese/Japanese/Korean), as it uses a different simhash algorithm.
		if Item1.GetTag(TagLanguage) != Item2.GetTag(TagLanguage) {
			return 64
		}

		// Enforce a minimum length, otherwise the simhash is pretty much meaningless.
		if Item1.Size < 20 || Item2.Size < 20 {
			return 64
		}

		// return the hamming distance
		return simhash.Compare(Item1.Simhash, Item2.Simhash)
	}

	return 64
}
