/*
File Name:  API.go
Copyright:  2018 Kleissner Investments s.r.o.
Author:     Peter Kleissner
Version:    1 from 11/19/2018

API client code for using the Intelligence X API. Create an IntelligenceXAPI object and call Init first.

*/

package ixapi

import (
	"context"
	"crypto/tls"
	"errors"
	"io"
	"io/ioutil"
	"net"
	"net/http"
	"net/url"
	"strconv"
	"strings"
	"time"

	uuid "forks/go.uuid"
)

const defaultAPIURL = "https://public.intelx.io/"
const publicAPIKey = "9df61df0-84f7-4dc7-b34c-8ccfb8646ace"

// IntelligenceXAPI holds all information for communicating with the Intelligence X API.
// Call Init() first.
type IntelligenceXAPI struct {
	URL string    // The API URL. Always ending with slash.
	Key uuid.UUID // The API key assigned by Intelligence X. Contact the company to receive one.

	// additional input. Set before calling Init
	ProxyURL string // Proxy to use
	BindToIP string // Bind to a specific IPv4 or IPv6

	// below are the HTTP client settings

	// one client for the session
	Client              http.Client
	RetryAttempts       int // in case of underlying transport failure
	UserAgent           string
	HTTPMaxResponseSize int64
}

// IntelligentSearchRequest is the information from the human for the search.
type IntelligentSearchRequest struct {
	Term        string        `json:"term"`       // Search term submitted by the user, e.g. "Document 1.docx" or "email@example.com"
	Buckets     []string      `json:"buckets"`    // Bucket identifiers
	Timeout     time.Duration `json:"timeout"`    // Timeout in seconds. May be limited by API config. 0 means default.
	MaxResults  int           `json:"maxresults"` // Total number of max results per bucket. May be limited by API config. 0 means default.
	DateFrom    string        `json:"datefrom"`   // Date from, both from/to are required if set, format "2006-01-02 15:04"
	DateTo      string        `json:"dateto"`     // Date to, both from/to are required if set, format "2006-01-02 15:04"
	Sort        int           `json:"sort"`       // Sort order: 0 = no sorting, 1 = X-Score ASC, 2 = X-Score DESC, 3 = Date ASC, 4 = Date DESC
	Media       int           `json:"media"`      // Media: 0 = not defined, otherwise MediaX as defined in ixservice
	TerminateID []uuid.UUID   `json:"terminate"`  // Optional: Previous search IDs to terminate (normal search or Phonebook). This is if the user makes a new search from the same tab. Same as first calling /intelligent/search/terminate.
}

// IntelligentSearchResponse is the result to the initial search request
type IntelligentSearchResponse struct {
	ID                  uuid.UUID `json:"id"`                  // id of the search job. This is used to get the results.
	SoftSelectorWarning bool      `json:"softselectorwarning"` // Warning of soft selectors, typically garbage in which results into garbage out
	Status              int       `json:"status"`              // Status of the search: 0 = Success (ID valid), 1 = Invalid Term, 2 = Error Max Concurrent Searches
}

// Tag classifies the items data
type Tag struct {
	Class int16  `json:"class"` // Class of tag
	Value string `json:"value"` // The value
}

// Relationship defines a relation between 2 items.
type Relationship struct {
	Target   uuid.UUID `json:"target"`   // Target item systemid
	Relation int       `json:"relation"` // The relationship, see RelationX
}

