/*
File Name:  HTTP Functions.go
Copyright:  2018 Kleissner Investments s.r.o.
Author:     Peter Kleissner

HTTP functions to connect to the API.
*/

package ixapi

import (
	"bytes"
	"context"
	"encoding/json"
	"errors"
	"io"
	"io/ioutil"
	"net/http"
	"strconv"
	"strings"
	"time"
)

// httpRequestPost makes a HTTP POST request and returns JSON data.
func (api *IntelligenceXAPI) httpRequestPost(ctx context.Context, Function string, DataIn interface{}, DataOut interface{}) (err error) {

	// marshal the JSON data
	data, err := json.Marshal(DataIn)
	if err != nil {
		return err
	}

	// make the POST request
	response, err := api.httpRequest(ctx, Function, "POST", data, "application/json")
	if err != nil {
		return err
	}

	defer response.Body.Close()

	if response.StatusCode != http.StatusOK {
		return api.apiStatusToError(response.StatusCode)
	}

	// if limit reader stops the read, keep-alive won't work because there is still data unread. It is intentional that in that case the connection won't be reused.
	return json.NewDecoder(io.LimitReader(response.Body, api.HTTPMaxResponseSize)).Decode(DataOut)
}

// httpRequestPost2 makes a HTTP POST request and returns nothing.
func (api *IntelligenceXAPI) httpRequestPost2(ctx context.Context, Function string, DataIn interface{}) (err error) {

	// marshal the JSON data
	data, err := json.Marshal(DataIn)
	if err != nil {
		return err
	}

	// make the POST request
	response, err := api.httpRequest(ctx, Function, "POST", data, "application/json")
	if err != nil {
		return err
	}

	// the response shall be max 1024 bytes [not needed, only status code will be interpreted]
	//	status, err := ioutil.ReadAll(io.LimitReader(response.Body, 1024))
	io.Copy(ioutil.Discard, response.Body) // required for using keep-alive
	response.Body.Close()

	err = api.apiStatusToError(response.StatusCode)

	return err
}

// httpRequestGet makes a HTTP GET request and returns JSON data.
func (api *IntelligenceXAPI) httpRequestGet(ctx context.Context, Function string, DataOut interface{}) (err error) {

	response, err := api.httpRequest(ctx, Function, "GET", nil, "")
	if err != nil {
		return err
	}

	defer response.Body.Close()

	if response.StatusCode != http.StatusOK {
		return api.apiStatusToError(response.StatusCode)
	}

	// if limit reader stops the read, keep-alive won't work because there is still data unread. It is intentional that in that case the connection won't be reused.
	return json.NewDecoder(io.LimitReader(response.Body, api.HTTPMaxResponseSize)).Decode(DataOut)
}

// httpRequestGet2 makes a HTTP GET request and returns nothing.
func (api *IntelligenceXAPI) httpRequestGet2(ctx context.Context, Function string) (err error) {

	response, err := api.httpRequest(ctx, Function, "GET", nil, "")
	if err != nil {
		return err
	}

	response.Body.Close()

	return api.apiStatusToError(response.StatusCode)
}

// httpRequest makes a HTTP request to the API. If err is nil, response must be closed by the caller.
func (api *IntelligenceXAPI) httpRequest(ctx context.Context, Function, Method string, Data []byte, ContentType string) (response *http.Response, err error) {

	for n := 0; ; n++ {

		var req *http.Request
		var body io.Reader

		if Method == "POST" {
			body = bytes.NewReader(Data)
		}

		req, err = http.NewRequest(Method, api.URL+Function, body)
		if err != nil {
			return nil, err
		}

		req.Header.Set("x-key", api.Key.String())
		req.Header.Set("Connection", "keep-alive")
		req.Header.Set("User-Agent", api.UserAgent)

		if Method == "POST" {
			req.Header.Set("Content-Type", ContentType)
		}

		// make the request
		response, err = api.Client.Do(req.WithContext(ctx))

		// special case: sockets exhausted. Wait for 200ms to give the system time to free up resources. Full error message: "bind: An operation on a socket could not be performed because the system lacked sufficient buffer space or because a queue was full."
		// or error "connectex: Only one usage of each socket address (protocol/network address/port) is normally permitted."
		if err != nil && (strings.Contains(err.Error(), "system lacked sufficient buffer space") || strings.Contains(err.Error(), "Only one usage of each socket address")) {
			time.Sleep(time.Millisecond * 200)
		}

		// normal access mode: return if success, max retry attempts
		if err == nil || n >= api.RetryAttempts {
			return response, err
		}
	}
}

// apiStatusToError translates the HTTP status code returned by services into a Go error
func (api *IntelligenceXAPI) apiStatusToError(StatusCode int) (err error) {

	switch StatusCode {
	case http.StatusOK:
		return nil
	case http.StatusBadRequest:
		return errors.New("Invalid input data")
	case http.StatusUnauthorized:
		return errors.New("Not authorized. Verify the API key")
	case http.StatusNotFound:
		return errors.New("Identifier not found")
	case http.StatusInternalServerError:
		return errors.New("Internal API error")
	case http.StatusNotImplemented:
		return errors.New("Not implemented by API")
	}

	return errors.New("Unknown API error, returned HTTP status " + strconv.Itoa(StatusCode))
}
