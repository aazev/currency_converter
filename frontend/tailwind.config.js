const withMT = require('@material-tailwind/react/utils/withMT');
module.exports = withMT({
    content: ['./index.html', './src/**/*.tsx', './src/**/*.ts'],
    theme: {
        extend: {
            gridTemplateAreas: {
                mobile: ['fromCurrency', 'toCurrency', 'convertButton', 'graph'],
                desktop: ['fromCurrency toCurrency', 'convertButton convertButton', 'graph graph'],
            },
            gridTemplateColumns: {
                mobile: '1fr',
                desktop: '1fr 1fr',
            },
            gridTemplateRows: {
                mobile: 'max-content max-content max-content max-content',
                desktop: 'max-content max-content max-content',
            },
        },
    },
    plugins: [require('@savvywombat/tailwindcss-grid-areas')],
});
