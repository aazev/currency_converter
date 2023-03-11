import { ApiCurrency } from './currency';
import { Quotation } from './quotations';

export type ApplicationState = {
    currencies: ApiCurrency[];
    baseCurrency: ApiCurrency;
    fromCurrency: ApiCurrency;
    toCurrency: ApiCurrency;
    fromAmount: number;
    toAmount: number;
    isFetching: boolean;
    Quotations: Quotation;
};
