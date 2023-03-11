import { Component } from 'react';
import { createRoot } from 'react-dom/client';

import { ApiCurrency } from './types/currency';
import { Quotation } from './types/quotations';

type AppProps = {};

type AppState = {
    fromCurrency: ApiCurrency;
    toCurrency: ApiCurrency;
    baseCurrency: ApiCurrency;
    currencies: ApiCurrency[];
    fromAmount: number;
    toAmount: number;
    graphData: Quotation[];
};

class App extends Component<AppProps, AppState> {
    constructor(props: AppProps) {
        super(props);
    }

    render() {
        return (
            <>
                <main className="w-full sm:w-[90vw] grid place-items-center place-content-center grid-cols-mobile grid-rows-mobile grid-areas-mobile sm:grid-cols-desktop sm:grid-rows-desktop sm:grid-areas-desktop">
                    <div className="grid-in-fromCurrency place-self-stretch">fromCurrency</div>
                    <div className="grid-in-toCurrency place-self-stretch">toCurrency</div>
                    <div className="grid-in-convertButton place-self-center">3</div>
                    <div className="grid-in-graph place-self-center">4</div>
                </main>
            </>
        );
    }
}

const container = document.getElementById('approot');
if (!container) throw new Error('No container found');
const root = createRoot(container);
root.render(<App />);
