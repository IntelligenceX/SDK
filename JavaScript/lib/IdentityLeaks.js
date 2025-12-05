const IntelXClient = require('./IntelXClient');

/**
 * @param {string} apiKey - API key value for X-Key header or k= query param
 * @param {Object} options
 * @param {string} [options.baseUrl] - Optional base URL, defaults to leaks API (3.intelx.io)
 */
class IdentityLeaks extends IntelXClient {
    constructor(apiKey, baseUrl = 'https://3.intelx.io', userAgent = 'IX-JavaScript/0.1') {
        super(apiKey,
            baseUrl,
            userAgent
        );
    }


    async searchInternal(selector, { limit = 100, bucket, skipinvalid = true, analyze = false, datefrom = null, dateto = null, terminate = null}) {
        const searchId = await this.searchInternalId(selector,
            {
            limit,
            bucket,
            skipinvalid,
            analyze,
            datefrom,
            dateto,
            terminate
        })

        this.handleSearchId(searchId)

        let data = ''
        while(true) {
            const r = await this.searchInternalResult(searchId)
            data += r.text
            if (r.status >= 2) {
                break
            }
        }

        this.terminateLiveSearch(searchId)

        return data
    }

    async searchInternalId(selector, { limit = 100, bucket, skipinvalid = true, analyze = false, datefrom = null, dateto = null, terminate = null}) {
        const params = {
            selector: selector,
            limit: limit,
            bucket: bucket,
            skipinvalid: skipinvalid,
            analyze: analyze,
            k: this.apiKey,
        }
        if (datefrom) {
            params['datefrom'] = datefrom
        }
        if (dateto) {
            params['dateto'] = dateto
        }
        if (terminate) {
            params['terminate'] = terminate
        }

        const r = await this._get('/live/search/internal', {params: params})

        if (r.status === 200) {
            const data = await r.json();
            if (data.status === 1 || data.status === 2) {
                return data.status;
            }
            return data.id;
        } else {
            return r.status;
        }
    }

    async searchInternalResult(id, { limit = 100, format = 0})
    {
        if (this.apiRateLimit) {
            await new Promise(resolve => setTimeout(resolve, this.apiRateLimit));
        }
        const params = {
            id: id,
            limit: limit,
            format: format,
        }
        const r = await this._get('/live/search/result', {params: params})
        return await r.json()
    }

    async reverseDomain(term, {maxresults = 10, format = 0, datefrom = null, dateto = null, terminate = null})
    {
        const searchId = await this.reverseDomainSearchId(term, {maxresults, datefrom, dateto, terminate})

        this.handleSearchId(searchId)

        let remaining = maxresults;
        // Poll until done
        const results = []
        while (true) {
            await new Promise(resolve => setTimeout(resolve, this.apiRateLimit));

            // Fetch next chunk of results
            const r = await this.searchInternalResult(searchId, {limit: remaining, format});

            const records = r.records || [];
            for (const rec of records) {
                results.push(rec);
            }

            remaining -= records.length;

            // status 1 or 2 or no more results allowed
            if (r.status === 1 || r.status === 2 || remaining <= 0) {
                if (remaining <= 0) {
                    await this.terminateLiveSearch(searchId);
                }
                break
            }
        }
        return results
    }

    async reverseDomainSearchId(term, {maxresults = 10, datefrom = null, dateto = null, terminate = null}) {
        const params = {
            selector: term,
            limit: maxresults,
            datefrom: datefrom,  // "YYYY-MM-DD HH:MM:SS",
            dateto: dateto,  // "YYYY-MM-DD HH:MM:SS"
            terminate: terminate,
        }

        const r = await this._get('/reverse/domain', {params: params})

        if (r.status === 200) {
            const data = await r.json();
            if (data.status === 1 || data.status === 2) {
                return data.status;
            }
            return data.id;
        } else {
            return r.status;
        }
    }

    /**
     * Terminate a previously initialized search based on its UUID.
     *
     * @param {string} uuid - Search ID (UUID).
     * @returns {Promise<boolean|number>} - true on success, or HTTP status code on error.
     */
    async terminateLiveSearch(uuid) {
        await new Promise(resolve => setTimeout(resolve, this.apiRateLimit));

        const params = { id: uuid };

        const r = await this._get('/live/search/terminate', {
            params,
        });

        if (r.status === 200) {
            return true;
        } else {
            return r.status;
        }
    }
}

module.exports = IdentityLeaks;