// Item represents any items meta-data. It origins from Indexed and is sent as search results.
// All fields except the identifier are optional and may be zero. It is perfectly valid that a service only knows partial information (like a name or storage id) of a given item.
type Item struct {
	SystemID    uuid.UUID `json:"systemid"`    // System identifier uniquely identifying the item
	StorageID   string    `json:"storageid"`   // Storage identifier, empty if not stored/available, otherwise a 64-byte blake2b hash hex-encoded
	InStore     bool      `json:"instore"`     // Whether the data of the item is in store and the storage id is valid. Also used to indicate update when false but storage id is set.
	Size        int64     `json:"size"`        // Size in bytes of the item data
	AccessLevel int       `json:"accesslevel"` // Native access level of the item (0 = Public..)
	Type        int       `json:"type"`        // Low-level content type (0 = Binary..)
	Media       int       `json:"media"`       // High-level media type (User, Paste, Tweet, Forum Post..)
	Added       time.Time `json:"added"`       // When the item was added to the system
	Date        time.Time `json:"date"`        // Full time stamp item when it was discovered or created
	Name        string    `json:"name"`        // Name or title
	Description string    `json:"description"` // Full description, text only
	XScore      int       `json:"xscore"`      // X-Score, ranking its relevancy. 0-100, default 50
	Simhash     uint64    `json:"simhash"`     // Simhash, depending on content type. Use hamming distance to compare equality of items data.
	Bucket      string    `json:"bucket"`      // Bucket

	// Tags are meta-data tags helping in classification of the items data. They reveal for example the language or a topic. Different to key-values they have hard-coded classes that
	// allow anyone to take action on them.
	Tags []Tag `json:"tags"`

	// Relations lists all related items.
	Relations []Relationship `json:"relations"`
}

// PanelSearchResultTag represents a tag in human form.
type PanelSearchResultTag struct {
	Class  int16  `json:"class"`  // Class of tag
	ClassH string `json:"classh"` // Class of tag, human friendly
	Value  string `json:"value"`  // The value
	ValueH string `json:"valueh"` // Value, human friendly
}

// PanelItemFriend is a human translation of a relationship
type PanelItemFriend struct {
	SystemID uuid.UUID              `json:"systemid"` // System identifier uniquely identifying the item
	Date     time.Time              `json:"date"`     // Full time stamp item when it was discovered or created
	Name     string                 `json:"name"`     // Name or title
	Inline   FriendShadowCopyInline `json:"inline"`   // inline shadow copy matching PanelSearchResultData
}

// FriendShadowCopyInline is an item copy inline
type FriendShadowCopyInline struct {
	Item
	AccessLevelH string                 `json:"accesslevelh"` // Human friendly access level info
	MediaH       string                 `json:"mediah"`       // Human friendly media type info
	SimhashH     string                 `json:"simhashh"`     // Human friendly simhash
	TypeH        string                 `json:"typeh"`        // Human friendly content type info
	TagsH        []PanelSearchResultTag `json:"tagsh"`        // Human friendly tags
	BucketH      string                 `json:"bucketh"`      // Human friendly bucket name
}

// SearchResult represents a single result record. The entire record IS the de-facto result. Every field is optional and may be empty.
type SearchResult struct {
	Item
	AccessLevelH string                 `json:"accesslevelh"` // Human friendly access level info
	MediaH       string                 `json:"mediah"`       // Human friendly media type info
	SimhashH     string                 `json:"simhashh"`     // Human friendly simhash
	TypeH        string                 `json:"typeh"`        // Human friendly content type info
	TagsH        []PanelSearchResultTag `json:"tagsh"`        // Human friendly tags
	Friends      []PanelItemFriend      `json:"friends"`      // Human friendly translated relations
	RandomID     uuid.UUID              `json:"randomid"`     // Random ID
	BucketH      string                 `json:"bucketh"`      // Human friendly bucket name
}

// IntelligentSearchResult contains the result items
type IntelligentSearchResult struct {
	Records []SearchResult `json:"records"` // The result records
	Status  int            `json:"status"`  // Status: 0 = Success with results, 1 = No more results available, 2 = Search ID not found, 3 = No results yet available keep trying
}

// Sort orders
const (
	SortNone       = 0 // No sorting
	SortXScoreAsc  = 1 // X-Score ascending = Least relevant first
	SortXScoreDesc = 2 // X-Score descending = Most relevant first
	SortDateAsc    = 3 // Date ascending = Oldest first
	SortDateDesc   = 4 // Date descending = Newest first
)

