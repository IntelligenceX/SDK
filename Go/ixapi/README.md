# Intelligence X API Go client

The Go package `ixapi` uses the Intelligence X API to perform searches and return results.

There is a full working command line program `ix` that uses this package.

## Using the package

To download the package:

```
go get -u github.com/IntelligenceX/SDK/Go/ixapi
```

Then import it in your code:

```go
import "github.com/IntelligenceX/SDK/Go/ixapi"
```

The code has a default public API key and URL embedded. If you received your own API key, make sure to specify it in the `Init` function.

Following code performs a search and queries the results. Selector is the search term.

```go
search := ixapi.IntelligenceXAPI{}
search.Init("", "")
results, selectorInvalid, err := search.Search(ctx, Selector, ixapi.SortXScoreDesc, 100, ixapi.DefaultWaitSortTime, ixapi.DefaultTimeoutGetResults)
```

These are all functions available of the `IntelligenceXAPI` struct:

```
Init                    Initializes the API with optional API URL and Key
SearchStart             Starts a search and returns the search ID
SearchStartAdvanced     Starts a search with optional parameters and returns the search ID
SearchGetResults        Returns available results
SearchTerminate         Terminates a search
FilePreview             Returns the preview (max first 1000 characters) of an item
FileRead                Returns the full item data
SearchGetResultsAll     Returns all results within a timeout
SetAPIKey               Sets API URL and Key to use
```

These are high-level functions that search and return the results immediately:

```
Search                  Starts a search and queries all results
SearchWithDates         Starts a search with dates and queries all results
```

