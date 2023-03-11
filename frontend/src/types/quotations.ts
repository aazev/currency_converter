import dayjs from 'dayjs';
import timezone from 'dayjs/plugin/timezone';
import utc from 'dayjs/plugin/utc';

dayjs.extend(utc);
dayjs.extend(timezone);

interface Serializable<T> {
    deserialize(input: Object): T;
}

export class Quotation implements Serializable<Quotation> {
    id!: number;
    symbol_id!: number;
    base_symbol_id!: number;
    date!: dayjs.Dayjs;
    open!: number;
    close!: number;
    created_at!: dayjs.Dayjs;
    updated_at!: dayjs.Dayjs | null;

    deserialize(input: any) {
        this.id = input.id;
        this.symbol_id = input.symbol_id;
        this.base_symbol_id = input.base_symbol_id;
        this.date = dayjs(input.date).utc();
        this.open = parseFloat(input.open);
        this.close = parseFloat(input.close);
        this.created_at = dayjs(input.created_at).utc();
        this.updated_at = input.updated_at ? dayjs(input.created_at).utc() : null;
        return this;
    }
}