// Init initializes the IX API. URL and Key may be empty to use defaults.
func (api *IntelligenceXAPI) Init(URL string, Key string) {
	api.SetAPIKey(URL, Key)

	api.RetryAttempts = 1
	api.HTTPMaxResponseSize = 100 * 1024 * 1024 // 100 MB

	// Timeouts
	NetworkDialerTimeout := 10 * time.Second
	NetworkTLSTimeout := 10 * time.Second
	HTTPTimeout := 60 * time.Second
	IdleConnTimeout := 90 * time.Second
	KeepAlive := 30 * time.Second

	// Check if to bind on a specific IP. Warning, IPv4 is not available when binding on IPv6! The reverse is true as well.
	var localAddr *net.TCPAddr
	if api.BindToIP != "" {
		localAddr = &net.TCPAddr{
			IP: net.ParseIP(api.BindToIP),
		}
	}

	// create the HTTP client
	var ProxyURLParsed *url.URL
	if api.ProxyURL != "" {
		ProxyURLParsed, _ = url.Parse(api.ProxyURL)
	}

	transport := &http.Transport{
		Proxy: http.ProxyURL(ProxyURLParsed),
		Dial: (&net.Dialer{
			LocalAddr: localAddr,
			Timeout:   NetworkDialerTimeout,
			KeepAlive: KeepAlive,
		}).Dial,
		TLSClientConfig:     &tls.Config{InsecureSkipVerify: true},
		TLSHandshakeTimeout: NetworkTLSTimeout,
		MaxIdleConns:        0,
		MaxIdleConnsPerHost: 100,
		IdleConnTimeout:     IdleConnTimeout,
		DisableKeepAlives:   false,
	}

	api.Client = http.Client{
		Transport: transport,
		CheckRedirect: func(req *http.Request, via []*http.Request) error {
			// Prevent implicit redirection on client.Do calls so that no requests without appropriate headers are sent
			return http.ErrUseLastResponse
		},
		Timeout: HTTPTimeout,
	}
}

// SetAPIKey sets the API URL and Key. URL and Key may be empty to use defaults.
func (api *IntelligenceXAPI) SetAPIKey(URL string, Key string) {
	if URL == "" {
		URL = defaultAPIURL
	}
	if Key == "" {
		Key = publicAPIKey
	}

	if !strings.HasSuffix(URL, "/") {
		URL += "/"
	}

	api.URL = URL
	api.Key, _ = uuid.FromString(Key)
}

// SearchStart starts a search
func (api *IntelligenceXAPI) SearchStart(ctx context.Context, Term string) (searchID uuid.UUID, selectorInvalid bool, err error) {
	request := IntelligentSearchRequest{Term: Term, Sort: SortXScoreDesc}
	response := IntelligentSearchResponse{}

	if err = api.httpRequestPost(ctx, "intelligent/search", request, &response); err != nil {
		return
	}

	switch response.Status {
	case 1:
		return searchID, false, errors.New("Invalid Term")
	case 2:
		return searchID, false, errors.New("Error Max Concurrent Searches")
	}

	return response.ID, response.SoftSelectorWarning, nil
}

// SearchStartAdvanced starts a search and allows the caller to set any advanced filter
func (api *IntelligenceXAPI) SearchStartAdvanced(ctx context.Context, Input IntelligentSearchRequest) (searchID uuid.UUID, selectorInvalid bool, err error) {
	response := IntelligentSearchResponse{}

	if err = api.httpRequestPost(ctx, "intelligent/search", Input, &response); err != nil {
		return
	}

	switch response.Status {
	case 1:
		return searchID, false, errors.New("Invalid Term")
	case 2:
		return searchID, false, errors.New("Error Max Concurrent Searches")
	}

	return response.ID, response.SoftSelectorWarning, nil
}

// SearchGetResults returns results
// Status: 0 = Success with results (continue), 1 = No more results available (this response might still have results), 2 = Search ID not found, 3 = No results yet available keep trying, 4 = Error
func (api *IntelligenceXAPI) SearchGetResults(ctx context.Context, searchID uuid.UUID, Limit int) (records []SearchResult, status int, err error) {
	request := "?id=" + searchID.String() + "&limit=" + strconv.Itoa(Limit) + "&previewlines=20"
	response := IntelligentSearchResult{}

	if err = api.httpRequestGet(ctx, "intelligent/search/result"+request, &response); err != nil {
		return nil, 4, err
	}

	return response.Records, response.Status, nil
}

