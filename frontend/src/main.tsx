import { Option, Select, ThemeProvider } from '@material-tailwind/react';
import axios from 'axios';
import { getEmojiByCurrencyCode } from 'country-currency-emoji-flags';
import React, { Component } from 'react';
import { createRoot } from 'react-dom/client';

import { ApiCurrency } from './types/currency';
import { Quotation } from './types/quotations';
import { ApiSymbol } from './types/symbol';

type AppProps = {};

type AppState = {
    working: boolean;
    availableSymbols: ApiSymbol[];
    fromCurrency: ApiCurrency;
    toCurrency: ApiCurrency;
    baseCurrency: ApiCurrency;
    currencies: ApiCurrency[];
    fromAmount: number;
    toAmount: number;
    graphData: Quotation[];
};

class App extends Component<AppProps, AppState> {
    private _client: any;

    constructor(props: AppProps) {
        super(props);

        this.state = {
            availableSymbols: [],
            working: false,
        } as any;
        this._client = axios.create({
            baseURL: '//currconv.dev.dux/api/v1/',
        });
        this._client.interceptors.request.use(
            (config: any) => {
                this.setState({ working: true });
                return config;
            },
            (error: any) => {
                this.setState({ working: false });
                return Promise.reject(error);
            }
        );
        this._client.interceptors.response.use(
            (response: any) => {
                this.setState({ working: false });
                return response;
            },
            (error: any) => {
                this.setState({ working: false });
                return Promise.reject(error);
            }
        );
    }

    componentDidMount(): void {
        this.fetchSymbols();
    }

    fetchSymbols = () => {
        this._client.get('/symbols').then((response: any) => {
            const symbols = response.data.symbols as ApiSymbol[];
            this.setState({
                availableSymbols: symbols.map(({ id, code, name }) => ({
                    id,
                    code,
                    name,
                })),
            });
        });
    };

    render() {
        return (
            <ThemeProvider>
                <main className="w-full sm:w-[60rem] grid gap-5 place-items-center place-content-center grid-cols-mobile grid-rows-mobile grid-areas-mobile sm:grid-cols-desktop sm:grid-rows-desktop sm:grid-areas-desktop">
                    <div className="grid-in-fromCurrency place-self-stretch">
                        {this.state.availableSymbols?.length > 0 && (
                            <Select
                                onChange={(value: any) => {
                                    const symbol = this.state.availableSymbols.find((symbol) => symbol.code === value);
                                    if (symbol)
                                        this.setState((oldState) => {
                                            return {
                                                fromCurrency: symbol,
                                            } as any;
                                        });
                                }}
                                label="Origem"
                                selected={(element: any) => {
                                    return (
                                        element &&
                                        React.cloneElement(element, {
                                            className: 'flex items-center px-0 gap-2 pointer-events-none',
                                        })
                                    );
                                }}
                            >
                                {this.state.availableSymbols?.map((symbol) => (
                                    <Option key={symbol.id} value={symbol.code} className="flex items-center space-x-2">
                                        <span>{getEmojiByCurrencyCode(symbol.code)}</span>
                                        <span>{symbol.name}</span>
                                    </Option>
                                ))}
                            </Select>
                        )}
                    </div>
                    <div className="grid-in-toCurrency place-self-stretch">
                        {this.state.availableSymbols?.length > 0 && (
                            <Select
                                onChange={(value: any) => {
                                    const symbol = this.state.availableSymbols.find((symbol) => symbol.code === value);
                                    if (symbol)
                                        this.setState((oldState) => {
                                            return {
                                                toCurrency: symbol,
                                            } as any;
                                        });
                                }}
                                label="Destino"
                                selected={(element: any) =>
                                    element &&
                                    React.cloneElement(element, {
                                        className: 'flex items-center px-0 gap-2 pointer-events-none',
                                    })
                                }
                            >
                                {this.state.availableSymbols?.map((symbol) => (
                                    <Option key={symbol.id} value={symbol.code} className="flex items-center space-x-2">
                                        <span>{getEmojiByCurrencyCode(symbol.code)}</span>
                                        <span>{symbol.name}</span>
                                    </Option>
                                ))}
                            </Select>
                        )}
                    </div>
                    <div className="grid-in-convertButton place-self-center">3</div>
                    <div className="grid-in-graph place-self-center">4</div>
                </main>
            </ThemeProvider>
        );
    }
}

const container = document.getElementById('approot');
if (!container) throw new Error('No container found');
const root = createRoot(container);
root.render(
    <React.StrictMode>
        <App />
    </React.StrictMode>
);
