module.exports = ({ env }) => ({
    meilisearch: {
        config: {
            host: env('MEILI_HOST'),
            apiKey: env('MEILI_MASTER_KEY')
        }
    }
})