// SearchTerminate terminates a search
func (api *IntelligenceXAPI) SearchTerminate(ctx context.Context, searchID uuid.UUID) (err error) {
	request := "?id=" + searchID.String()

	return api.httpRequestGet2(ctx, "intelligent/search/terminate"+request)
}

// FilePreview loads the preview of an item. Previews are always capped at 1000 characters.
func (api *IntelligenceXAPI) FilePreview(ctx context.Context, item *Item) (text string, err error) {
	// Request: GET /file/preview?c=[Content Type]&m=[Media Type]&f=[Target Format]&sid=[Storage Identifier]&b=[Bucket]&e=[0|1]
	request := "?sid=" + item.StorageID + "&f=0&l=20&c=" + strconv.Itoa(item.Type) + "&m=" + strconv.Itoa(item.Media) + "&b=" + item.Bucket + "&k=" + api.Key.String()

	response, err := api.httpRequest(ctx, "file/preview"+request, "GET", nil, "")
	if err != nil {
		return "", err
	}

	defer response.Body.Close()

	if response.StatusCode != http.StatusOK {
		return "", api.apiStatusToError(response.StatusCode)
	}

	responseBytes, err := ioutil.ReadAll(io.LimitReader(response.Body, 1000))

	return string(responseBytes), err
}

// FileRead reads the data of an item.
func (api *IntelligenceXAPI) FileRead(ctx context.Context, item *Item, Limit int64) (data []byte, err error) {
	// Request: GET /file/read?type=0&storageid=[storage identifier]&bucket=[optional bucket]
	request := "?type=0&storageid=" + item.StorageID + "&bucket=" + item.Bucket

	response, err := api.httpRequest(ctx, "file/read"+request, "GET", nil, "")
	if err != nil {
		return nil, err
	}

	defer response.Body.Close()

	if response.StatusCode != http.StatusOK {
		return nil, api.apiStatusToError(response.StatusCode)
	}

	responseBytes, err := ioutil.ReadAll(io.LimitReader(response.Body, Limit))

	return responseBytes, err
}

// SearchGetResultsAll returns all results up to Limit and up to the given Timeout. It will automatically terminate the search before returning.
// Unless the underlying API requests report and error, no error will be returned. Deadline exceeded is treated as no error.
func (api *IntelligenceXAPI) SearchGetResultsAll(ctx context.Context, searchID uuid.UUID, Limit int, Timeout time.Duration) (records []SearchResult, err error) {
	var lastStatus int

	newContext, cancel := context.WithDeadline(ctx, time.Now().Add(Timeout))
	defer cancel()

	for {
		var recordsNew []SearchResult
		currentLimit := Limit - len(records)
		recordsNew, lastStatus, err = api.SearchGetResults(newContext, searchID, currentLimit)

		if err != nil && (strings.Contains(err.Error(), context.Canceled.Error()) || strings.Contains(err.Error(), context.DeadlineExceeded.Error())) {
			lastStatus = 5
			break
		} else if err != nil {
			return records, err
		}

		if len(recordsNew) > 0 {
			records = append(records, recordsNew...)
		}

		if len(records) >= Limit {
			break
		}

		// Status: 0 = Success with results (continue), 1 = No more results available (this response might still have results), 2 = Search ID not found, 3 = No results yet available keep trying, 4 = Error
		if lastStatus != 0 && lastStatus != 3 {
			break
		}

		// wait 250 ms before querying the results again
		time.Sleep(time.Millisecond * 250)
	}

	// Terminate the search if required. When Status: 0 = Success with results (continue), 3 = No results yet available keep trying, 4 = Error, 5 = Deadline exceeded
	if lastStatus == 0 || lastStatus == 3 || lastStatus == 4 || lastStatus == 5 {
		api.SearchTerminate(context.Background(), searchID)
	}

	if lastStatus != 4 {
		err = nil
	}

	return records, err
}
