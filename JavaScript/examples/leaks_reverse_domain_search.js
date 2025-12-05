const IdentityLeaks = require('../lib/IdentityLeaks');

(async () => {
    const apiKey = "00000000-0000-0000-0000-000000000000"

    const client = new IdentityLeaks(apiKey)

    try {
        const data = await client.reverseDomain('riseup.net', { maxresults: 10, format: 1 })
        console.table(data)
    } catch (err) {
        console.error('\nFatal error while running ix.js:');
        console.error(err);
        process.exit(1);
    }
})();
